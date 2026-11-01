use rfd::AsyncFileDialog;
use std::sync::{Arc, Mutex};
use vyxen::prelude::*;

const ROBOTO: &[u8; 159900] = include_bytes!("Roboto-Bold.ttf");

#[cfg(not(target_arch = "wasm32"))]
fn spawn_future(fut: impl std::future::Future<Output = ()> + Send + 'static) {
    std::thread::spawn(move || pollster::block_on(fut));
}

#[cfg(target_arch = "wasm32")]
fn spawn_future(fut: impl std::future::Future<Output = ()> + 'static) {
    wasm_bindgen_futures::spawn_local(fut);
}

fn pick_video(video_bytes: &Arc<Mutex<Option<Vec<u8>>>>) {
    let video_bytes = Arc::clone(video_bytes);
    spawn_future(async move {
        let file = AsyncFileDialog::new()
            .add_filter("Video Files", &["mp4"])
            .pick_file()
            .await;
        if let Some(file) = file {
            let bytes = file.read().await;
            *video_bytes.lock().unwrap() = Some(bytes);
        }
    });
}

fn set_control_tint(game: &mut Game, tint: Color) {
    if let Some(ui) = game
        .get_scene_mut()
        .unwrap()
        .get_node_mut(3)
        .unwrap()
        .get_component_mut::<UiElement>()
    {
        if let ElementType::Text(text) = ui.get_element_type_mut() {
            text.set_tint(tint);
        }
    }

    if let Some(ui) = game
        .get_scene_mut()
        .unwrap()
        .get_node_mut(4)
        .unwrap()
        .get_component_mut::<UiElement>()
    {
        if let ElementType::Text(text) = ui.get_element_type_mut() {
            text.set_tint(tint);
        }
    }

    if let Some(ui) = game
        .get_scene_mut()
        .unwrap()
        .get_node_mut(5)
        .unwrap()
        .get_component_mut::<UiElement>()
    {
        if let ElementType::Text(text) = ui.get_element_type_mut() {
            text.set_tint(tint);
        }
    }
}

