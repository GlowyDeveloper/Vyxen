use crate::{Video, error::Error, resource::video::RawFrame};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, atomic::Ordering::Relaxed},
};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use web_sys::{
    Blob, BlobPropertyBag, Event, HtmlVideoElement, OffscreenCanvas,
    OffscreenCanvasRenderingContext2d, Url,
};

struct PipelineState {
    video_el: HtmlVideoElement,
    canvas: OffscreenCanvas,
    ctx: OffscreenCanvasRenderingContext2d,
    object_url: String,
}

impl Drop for PipelineState {
    fn drop(&mut self) {
        // stop the element so it's not still buffering/decoding in the background
        self.video_el.pause().ok();
        self.video_el.set_src("");
        let _ = Url::revoke_object_url(&self.object_url);
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
            pipelines.remove(&key);

            playback.stopped.store(false, Relaxed);
            *playback.current_frame.lock().unwrap() = None;

            if let Err(e) = Self::start_pipeline(video, &mut pipelines, key) {
                log::error!("Failed to start video pipeline: {:?}", e);
                playback.stopped.store(true, Relaxed);
            }
        }

        let mut should_remove = false;

        if let Some(state) = pipelines.get(&key) {
            if playback.stopped.load(Relaxed) {
                should_remove = true;
            } else {
                let video_el = &state.video_el;

                video_el.set_loop(playback.looping.load(Relaxed));

                if video_el.ready_state() >= 2 {
                    let w = video_el.video_width();
                    let h = video_el.video_height();

                    if w > 0 && h > 0 {
                        if state.canvas.width() != w {
                            state.canvas.set_width(w);
                        }
                        if state.canvas.height() != h {
                            state.canvas.set_height(h);
                        }

                        if state
                            .ctx
                            .draw_image_with_html_video_element(video_el, 0.0, 0.0)
                            .is_ok()
                        {
                            if let Ok(image_data) =
                                state.ctx.get_image_data(0.0, 0.0, w as f64, h as f64)
                            {
                                *playback.current_frame.lock().unwrap() = Some(RawFrame {
                                    bytes: image_data.data().to_vec(),
                                    width: w,
                                    height: h,
                                });
                            }
                        }
                    }
                }

                if video_el.ended() {
                    if playback.looping.load(Relaxed) {
                        video_el.set_current_time(0.0);
                        let _ = video_el.play();
                    } else {
                        playback.stopped.store(true, Relaxed);
                        should_remove = true;
                    }
                }
            }
        }

        if should_remove {
            pipelines.remove(&key);
        }
    }

    fn start_pipeline(
        video: &Video,
        pipelines: &mut HashMap<usize, PipelineState>,
        key: usize,
    ) -> Result<(), JsValue> {
        let bytes = js_sys::Uint8Array::from(video.raw_file_bytes.as_slice());
        let parts = js_sys::Array::new();
        parts.push(&bytes.buffer());

        let props = BlobPropertyBag::new();
        props.set_type("video/mp4");
        let blob = Blob::new_with_buffer_source_sequence_and_options(&parts, &props)?;
        let object_url = Url::create_object_url_with_blob(&blob)?;

        let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
        let document = window
            .document()
            .ok_or_else(|| JsValue::from_str("no document"))?;
        let video_el = document
            .create_element("video")?
            .dyn_into::<HtmlVideoElement>()?;

        let video_el_for_err = video_el.clone();
        let on_error = Closure::<dyn FnMut(Event)>::new(move |_evt: Event| {
            if let Some(err) = video_el_for_err.error() {
                log::error!("video error: code={} message={}", err.code(), err.message());
            }
        });

        video_el.set_onerror(Some(on_error.as_ref().unchecked_ref()));
        on_error.forget();
        video_el.set_src(&object_url);
        video_el.set_muted(true);
        video_el.set_loop(video.playback.looping.load(Relaxed));
        video_el.set_cross_origin(None);

        let _ = video_el.play();

        let canvas = OffscreenCanvas::new(1, 1)?;
        let ctx = canvas
            .get_context("2d")?
            .ok_or_else(|| JsValue::from_str("no 2d context"))?
            .dyn_into::<OffscreenCanvasRenderingContext2d>()?;

        pipelines.insert(
            key,
            PipelineState {
                video_el,
                canvas,
                ctx,
                object_url,
            },
        );

        Ok(())
    }
}
