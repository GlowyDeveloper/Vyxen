use crate::{Video, error::Error, resource::video::RawFrame};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, atomic::Ordering::Relaxed},
    time::Duration,
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
    pub(crate) fn sync_video(&self, video: &Video, max_texture_dim: u32) {
        let key = Arc::as_ptr(&video.playback) as usize;
        let playback = video.playback.clone();
        let mut pipelines = self.pipelines.lock().unwrap();

        if playback.stop_requested.swap(false, Relaxed) {
            if let Some(state) = pipelines.get(&key) {
                let secs = state.video_el.current_time();
                *playback.resume_position.lock().unwrap() = Some(Duration::from_secs_f64(secs));
            }
            playback.stopped.store(true, Relaxed);
        }

        if playback.play_requested.swap(false, Relaxed) {
            pipelines.remove(&key);

            playback.stopped.store(false, Relaxed);
            *playback.current_frame.lock().unwrap() = None;

            let resume_at = playback.resume_position.lock().unwrap().take();

            if let Err(e) = Self::start_pipeline(video, &mut pipelines, key, resume_at) {
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
                video_el.set_volume(f32::from_bits(playback.volume.load(Relaxed)) as f64);

                if video_el.ready_state() >= 2 {
                    let native_w = video_el.video_width();
                    let native_h = video_el.video_height();

                    if native_w > 0 && native_h > 0 {
                        let scale = (max_texture_dim as f64 / native_w as f64)
                            .min(max_texture_dim as f64 / native_h as f64)
                            .min(1.0);
                        let w = ((native_w as f64 * scale).floor() as u32).max(1);
                        let h = ((native_h as f64 * scale).floor() as u32).max(1);

                        if state.canvas.width() != w {
                            state.canvas.set_width(w);
                        }
                        if state.canvas.height() != h {
                            state.canvas.set_height(h);
                        }

                        if state
                            .ctx
                            .draw_image_with_html_video_element_and_dw_and_dh(
                                video_el, 0.0, 0.0, w as f64, h as f64,
                            )
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
                                playback.frame_generation.fetch_add(1, Relaxed);
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
                        *playback.resume_position.lock().unwrap() = None;
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
        resume_at: Option<Duration>,
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
        video_el.set_loop(video.playback.looping.load(Relaxed));
        video_el.set_volume(f32::from_bits(video.playback.volume.load(Relaxed)) as f64);
        video_el.set_cross_origin(None);

        if let Some(pos) = resume_at {
            let seconds = pos.as_secs_f64();
            video_el.set_current_time(seconds);

            let video_el_for_seek = video_el.clone();
            let on_loaded = Closure::<dyn FnMut(Event)>::new(move |_evt: Event| {
                video_el_for_seek.set_current_time(seconds);
            });
            video_el.set_onloadedmetadata(Some(on_loaded.as_ref().unchecked_ref()));
            on_loaded.forget();
        }

        if let Ok(promise) = video_el.play() {
            let video_el_for_fallback = video_el.clone();
            let on_play_rejected = Closure::<dyn FnMut(JsValue)>::new(move |_err: JsValue| {
                log::warn!("Autoplay with audio was blocked; retrying muted");
                video_el_for_fallback.set_muted(true);
                let _ = video_el_for_fallback.play();
            });
            let _ = promise.catch(&on_play_rejected);
            on_play_rejected.forget();
        }

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