fn main() {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    panic!("RFD (File dialog library) isn't supported on android and ios");
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    #[cfg(target_arch = "wasm32")]
    console_log::init_with_level(log::Level::Debug).unwrap();

    let mut game = Game::new();
    let mut scene = Scene::new();
    let mut picker_visible = true;
    let video_bytes: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));

    let mut picker = Node::new("picker".to_string());
    picker.add_component(Sprite::with_font(
        "Click to open file picker".to_string(),
        load_data(ROBOTO).unwrap(),
        32.0,
    ));
    picker.set_id(1);
    scene.add_node(picker);

    let mut video_node = Node::new("video".to_string());
    video_node.add_component(Sprite::with_color(TRANSPARENT));
    video_node.set_id(2);
    scene.add_node(video_node);

    let mut stop_text = Node::new("stop_text".to_string());
    stop_text.add_component(UiElement::with_text(
        "Stop".to_string(),
        load_data(ROBOTO).unwrap(),
        32.0,
    ));
    stop_text.set_id(3);
    stop_text.move_to(Vector2 { x: 50.0, y: 30.0 });
    scene.add_node(stop_text);

    let mut restart_text = Node::new("restart_text".to_string());
    restart_text.add_component(UiElement::with_text(
        "Restart".to_string(),
        load_data(ROBOTO).unwrap(),
        32.0,
    ));
    restart_text.set_id(4);
    restart_text.move_to(Vector2 { x: 155.0, y: 30.0 });
    scene.add_node(restart_text);

    let mut exit_text = Node::new("exit_text".to_string());
    exit_text.add_component(UiElement::with_text(
        "Exit".to_string(),
        load_data(include_bytes!("Roboto-Bold.ttf")).unwrap(),
        32.0,
    ));
    exit_text.set_id(5);
    exit_text.move_to(Vector2 { x: 250.0, y: 30.0 });
    scene.add_node(exit_text);

    game.load_scene(scene);

    set_control_tint(&mut game, TRANSPARENT);
    
    let mut window = WindowConfig::new();
    window.set_title("Video Player".to_string());

    game.set_config(window);

    let mut time = 0.0;
    game.run(move |game, _, event, dt| {
        let mut handle_input = None;

        time += dt;

        match event {
            Event::MouseInput(_, state, pos) => {
                time = 0.0;
                if state == KeyState::Released {
                    if picker_visible {
                        pick_video(&video_bytes);
                    } else {
                        handle_input = Some(pos);
                    }
                }
            }
            Event::Touch(pos, phase) => {
                time = 0.0;
                if phase == TouchPhase::Ended {
                    if picker_visible {
                        pick_video(&video_bytes);
                    } else {
                        handle_input = Some(pos);
                    }
                }
            }
            Event::CursorMoved(_) => {
                time = 0.0;
            }
            _ => {}
        }

        if let Some(pos) = handle_input {
            if let Some(camera) = game.get_camera() {
                let h = camera.get_height();
                let final_pos = Vector2 {
                    x: pos.x,
                    y: -(pos.y - h),
                };

                if final_pos.x >= 100.0 && final_pos.x <= 215.0 && final_pos.y <= 60.0 {
                    if let Some(sprite) = game
                        .get_scene_mut()
                        .unwrap()
                        .get_node_mut(2)
                        .unwrap()
                        .get_component_mut::<Sprite>()
                    {
                        if let ElementType::Video(video) = sprite.get_element_type() {
                            video.restart();
                        }
                    }
                } else if final_pos.x >= 215.0 && final_pos.x <= 290.0 && final_pos.y <= 60.0 {
                    set_control_tint(game, TRANSPARENT);
                    picker_visible = true;
                    video_bytes.lock().unwrap().take();
                    if let Some(sprite) = game
                        .get_scene_mut()
                        .unwrap()
                        .get_node_mut(1)
                        .unwrap()
                        .get_component_mut::<Sprite>()
                    {
                        if let ElementType::Text(text) = sprite.get_element_type_mut() {
                            text.set_tint(WHITE);
                        }
                    }
                    if let Some(sprite) = game
                        .get_scene_mut()
                        .unwrap()
                        .get_node_mut(2)
                        .unwrap()
                        .get_component_mut::<Sprite>()
                    {
                        sprite.set_element_type(ElementType::Color(TRANSPARENT));
                    }
                } else {
                    if let Some(sprite) = game
                        .get_scene_mut()
                        .unwrap()
                        .get_node_mut(2)
                        .unwrap()
                        .get_component_mut::<Sprite>()
                    {
                        if let ElementType::Video(video) = sprite.get_element_type() {
                            if video.is_playing() {
                                video.stop();
                            } else {
                                video.play();
                            }
                        }
                    }
                }
            }
        }

        if let Some(bytes) = video_bytes.lock().unwrap().take() {
            picker_visible = false;
            if let Some(sprite) = game
                .get_scene_mut()
                .unwrap()
                .get_node_mut(1)
                .unwrap()
                .get_component_mut::<Sprite>()
            {
                if let ElementType::Text(text) = sprite.get_element_type_mut() {
                    text.set_tint(TRANSPARENT);
                }
            }

            set_control_tint(game, WHITE);

            if let Some(sprite) = game
                .get_scene_mut()
                .unwrap()
                .get_node_mut(2)
                .unwrap()
                .get_component_mut::<Sprite>()
            {
                let video = Video::new(bytes);
                video.play();
                sprite.set_element_type(ElementType::Video(video));
            }
        }

        if let Some(camera) = game.get_camera() {
            let h = camera.get_height();
            let w = camera.get_width();

            let _ = camera;

            if let Some(sprite) = game
                .get_scene_mut()
                .unwrap()
                .get_node_mut(2)
                .unwrap()
                .get_component_mut::<Sprite>()
            {
                if let ElementType::Video(video) = sprite.get_element_type() {
                    if let Some(video_w) = video.get_width()
                        && let Some(video_h) = video.get_height()
                    {
                        let scale = (w / video_w as f32).min(h / video_h as f32);

                        let fit_w = video_w as f32 * scale;
                        let fit_h = video_h as f32 * scale;
                        sprite.set_shape(Box::new(fit_w, fit_h));
                    }
                }
            }
        }

        if !picker_visible {
            if time <= 0.0 {
                set_control_tint(game, Color::from_rgba(1.0, 1.0, 1.0, 1.0));
            } else if time >= 6.0 {
                set_control_tint(game, Color::from_rgba(1.0, 1.0, 1.0, 0.0));
            } else if time >= 5.0 {
                let t = time - 5.0;
                let alpha = (1.0 - t).clamp(0.0, 1.0);
                set_control_tint(game, Color::from_rgba(1.0, 1.0, 1.0, alpha));
            }
        }
    })
    .unwrap();
}
