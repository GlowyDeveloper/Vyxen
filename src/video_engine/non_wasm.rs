use crate::{Video, error::Error, resource::video::RawFrame};
use gstreamer::prelude::*;
use std::{
    collections::HashMap, sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering::Relaxed},
    },
};

struct PipelineState {
    pipeline: gstreamer::Pipeline,
    bus: gstreamer::Bus,
    cursor: Arc<AtomicUsize>,
}

impl Drop for PipelineState {
    fn drop(&mut self) {
        let _ = self.pipeline.set_state(gstreamer::State::Null);
    }
}

/// The processer of the video resources
///
/// Generally, you shouldn't create your own video engine.
/// This is managed by `Game`
pub struct VideoEngine {
    pipelines: Arc<Mutex<HashMap<usize, PipelineState>>>,
}

impl VideoEngine {
    /// Creates a new video engine
    pub fn new() -> Result<Self, Error> {
        gstreamer::init().map_err(|e| Error::FailedToCreateAudioDecoder(e.to_string()))?;
        Ok(Self {
            pipelines: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Returns with a handle for this engine.
    pub fn handle(&self) -> VideoEngineHandle {
        VideoEngineHandle {
            pipelines: self.pipelines.clone(),
        }
    }
}

/// A handle pointing towards a video engine, generally the one created by `Game`
#[derive(Clone)]
pub struct VideoEngineHandle {
    pipelines: Arc<Mutex<HashMap<usize, PipelineState>>>,
}

impl VideoEngineHandle {
    pub(crate) fn sync_video(&self, video: &Video) {
        let key = Arc::as_ptr(&video.playback) as usize;
        let playback = video.playback.clone();
        let mut pipelines = self.pipelines.lock().unwrap();

        if playback.stop_requested.swap(false, Relaxed) {
            playback.stopped.store(true, Relaxed);
        }

        if playback.play_requested.swap(false, Relaxed) {
            if let Some(old) = pipelines.remove(&key) {
                let _ = old.pipeline.set_state(gstreamer::State::Null);
            }
            
            playback.stopped.store(false, Relaxed);
            *playback.current_frame.lock().unwrap() = None;

            let pipeline = gstreamer::Pipeline::with_name("video-stream-pipeline");
            let src = gstreamer::ElementFactory::make("appsrc").build().unwrap();
            let decoder = gstreamer::ElementFactory::make("decodebin")
                .build()
                .unwrap();
            let convert = gstreamer::ElementFactory::make("videoconvert")
                .build()
                .unwrap();
            let sink = gstreamer::ElementFactory::make("appsink").build().unwrap();

            pipeline
                .add_many([&src, &decoder, &convert, &sink])
                .unwrap();
            src.link(&decoder).unwrap();
            convert.link(&sink).unwrap();

            let convert_clone = convert.clone();
            decoder.connect_pad_added(move |_, src_pad| {
                let sink_pad = convert_clone.static_pad("sink").unwrap();
                if !sink_pad.is_linked() {
                    src_pad.link(&sink_pad).unwrap();
                }
            });

            let appsink = sink.dynamic_cast::<gstreamer_app::AppSink>().unwrap();
            let caps = gstreamer::Caps::builder("video/x-raw")
                .field("format", "RGBA")
                .build();
            appsink.set_caps(Some(&caps));
            
            let inner_shared = playback.clone();
            appsink.set_callbacks(
                gstreamer_app::AppSinkCallbacks::builder()
                    .new_sample(move |appsink| {
                        if inner_shared.stopped.load(Relaxed) {
                            return Ok(gstreamer::FlowSuccess::Ok);
                        }
                        if let Ok(sample) = appsink.pull_sample() {
                            if let Some(buffer) = sample.buffer() {
                                if let Ok(map) = buffer.map_readable() {
                                    if let Some(caps) = sample.caps() {
                                        if let Ok(video_info) =
                                            gstreamer_video::VideoInfo::from_caps(caps)
                                        {
                                            *inner_shared.current_frame.lock().unwrap() =
                                                Some(RawFrame {
                                                    bytes: map.as_slice().to_vec(),
                                                    width: video_info.width(),
                                                    height: video_info.height(),
                                                });
                                        }
                                    }
                                }
                            }
                        }
                        Ok(gstreamer::FlowSuccess::Ok)
                    })
                    .build(),
            );
            
            let appsrc = src.dynamic_cast::<gstreamer_app::AppSrc>().unwrap();
            appsrc.set_stream_type(gstreamer_app::AppStreamType::RandomAccess);
            appsrc.set_format(gstreamer::Format::Bytes);
            appsrc.set_size(video.raw_file_bytes.len() as i64);

            let cursor = Arc::new(AtomicUsize::new(0));
            let need_bytes = video.raw_file_bytes.clone();
            let need_cursor = cursor.clone();
            let seek_bytes = video.raw_file_bytes.clone();
            let seek_cursor = cursor.clone();

            appsrc.set_callbacks(
                gstreamer_app::AppSrcCallbacks::builder()
                    .need_data(move |appsrc, length| {
                        let pos = need_cursor.load(Relaxed);
                        if pos >= need_bytes.len() {
                            let _ = appsrc.end_of_stream();
                            return;
                        }
                        let want = if length == 0 {
                            64 * 1024
                        } else {
                            length as usize
                        };
                        let end = (pos + want).min(need_bytes.len());
                        let slice = &need_bytes[pos..end];

                        let Ok(mut buffer) = gstreamer::Buffer::with_size(slice.len()) else {
                            return;
                        };
                        
                        {
                            let buffer_mut = buffer.get_mut().unwrap();
                            buffer_mut.set_offset(pos as u64);
                            if let Ok(mut map) = buffer_mut.map_writable() {
                                map.as_mut_slice().copy_from_slice(slice);
                            }
                        }
                        need_cursor.store(end, Relaxed);
                        let _ = appsrc.push_buffer(buffer);
                    })
                    .seek_data(move |_appsrc, offset| {
                        if (offset as usize) <= seek_bytes.len() {
                            seek_cursor.store(offset as usize, Relaxed);
                            true
                        } else {
                            false
                        }
                    })
                    .build(),
            );

            match pipeline.set_state(gstreamer::State::Playing) {
                Ok(_) => {}
                Err(e) => {
                    log::error!("Failed to start video pipeline: {}", e);
                    return;
                }
            }

            let bus = pipeline.bus().unwrap();

            pipelines.insert(
                key,
                PipelineState {
                    pipeline,
                    bus,
                    cursor,
                },
            );
        }

        
        let mut should_remove = false;

        if let Some(state) = pipelines.get(&key) {
            if playback.stopped.load(Relaxed) {
                should_remove = true;
            } else {
                while let Some(msg) = state.bus.pop() {
                    match msg.view() {
                        gstreamer::MessageView::Eos(_) => {
                            if playback.looping.load(Relaxed) {
                                state.cursor.store(0, Relaxed);

                                if state
                                    .pipeline
                                    .seek_simple(
                                        gstreamer::SeekFlags::FLUSH,
                                        gstreamer::ClockTime::ZERO,
                                    )
                                    .is_err()
                                {
                                    log::error!(
                                        "video: failed to seek back to start for looping"
                                    );
                                    playback.stopped.store(true, Relaxed);
                                    should_remove = true;
                                }
                            } else {
                                playback.stopped.store(true, Relaxed);
                                should_remove = true;
                            }
                        }

                        gstreamer::MessageView::Error(err) => {
                            log::error!(
                                "Video pipeline error from {:?}: {} ({:?})",
                                err.src().map(|s| s.path_string()),
                                err.error(),
                                err.debug()
                            );

                            playback.stopped.store(true, Relaxed);
                            should_remove = true;
                        }

                        _ => {}
                    }
                }
            }
        }

        if should_remove {
            if let Some(state) = pipelines.remove(&key) {
                let _ = state.pipeline.set_state(gstreamer::State::Null);
            }
        }
    }
}
