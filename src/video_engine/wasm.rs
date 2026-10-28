use crate::{Video, error::Error};

/// The processer of the video resources
///
/// Generally, you shouldn't create your own video engine.
/// This is managed by `Game`
pub struct VideoEngine {
    
}

impl VideoEngine {
    /// Creates a new video engine
    pub fn new() -> Result<Self, Error> {
        todo!();
    }

    /// Returns with a handle for this engine.
    pub fn handle(&self) -> VideoEngineHandle {
        todo!();
    }
}

/// A handle pointing towards a video engine, generally the one created by `Game`
#[derive(Clone)]
pub struct VideoEngineHandle {
    
}

impl VideoEngineHandle {
    pub(crate) fn sync_video(&self, _: &Video) {
        todo!();
    }
}
