use crate::{error::Error, resource::Resource};
use std::{
    any::Any,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering::Relaxed},
    },
};

#[derive(Debug, Clone)]
pub(crate) struct RawFrame {
    pub bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub(crate) struct VideoPlayback {
    pub play_requested: AtomicBool,
    pub stop_requested: AtomicBool,
    pub stopped: AtomicBool,
    pub looping: AtomicBool,
    pub current_frame: Mutex<Option<RawFrame>>,
}

impl Default for VideoPlayback {
    fn default() -> Self {
        Self {
            play_requested: AtomicBool::new(false),
            stop_requested: AtomicBool::new(false),
            stopped: AtomicBool::new(false),
            looping: AtomicBool::new(false),
            current_frame: Mutex::new(None),
        }
    }
}

/// An encoded video resource (.mp4, .mkv, .webm).
///
/// # Example
/// ```rust,ignore
/// use vyxen::{load_path, Video};
///
/// let video = load_data::<Video>("path/to/file.mp4").unwrap();
/// video.play();
/// ```
#[derive(Debug, Clone)]
pub struct Video {
    pub(crate) raw_file_bytes: Arc<Vec<u8>>,
    pub(crate) playback: Arc<VideoPlayback>,
}

impl PartialEq for Video {
    fn eq(&self, other: &Self) -> bool {
        self.raw_file_bytes == other.raw_file_bytes
    }
}

impl Resource for Video {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn load(data: &[u8]) -> Result<Self, Error> {
        Ok(Self {
            raw_file_bytes: Arc::new(data.to_vec()),
            playback: Arc::new(VideoPlayback::default()),
        })
    }
}

impl Video {
    /// Creates a new video from already encoded bytes.
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            raw_file_bytes: Arc::new(bytes),
            playback: Arc::new(VideoPlayback::default()),
        }
    }

    /// Starts (or restarts) playback.
    pub fn play(&self) {
        self.playback.stop_requested.store(false, Relaxed);
        self.playback.play_requested.store(true, Relaxed);
    }

    /// Stops playback.
    pub fn stop(&self) {
        self.playback.play_requested.store(false, Relaxed);
        self.playback.stop_requested.store(true, Relaxed);
    }

    /// Sets if the video should loop
    pub fn set_looping(&self, looping: bool) {
        self.playback.looping.store(looping, Relaxed);
    }

    /// Says if the video is currently playing.
    pub fn is_playing(&self) -> bool {
        !self.playback.stopped.load(Relaxed)
    }

    pub(crate) fn current_frame(&self) -> Option<RawFrame> {
        self.playback.current_frame.lock().unwrap().clone()
    }
}
