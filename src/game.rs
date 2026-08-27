use crate::{
    Camera, Scene, Sprite, Vector2, WindowConfig,
    inputs::{Inputs, KeyCode, KeyState, MouseInput, TouchPhase},
    renderer::state::State,
    ui::UiElement,
};
use std::{path::PathBuf, sync::Arc};

use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::PhysicalKey,
    window::WindowId,
};

#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(target_arch = "wasm32")]
use web_time::Instant;

type Callback = Box<dyn FnMut(&mut Game, Event, f32)>;

/// Game struct to hold everything
///
/// # Examples
/// ```rust
/// use vyxen_core::{Scene, Game};
///
/// let scene = Scene::new();
///
/// let mut game = Game::new();
///
/// game.load_scene(scene);
/// ```
pub struct Game {
    loaded_scene: Option<Scene>,
    state: Option<State>,
    callback: Option<Callback>,
    ctx: Context,
    last_redraw: Instant,
    dt: f32,

    #[cfg(target_arch = "wasm32")]
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    /// Game struct to hold everything
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Scene, Game};
    ///
    /// let scene = Scene::new();
    ///
    /// let mut game = Game::new();
    ///
    /// game.load_scene(scene);
    /// ```
    pub fn new() -> Self {
        Self {
            loaded_scene: None,
            state: None,
            callback: None,
            ctx: Context {
                inputs: Inputs::new(),
                cursor_pos: Vector2::zero(),
                config: WindowConfig::new(),
            },
            last_redraw: Instant::now(),
            dt: 0.0,

            #[cfg(target_arch = "wasm32")]
            proxy: None,
        }
    }

    /// Steps the scene.
    ///
    /// Ran automatically in `run()`
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Scene, Game};
    ///
    /// let scene = Scene::new();
    ///
    /// let mut game = Game::new();
    ///
    /// game.load_scene(scene);
    ///
    /// game.step(0.1);
    /// ```
    pub fn step(&mut self, dt: f32) {
        if let Some(scene) = &mut self.loaded_scene {
            scene.step(dt, self.ctx.clone());
        }
    }

    /// Gets the loaded scene.
    ///
    /// If you want the mutable version, refer to `get_scene_mut()`
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Scene, Game};
    ///
    /// let scene = Scene::new();
    ///
    /// let mut game = Game::new();
    ///
    /// game.load_scene(scene);
    ///
    /// let loaded = game.get_scene().unwrap();
    /// ```
    pub fn get_scene(&self) -> Option<&Scene> {
        self.loaded_scene.as_ref()
    }

    /// Gets the loaded scene mutably.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Scene, Game};
    ///
    /// let mut scene = Scene::new();
    ///
    /// let mut game = Game::new();
    ///
    /// game.load_scene(scene);
    ///
    /// let mut loaded = game.get_scene_mut().unwrap();
    /// ```
    pub fn get_scene_mut(&mut self) -> Option<&mut Scene> {
        self.loaded_scene.as_mut()
    }

    /// Loads a new scene.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Scene, Game};
    ///
    /// let scene = Scene::new();
    ///
    /// let mut game = Game::new();
    ///
    /// game.load_scene(scene);
    /// ```
    pub fn load_scene(&mut self, scene: Scene) {
        self.loaded_scene = Some(scene);
    }

    /// Returns the camera of the game.
    ///
    /// If you want the mutable version, refer to `get_camera_mut()`
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Scene, Game};
    ///
    /// let mut game = Game::new();
    ///
    /// let camera = game.get_camera();
    /// assert!(camera.is_none());
    /// ```
    pub fn get_camera(&self) -> Option<&Camera> {
        self.state.as_ref().map(|s| s.get_camera())
    }

    /// Returns the camera of the game mutably.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Scene, Game};
    /// use vyxen_math::Vector2;
    ///
    /// let mut game = Game::new();
    ///
    /// let camera = game.get_camera_mut();
    /// assert!(camera.is_none());
    /// ```
    pub fn get_camera_mut(&mut self) -> Option<&mut Camera> {
        self.state.as_mut().map(|s| s.get_camera_mut())
    }

    /// Sets a new config
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Scene, Game};
    /// use vyxen_renderer::WindowConfig;
    ///
    /// let mut game = Game::new();
    ///
    /// let mut conf = WindowConfig::new();
    /// conf.set_title("Foobar".to_string());
    ///
    /// game.set_config(conf);
    /// ```
    pub fn set_config(&mut self, config: WindowConfig) {
        self.ctx.config = config;
    }

    /// Updates the sprites for the renderer.
    ///
    /// Called automatically by `run()`.
    pub fn update_sprites(&mut self) {
        if let Some(scene) = &mut self.loaded_scene {
            if let Some(state) = self.state.as_mut() {
                let sprites = scene
                    .get_nodes_mut()
                    .iter_mut()
                    .filter_map(|(_, node)| {
                        let pos = node.get_position();
                        let rot = node.get_rotation();

                        if let Some(sprite) = node.get_component_mut::<Sprite>() {
                            sprite.set_position(pos);
                            sprite.set_rotation(rot);
                            Some(sprite.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Sprite>>();

                state.set_sprites(sprites);

                let ui = scene
                    .get_nodes_mut()
                    .iter_mut()
                    .filter_map(|(_, node)| {
                        let pos = node.get_position();
                        let rot = node.get_rotation();

                        if let Some(sprite) = node.get_component_mut::<UiElement>() {
                            sprite.set_position(pos);
                            sprite.set_rotation(rot);
                            Some(sprite.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<UiElement>>();

                state.set_ui_elements(ui);
            }
        }
    }

    /// Runs the game.
    ///
    /// Fields are:
    ///  - `Game` (the current game)
    ///  - `Event` (the event that triggered the callback)
    ///  - `f32` (the delta time since the last frame redraw)
    ///
    /// # Examples
    /// ```rust, no_run
    /// use vyxen_core::{Scene, Game};
    ///
    /// let scene = Scene::new();
    ///
    /// let mut game = Game::new();
    ///
    /// game.load_scene(scene);
    ///
    /// game.run(|_, _, _| {
    ///     println!("callback"); // Called every frame
    /// });
    /// ```
    pub fn run<F>(mut self, callback: F) -> anyhow::Result<()>
    where
        F: FnMut(&mut Game, Event, f32) + 'static,
    {
        let event_loop: EventLoop<State> = EventLoop::with_user_event().build()?;

        #[cfg(target_arch = "wasm32")]
        {
            let proxy = Some(event_loop.create_proxy());
            self.proxy = proxy;
        }

        self.callback = Some(Box::new(callback));

        #[cfg(not(target_arch = "wasm32"))]
        {
            event_loop.run_app(&mut self)?;
        }
        #[cfg(target_arch = "wasm32")]
        {
            event_loop.spawn_app(self);
        }

        Ok(())
    }

    /// Runs the game without a callback.
    ///
    /// # Examples
    /// ```rust, no_run
    /// use vyxen_core::{Scene, Game};
    ///
    /// let scene = Scene::new();
    ///
    /// let mut game = Game::new();
    ///
    /// game.load_scene(scene);
    ///
    /// game.run_without_callback();
    /// ```
    #[allow(unused_mut)]
    pub fn run_without_callback(mut self) -> anyhow::Result<()> {
        let event_loop: EventLoop<State> = EventLoop::with_user_event().build()?;

        #[cfg(target_arch = "wasm32")]
        {
            let proxy = Some(event_loop.create_proxy());
            self.proxy = proxy;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            event_loop.run_app(&mut self)?;
        }
        #[cfg(target_arch = "wasm32")]
        {
            event_loop.spawn_app(self);
        }

        Ok(())
    }

    /// If a key has been pressed between the current frame and the last.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_input::{KeyCode, Inputs};
    /// use vyxen_core::Game;
    ///
    /// let mut game = Game::new();
    ///
    /// assert!(!game.is_just_pressed(KeyCode::KeyH));
    /// ```
    ///
    /// # Note
    ///
    /// For `is_just_pressed` to be processed correctly, the game must be first ran from `run` or `run_without_callback`.
    pub fn is_just_pressed(&self, keycode: KeyCode) -> bool {
        self.ctx.inputs.just_pressed(keycode)
    }

    /// If a key has been released between the current frame and the last.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_input::KeyCode;
    /// use vyxen_core::Game;
    ///
    /// let mut game = Game::new();
    ///
    /// assert!(!game.is_just_released(KeyCode::KeyH));
    /// ```
    ///
    /// # Note
    ///
    /// For `is_just_released` to be processed correctly, the game must be first ran from `run` or `run_without_callback`.
    pub fn is_just_released(&self, keycode: KeyCode) -> bool {
        self.ctx.inputs.just_released(keycode)
    }

    /// If a key is currently held.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_input::KeyCode;
    /// use vyxen_core::Game;
    ///
    /// let mut game = Game::new();
    ///
    /// assert!(!game.is_held(KeyCode::KeyH));
    /// ```
    ///
    /// # Note
    ///
    /// For `is_held` to be processed correctly, the game must be first ran from `run` or `run_without_callback`.
    pub fn is_held(&self, keycode: KeyCode) -> bool {
        self.ctx.inputs.held(keycode)
    }

    /// The current mouse position.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::Game;
    /// use vyxen_math::Vector2;
    ///
    /// let mut game = Game::new();
    ///
    /// assert_eq!(game.get_mouse_position(), Vector2::zero());
    /// ```
    ///
    /// # Note
    ///
    /// For `get_mouse_position` to be processed correctly,
    ///  - the game must be first ran from `run` or `run_without_callback`. If not, `Vector2::zero()` will be returned.
    ///  - the cursor position will only be updated when the cursor is in the window. If not, the most recent reported mouse position will be returned.
    pub fn get_mouse_position(&self) -> Vector2 {
        self.ctx.cursor_pos
    }

    /// Returns the delta time.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::Game;
    ///
    /// let mut game = Game::new();
    ///
    /// assert_eq!(game.get_last_dt(), 0.0); // initial value is 0.0
    /// ```
    pub fn get_last_dt(&self) -> f32 {
        self.dt
    }

    /// Returns the frames per second.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::Game;
    ///
    /// let mut game = Game::new();
    ///
    /// assert_eq!(game.get_fps(), None); // initial value is None
    /// ```
    ///
    /// # Note
    ///
    /// Returns `None` if the state is not initialized.
    pub fn get_fps(&self) -> Option<f32> {
        self.state.as_ref().map(|s| s.get_fps())
    }

    /// Converts screen coordinates to world coordinates
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_core::Game;
    ///
    /// let mut game = Game::new();
    ///
    /// assert_eq!(game.screen_to_world(Vector2 { x: 500.0, y: 200.0 }), None);
    /// ```
    ///
    /// # Note
    ///
    /// Returns `None` if the state is not initialized.
    pub fn screen_to_world(&self, vector2: Vector2) -> Option<Vector2> {
        if let Some(camera) = self.get_camera() {
            let screen_center = Vector2 {
                x: camera.get_width(),
                y: camera.get_height(),
            } / 2.0;

            let res = camera.get_position() + (vector2 - screen_center) / camera.get_zoom();

            Some(Vector2 {
                x: res.x,
                y: -res.y,
            })
        } else {
            None
        }
    }

    /// Converts world coordinates to screen coordinates
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_core::Game;
    ///
    /// let mut game = Game::new();
    ///
    /// assert_eq!(game.world_to_screen(Vector2 { x: 500.0, y: 200.0 }), None);
    /// ```
    ///
    /// # Note
    ///
    /// Returns `None` if the state is not initialized.
    pub fn world_to_screen(&self, vector2: Vector2) -> Option<Vector2> {
        if let Some(camera) = self.get_camera() {
            let screen_center = Vector2 {
                x: camera.get_width(),
                y: camera.get_height(),
            } / 2.0;

            let res = (vector2 - screen_center) * camera.get_zoom() + camera.get_position();

            Some(Vector2 {
                x: res.x,
                y: -res.y,
            })
        } else {
            None
        }
    }
}

impl ApplicationHandler<State> for Game {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(self.ctx.config.clone().into())
                .unwrap(),
        );

        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut state =
                pollster::block_on(State::new(window, self.ctx.config.clone())).unwrap();
            state.resize(
                state.get_window().inner_size().width,
                state.get_window().inner_size().height,
            );
            self.state = Some(state);
        }
        #[cfg(target_arch = "wasm32")]
        {
            let config = self.ctx.config.clone();
            let proxy = self.proxy.clone().unwrap();
            wasm_bindgen_futures::spawn_local(async move {
                assert!(
                    proxy
                        .send_event(State::new(window, config.clone()).await.unwrap())
                        .is_ok()
                )
            });
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(mut callback) = self.callback.take() {
            let into: Event = match event {
                WindowEvent::MouseInput { state, button, .. } => {
                    Event::MouseInput(button.into(), state.into(), self.ctx.cursor_pos)
                }
                _ => event.clone().into(),
            };
            if into != Event::Unknown {
                callback(self, into, self.dt);
            }

            self.callback = Some(callback);
        }

        match event {
            WindowEvent::Resized(physical_size) => {
                if physical_size.width == 0 || physical_size.height == 0 {
                    return;
                }

                if let Some(state) = &mut self.state {
                    state.resize(physical_size.width, physical_size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let dt = (now - self.last_redraw).as_secs_f32();
                self.last_redraw = now;
                self.dt = dt;

                self.step(dt);
                self.update_sprites();

                if let Some(state) = &mut self.state {
                    state.update();
                    state.render().unwrap();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    match event.state {
                        ElementState::Pressed => self.ctx.inputs.key_pressed(code.into()),
                        ElementState::Released => self.ctx.inputs.key_released(code.into()),
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.ctx.cursor_pos = Vector2 {
                    x: position.x as f32,
                    y: position.y as f32,
                }
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        self.ctx.inputs.begin_frame();

        if let Some(state) = &self.state {
            state.get_window().request_redraw();
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        event.get_window().request_redraw();
        event.resize(
            event.get_window().inner_size().width,
            event.get_window().inner_size().height,
        );
        self.state = Some(event);
    }
}

/// The context of the game.
#[derive(Clone)]
pub struct Context {
    pub inputs: Inputs,
    pub cursor_pos: Vector2,
    pub config: WindowConfig,
}

impl Context {
    /// If a key is currently held.
    pub fn is_held(&self, keycode: KeyCode) -> bool {
        self.inputs.held(keycode)
    }

    /// If a key has been pressed between the current frame and the last.
    pub fn is_just_pressed(&self, keycode: KeyCode) -> bool {
        self.inputs.just_pressed(keycode)
    }

    /// If a key has been released between the current frame and the last.
    pub fn is_just_released(&self, keycode: KeyCode) -> bool {
        self.inputs.just_released(keycode)
    }
}

/// Window Events.
/// Retuned from a callback in `Game`.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Event {
    /// Change of window size
    Resized(Vector2),
    /// Window movement
    Moved(Vector2),
    /// Close Request
    CloseRequested,
    /// Window closing
    Destroyed,
    /// A file dropped on the window
    DroppedFile(PathBuf),
    /// A file hovered on the window
    HoveredFile(PathBuf),
    /// A file was hovered, and moved from the window
    HoveredFileCancelled,
    /// Window gained focus
    Focused,
    /// Window lost focus
    Unfocused,
    /// Keyboard input
    KeyboardInput(KeyCode, KeyState),
    /// Cursor movement
    CursorMoved(Vector2),
    /// Cursor entered the window
    CursorEntered,
    /// Cursor exited the window
    CursorExited,
    /// Cursor mouse wheel movement
    MouseWheel(Vector2, TouchPhase),
    /// Cursor input
    MouseInput(MouseInput, KeyState, Vector2),
    /// Two-finger pinch gesture
    /// MacOS and iOS only.
    PinchGesture(f64),
    /// Pan gesture
    /// MacOS and iOS only.
    PanGesture(Vector2),
    /// Double tap gesture
    /// MacOS and iOS only.
    DoubleTapGesture,
    /// Two-finger rotation gesture
    /// MacOS and iOS only.
    RotationGesture(f32),
    /// Touchpad pressure, including stage.
    /// MacOS only.
    TouchpadPressure(f32, i64),
    /// Touch input
    Touch(Vector2, TouchPhase),
    /// Window moved across screens with different DPIs.
    ScaleChanged(f64),
    /// Window hidden behind another.
    Occluded,
    /// Window became visible
    Visible,
    /// Window should be redrawn
    RedrawRequested,
    /// Window should be redrawn
    Unknown,
}

impl From<WindowEvent> for Event {
    fn from(value: WindowEvent) -> Self {
        match value {
            WindowEvent::Resized(physical) => Self::Resized(Vector2 {
                x: physical.width as f32,
                y: physical.height as f32,
            }),
            WindowEvent::Moved(physical) => Self::Moved(Vector2 {
                x: physical.x as f32,
                y: physical.y as f32,
            }),
            WindowEvent::CloseRequested => Self::CloseRequested,
            WindowEvent::Destroyed => Self::Destroyed,
            WindowEvent::DroppedFile(path) => Self::DroppedFile(path),
            WindowEvent::HoveredFile(path) => Self::HoveredFile(path),
            WindowEvent::HoveredFileCancelled => Self::HoveredFileCancelled,
            WindowEvent::Focused(true) => Self::Focused,
            WindowEvent::Focused(false) => Self::Unfocused,
            WindowEvent::CursorEntered { .. } => Self::CursorEntered,
            WindowEvent::CursorLeft { .. } => Self::CursorExited,
            WindowEvent::PinchGesture { delta, .. } => Self::PinchGesture(delta),
            WindowEvent::PanGesture { delta, .. } => Self::PanGesture(Vector2 {
                x: delta.x,
                y: delta.y,
            }),
            WindowEvent::DoubleTapGesture { .. } => Self::DoubleTapGesture,
            WindowEvent::RotationGesture { delta, .. } => Self::RotationGesture(delta),
            WindowEvent::TouchpadPressure {
                pressure, stage, ..
            } => Self::TouchpadPressure(pressure, stage),
            WindowEvent::Touch(touch) => Self::Touch(
                Vector2 {
                    x: touch.location.x as f32,
                    y: touch.location.y as f32,
                },
                touch.phase.into(),
            ),
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                Self::ScaleChanged(scale_factor)
            }
            WindowEvent::Occluded(true) => Self::Occluded,
            WindowEvent::Occluded(false) => Self::Visible,
            WindowEvent::RedrawRequested => Self::RedrawRequested,
            WindowEvent::KeyboardInput { event, .. } => match event.physical_key {
                PhysicalKey::Code(code) => Self::KeyboardInput(code.into(), event.state.into()),
                PhysicalKey::Unidentified(_) => Self::Unknown,
            },
            WindowEvent::CursorMoved { position, .. } => Self::CursorMoved(Vector2 {
                x: position.x as f32,
                y: position.y as f32,
            }),
            WindowEvent::MouseWheel { delta, phase, .. } => Self::MouseWheel(
                match delta {
                    MouseScrollDelta::LineDelta(x, y) => Vector2 { x, y },
                    MouseScrollDelta::PixelDelta(pos) => Vector2 {
                        x: pos.x as f32,
                        y: pos.y as f32,
                    },
                },
                phase.into(),
            ),
            _ => Self::Unknown,
        }
    }
}
