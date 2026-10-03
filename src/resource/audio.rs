use crate::{error::Error, resource::Resource};
use std::{any::Any, io::Cursor};
use symphonia::core::{
    codecs::audio::AudioDecoderOptions,
    errors::Error as SymphoniaError,
    formats::{FormatOptions, TrackType, probe::Hint},
    io::MediaSourceStream,
    meta::MetadataOptions,
};

/// Represents an audio resource.
///
/// Contains the audio data, sample rate, and number of channels.
///
/// Supports the following audio formats:
/// - WAV
/// - MP3
/// - OGG
/// - FLAC
/// - ALAC
/// - AAC
/// - PCM
/// - Vorbis
///
/// # Examples
///
/// ```rust,ignore
/// use vyxen::{load_path, Audio};
///
/// let data = load_path("path/to/audio.wav")?;
/// let audio = Audio::new(data)?;
///
/// let sample_rate = audio.get_sample_rate();
/// let channels = audio.get_channels();
/// let decoded_data = audio.get_data();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Audio {
    data: Vec<f32>,
    sample_rate: u32,
    channels: u16,
}

impl Resource for Audio {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn load(data: &[u8]) -> Result<Self, Error> {
        let mss = MediaSourceStream::new(Box::new(Cursor::new(data)), Default::default());

        let mut format = symphonia::default::get_probe()
            .probe(
                &Hint::new(),
                mss,
                FormatOptions::default(),
                MetadataOptions::default(),
            )
            .map_err(|e| Error::FailedAudioProbe(e.to_string()))?;

        let track = format
            .default_track(TrackType::Audio)
            .ok_or(Error::NoAudioTrack)?;
        let track_id = track.id;

        let params = track
            .codec_params
            .as_ref()
            .and_then(|p| p.audio())
            .ok_or(Error::NoAudioCodec)?;

        let mut decoder = symphonia::default::get_codecs()
            .make_audio_decoder(params, &AudioDecoderOptions::default())
            .map_err(|e| Error::FailedToCreateAudioDecoder(e.to_string()))?;

        let mut samples: Vec<f32> = Vec::new();
        let mut scratch: Vec<f32> = Vec::new();
        let mut spec: Option<(u32, u16)> = None;

        while let Some(packet) = format
            .next_packet()
            .map_err(|e| Error::FailedAudioProbe(e.to_string()))?
        {
            if packet.track_id != track_id {
                continue;
            }

            match decoder.decode(&packet) {
                Ok(buf) => {
                    let s = buf.spec();
                    spec.get_or_insert((s.rate(), s.channels().count() as u16));

                    scratch.resize(buf.samples_interleaved(), 0.0);
                    buf.copy_to_slice_interleaved(&mut scratch);
                    samples.extend_from_slice(&scratch);
                }
                Err(SymphoniaError::DecodeError(_) | SymphoniaError::IoError(_)) => continue,
                Err(e) => return Err(Error::FailedToDecodeAudio(e.to_string())),
            }
        }

        let (sample_rate, channels) = spec.ok_or(Error::NoAudioDecoded)?;

        Ok(Audio::new(samples, sample_rate, channels))
    }
}

impl Audio {
    /// Creates a new `Audio` instance with the given data, sample rate, and channels.
    pub fn new(data: Vec<f32>, sample_rate: u32, channels: u16) -> Self {
        Self {
            data,
            sample_rate,
            channels,
        }
    }

    /// Returns a reference to the audio data.
    pub fn get_data(&self) -> &[f32] {
        &self.data
    }

    /// Returns the sample rate of the audio.
    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Returns the number of channels of the audio.
    pub fn get_channels(&self) -> u16 {
        self.channels
    }
}
