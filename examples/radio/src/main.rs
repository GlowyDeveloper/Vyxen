use vyxen::prelude::*;

const RADIO: &[u8] = include_bytes!("resources/radio.png");
const CROTCHET: &[u8] = include_bytes!("resources/crotchet.png");
const QUAVER: &[u8] = include_bytes!("resources/quaver.png");
const MUSIC: &[u8] = include_bytes!("resources/radio.wav");
const ROBOTO: &[u8] = include_bytes!("resources/Roboto-Bold.ttf");

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    #[cfg(target_arch = "wasm32")]
    console_log::init_with_level(log::Level::Debug).unwrap();

    let mut game = Game::new();
    let mut scene = Scene::new();

    let mut radio_sprite = Sprite::with_texture(load_data(RADIO).unwrap());
    radio_sprite.set_z(1.0);
    radio_sprite.set_shape(Box::new(200.0, 328.0));
    let mut radio_node = Node::new("Radio".to_string());
    radio_node.add_component(radio_sprite.clone());
    radio_node.set_is_static(true);
    radio_node.move_to(Vector2 { x: 0.0, y: 0.0 });
    radio_node.set_id(2);

    scene.add_node(radio_node);

    let mut start_stop_text = Text::new(
        "Click/Tap to start/stop".to_string(),
        load_data(ROBOTO).unwrap(),
        48.0,
    );
    start_stop_text.set_tint(BLACK);
    start_stop_text.set_anchor(TextAnchor::Center);
    let mut start_stop_sprite = Sprite::new();
    start_stop_sprite.set_element_type(ElementType::Text(start_stop_text));
    let mut start_stop = Node::new("StartStop".to_string());
    start_stop.add_component(start_stop_sprite);
    start_stop.move_to(Vector2 { x: 0.0, y: -250.0 });
    start_stop.set_id(3);

    scene.add_node(start_stop);

    game.load_scene(scene);

    let mut window = WindowConfig::new();
    window.set_title("Radio".to_string());
    window.set_background_color(WHITE);

    game.set_config(window);

    let mut sound_handle: Option<SoundHandle> = None;
    let audio = load_data(MUSIC).unwrap();

    let mut time_since_note = 0.0;

    let _ = game.run(move |game, _, event, dt| {
        time_since_note += dt;

        let clicked = match event {
            Event::Touch(_, phase) => phase == TouchPhase::Ended,
            Event::MouseInput(_, state, _) => state == KeyState::Released,
            _ => false,
        };

        if clicked {
            if let Some(handle) = sound_handle.take() {
                handle.stop();
                let scene = game.get_scene_mut().unwrap();
                let child_ids: Vec<_> = scene.get_node(3).unwrap().get_children_ids().to_vec();
                for id in child_ids {
                    scene.remove_node_by_id(id).unwrap();
                }
            } else {
                sound_handle = game.play_sound_with(&audio, 1.0, true);
            }
        }

        let scene = game.get_scene_mut().unwrap();

        if time_since_note >= Random::from_time().range_f32(1.0..10.0) {
            time_since_note = 0.0;

            if sound_handle.is_some() {
                let mut note = Node::new("Note".to_string());
                let texture = if Random::from_time().range_u32(0..2) == 0 {
                    CROTCHET
                } else {
                    QUAVER
                };
                let mut sprite = Sprite::with_texture(load_data(texture).unwrap());
                sprite.set_shape(Box::new(50.0, 50.0));
                note.add_component(sprite);
                note.move_to(Vector2 {
                    x: Random::from_time().range_f32(-300.0..300.0),
                    y: -300.0,
                });
                note.rotate_to(Random::from_time().range_f32(-1.0..1.0));

                note.set_physics_process(|node, _, dt, _| {
                    node.move_by(Vector2 {
                        x: 0.0,
                        y: dt * 100.0,
                    });
                });

                let id = note.get_id();
                scene.add_node(note);
                scene.get_node_mut(3).unwrap().add_child(id);
            }
        }
    });
}
