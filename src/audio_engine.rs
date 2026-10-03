use std::{
    any::Any,
    sync::{
        Arc, Mutex, MutexGuard,
        atomic::{AtomicBool, AtomicU32, Ordering::Relaxed},
    },
};

use cpal::{
    Device, FromSample, SampleFormat, SizedSample, Stream, StreamConfig, default_host,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

use crate::{Audio, error::Error, node::Component};

struct VoiceShared {
    volume: AtomicU32,
    finished: AtomicBool,
    stopped: AtomicBool,
}

struct Voice {
    audio: Arc<Audio>,
    frame_pos: f64,
    looping: bool,
    shared: Arc<VoiceShared>,
}

struct Mixer {
    voices: Vec<Voice>,
    master: f32,
}

impl Mixer {
    fn mix_into(&mut self, out: &mut [f32], channels: usize, device_rate: u32) {
        out.fill(0.0);
        let frames = out.len() / channels;

        for voice in &mut self.voices {
            if voice.shared.stopped.load(Relaxed) {
                voice.shared.finished.store(true, Relaxed);
                continue;
            }

            let src = voice.audio.get_data();
            let src_ch = voice.audio.get_channels() as usize;
            let src_frames = src.len().checked_div(src_ch).unwrap_or(0);
            if src_frames == 0 {
                voice.shared.finished.store(true, Relaxed);
                continue;
            }

            let step = voice.audio.get_sample_rate() as f64 / device_rate as f64;
            let vol = f32::from_bits(voice.shared.volume.load(Relaxed)) * self.master;

            for f in 0..frames {
                if voice.frame_pos >= src_frames as f64 {
                    if voice.looping {
                        voice.frame_pos %= src_frames as f64;
                    } else {
                        voice.shared.finished.store(true, Relaxed);
                        break;
                    }
                }

                let i0 = voice.frame_pos as usize;
                let frac = (voice.frame_pos - i0 as f64) as f32;
                let i1 = if i0 + 1 < src_frames {
                    i0 + 1
                } else if voice.looping {
                    0
                } else {
                    i0
                };

                for c in 0..channels {
                    let sc = if src_ch == 1 { 0 } else { c % src_ch };
                    let a = src[i0 * src_ch + sc];
                    let b = src[i1 * src_ch + sc];
                    out[f * channels + c] += (a + (b - a) * frac) * vol;
                }

                voice.frame_pos += step;
            }
        }

        self.voices.retain(|v| !v.shared.finished.load(Relaxed));
    }
}

fn lock(m: &Mutex<Mixer>) -> MutexGuard<'_, Mixer> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

/// Handle to the audio engine mixer.
#[derive(Clone)]
pub struct AudioHandle {
    mixer: Arc<Mutex<Mixer>>,
}

impl AudioHandle {
    /// Starts a voice from the audio.
    pub fn start_sound(&self, audio: &Audio, volume: f32, looping: bool) -> SoundHandle {
        let shared = Arc::new(VoiceShared {
            volume: AtomicU32::new(volume.to_bits()),
            finished: AtomicBool::new(false),
            stopped: AtomicBool::new(false),
        });
        lock(&self.mixer).voices.push(Voice {
            audio: Arc::new(audio.clone()),
            frame_pos: 0.0,
            looping,
            shared: shared.clone(),
        });
        SoundHandle { shared }
    }

    /// Plays the audio with the default volume and no looping.
    pub fn play(&self, audio: &Audio) {
        self.start_sound(audio, 1.0, false);
    }
}

/// Handle to a playing sound.
pub struct SoundHandle {
    shared: Arc<VoiceShared>,
}

impl SoundHandle {
    /// Stops the sound.
    pub fn stop(&self) {
        self.shared.stopped.store(true, Relaxed);
    }

    /// Sets the volume of the sound.
    pub fn set_volume(&self, v: f32) {
        self.shared.volume.store(v.to_bits(), Relaxed);
    }

    /// Returns whether the sound is currently playing.
    pub fn is_playing(&self) -> bool {
        !self.shared.finished.load(Relaxed)
    }
}

/// Audio engine for playing sounds.
pub struct AudioEngine {
    stream: Option<Stream>,
    mixer: Arc<Mutex<Mixer>>,
    device_name: String,
}

impl AudioEngine {
    /// Creates a new audio engine with the default output device.
    pub fn new() -> Result<Self, Error> {
        Self::open(None)
    }

    pub fn resume(&self) -> Result<(), Error> {
        if let Some(s) = &self.stream {
            s.play()
                .map_err(|e| Error::FailedToOpenAudioDevice(e.to_string()))?;
        }
        Ok(())
    }

    /// Returns a list of available output devices.
    pub fn list_output_devices() -> Vec<String> {
        default_host()
            .output_devices()
            .map(|devs| {
                devs.filter_map(|d| d.description().ok().map(|d| d.name().to_string()))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Creates a new audio engine with the specified output device.
    pub fn with_device(name: &str) -> Result<Self, Error> {
        Self::open(Some(name))
    }

    fn open(name: Option<&str>) -> Result<Self, Error> {
        let mixer = Arc::new(Mutex::new(Mixer {
            voices: Vec::new(),
            master: 1.0,
        }));

        let (stream, device_name) = start_stream(name, mixer.clone())?;

        Ok(Self {
            stream: Some(stream),
            mixer,
            device_name,
        })
    }

    /// Sets the output device to the specified device.
    pub fn set_device(&mut self, name: Option<&str>) -> Result<(), Error> {
        self.stream = None;
        let (stream, device_name) = start_stream(name, self.mixer.clone())?;
        self.stream = Some(stream);
        self.device_name = device_name;
        Ok(())
    }

    /// Returns the name of the current output device.
    pub fn get_device_name(&self) -> &str {
        &self.device_name
    }

    /// Sets the master volume of the audio engine.
    pub fn set_master_volume(&self, v: f32) {
        lock(&self.mixer).master = v;
    }

    /// Returns the handle of the audio engine.
    pub fn handle(&self) -> AudioHandle {
        AudioHandle {
            mixer: self.mixer.clone(),
        }
    }

    /// Stops all voices in the audio engine.
    pub fn stop_all(&self) {
        for v in &lock(&self.mixer).voices {
            v.shared.stopped.store(true, Relaxed);
        }
    }
}

fn find_device(name: Option<&str>) -> Result<Device, Error> {
    let host = cpal::default_host();
    match name {
        None => host
            .default_output_device()
            .ok_or_else(|| Error::FailedToOpenAudioDevice("no default output device".into())),
        Some(n) => host
            .output_devices()
            .map_err(|e| Error::FailedToOpenAudioDevice(e.to_string()))?
            .find(|d| d.description().is_ok_and(|d| d.name() == n))
            .ok_or_else(|| Error::FailedToOpenAudioDevice(format!("device not found: {n}"))),
    }
}

fn start_stream(name: Option<&str>, mixer: Arc<Mutex<Mixer>>) -> Result<(Stream, String), Error> {
    let device = find_device(name)?;
    let device_name = device
        .description()
        .map(|d| d.name().to_string())
        .unwrap_or_default();

    let supported = device
        .default_output_config()
        .map_err(|e| Error::FailedToOpenAudioDevice(e.to_string()))?;
    let format = supported.sample_format();
    let config: StreamConfig = supported.into();

    let stream = match format {
        SampleFormat::F32 => build_stream::<f32>(&device, config, mixer),
        SampleFormat::I16 => build_stream::<i16>(&device, config, mixer),
        SampleFormat::U16 => build_stream::<u16>(&device, config, mixer),
        SampleFormat::I32 => build_stream::<i32>(&device, config, mixer),
        other => Err(Error::FailedToOpenAudioDevice(format!(
            "unsupported sample format: {other:?}"
        ))),
    }?;

    stream
        .play()
        .map_err(|e| Error::FailedToOpenAudioDevice(e.to_string()))?;
    Ok((stream, device_name))
}

fn build_stream<T>(
    device: &Device,
    config: StreamConfig,
    mixer: Arc<Mutex<Mixer>>,
) -> Result<Stream, Error>
where
    T: SizedSample + FromSample<f32>,
{
    let channels = config.channels as usize;
    let rate = config.sample_rate;
    let mut scratch: Vec<f32> = Vec::new();

    device
        .build_output_stream(
            config,
            move |data: &mut [T], _| {
                scratch.resize(data.len(), 0.0);
                lock(&mixer).mix_into(&mut scratch, channels, rate);
                for (o, s) in data.iter_mut().zip(&scratch) {
                    *o = T::from_sample(s.clamp(-1.0, 1.0));
                }
            },
            |e| log::error!("audio stream error: {e}"),
            None,
        )
        .map_err(|e| Error::FailedToOpenAudioDevice(e.to_string()))
}

/// Represents an audio source that can be played back through the audio engine.
pub struct AudioSource {
    audio: Arc<Audio>,
    volume: f32,
    looping: bool,
    play_requested: bool,
    stop_requested: bool,
    voice: Option<SoundHandle>,
}

impl AudioSource {
    /// Creates a new audio source from the given audio data.
    pub fn new(audio: Audio) -> Self {
        Self {
            audio: Arc::new(audio),
            volume: 1.0,
            looping: false,
            play_requested: false,
            stop_requested: false,
            voice: None,
        }
    }

    /// Plays the audio source.
    pub fn play(&mut self) {
        self.play_requested = true;
    }

    /// Stops the audio source.
    pub fn stop(&mut self) {
        self.stop_requested = true;
    }

    /// Sets whether the audio source should loop.
    pub fn set_looping(&mut self, l: bool) {
        self.looping = l;
    }

    /// Sets the volume of the audio source.
    pub fn set_volume(&mut self, v: f32) {
        self.volume = v;
        if let Some(voice) = &self.voice {
            voice.set_volume(v);
        }
    }

    /// Returns whether the audio source is playing.
    pub fn is_playing(&self) -> bool {
        self.voice.as_ref().is_some_and(|v| v.is_playing())
    }

    pub(crate) fn sync(&mut self, handle: &AudioHandle) {
        if self.stop_requested {
            if let Some(v) = self.voice.take() {
                v.stop();
            }
            self.stop_requested = false;
        }
        if self.play_requested {
            if let Some(v) = self.voice.take() {
                v.stop();
            }
            self.voice = Some(handle.start_sound(&self.audio, self.volume, self.looping));
            self.play_requested = false;
        }
    }
}

impl Component for AudioSource {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
