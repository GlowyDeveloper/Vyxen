use vyxen::prelude::*;

fn main() {
    let speed = 200.0;

    let mut game = Game::new();
    let mut scene = Scene::new();

    let mut sprite1 = Sprite::with_texture(load_path("testbed/src/test-img.png").unwrap());
    sprite1.set_shape(Box::new(200.0, 200.0));

    let mut node1 = Node::new("Foo".to_string());
    node1.add_component(sprite1);
    node1.set_is_static(true);
    scene.add_node(node1);

    let mut sprite2 = Sprite::with_texture(load_data(include_bytes!("test-img.png")).unwrap());
    sprite2.set_shape(Box::new(100.0, 100.0));

    let mut node2 = Node::new("Foo2".to_string());
    node2.add_component(sprite2);
    node2.set_is_static(true);
    node2.set_physics_process(move |node, _, dt, ctx| {
        let speed = speed * dt;
        if ctx.is_held(KeyCode::ArrowUp) {
            node.move_by(Vector2 { x: 0.0, y: speed });
        }
        if ctx.is_held(KeyCode::ArrowLeft) {
            node.move_by(Vector2 { x: -speed, y: 0.0 });
        }
        if ctx.is_held(KeyCode::ArrowDown) {
            node.move_by(Vector2 { x: 0.0, y: -speed });
        }
        if ctx.is_held(KeyCode::ArrowRight) {
            node.move_by(Vector2 { x: speed, y: 0.0 });
        }
    });
    scene.add_node(node2);

    let mut ui3 =
        UiElement::with_image(load_data(include_bytes!("../../docs/Example-2.png")).unwrap());
    ui3.set_shape(Box::new(100.0, 100.0));

    let mut node3 = Node::new("Bar".to_string());
    node3.add_component(ui3);
    node3.move_to(Vector2 { x: 100.0, y: 100.0 });
    node3.set_is_static(true);
    scene.add_node(node3);

    let ui4 = UiElement::with_text(
        "Hello World!".to_string(),
        load_data(include_bytes!("Roboto-Bold.ttf")).unwrap(),
        64.0,
    );
    let mut node4 = Node::new("Bar2".to_string());
    node4.add_component(ui4);
    node4.move_to(Vector2 { x: 200.0, y: 200.0 });
    node4.set_is_static(true);
    scene.add_node(node4);

    game.load_scene(scene);

    let _ = game.run(move |game, _, dt| {
        let speed = speed * dt;
        if game.is_held(KeyCode::KeyW) {
            let cam_pos = game.get_camera().unwrap().get_position();
            game.get_camera_mut()
                .unwrap()
                .set_position(cam_pos + Vector2 { x: 0.0, y: speed });
        }
        if game.is_held(KeyCode::KeyA) {
            let cam_pos = game.get_camera().unwrap().get_position();
            game.get_camera_mut()
                .unwrap()
                .set_position(cam_pos + Vector2 { x: -speed, y: 0.0 });
        }
        if game.is_held(KeyCode::KeyS) {
            let cam_pos = game.get_camera().unwrap().get_position();
            game.get_camera_mut()
                .unwrap()
                .set_position(cam_pos + Vector2 { x: 0.0, y: -speed });
        }
        if game.is_held(KeyCode::KeyD) {
            let cam_pos = game.get_camera().unwrap().get_position();
            game.get_camera_mut()
                .unwrap()
                .set_position(cam_pos + Vector2 { x: speed, y: 0.0 });
        }

        if game.is_held(KeyCode::KeyL) {
            for i in 0..10_000_u64 {
                for j in 0..10_000_u64 {
                    std::hint::black_box(i.wrapping_mul(j));
                }
            }
        }

        game.get_scene_mut().unwrap().remove_node_by_id(50).unwrap();

        let ui_fps2 = UiElement::with_text(
            format!("FPS: {}", game.get_fps().unwrap_or_default().round()),
            load_data(include_bytes!("Roboto-Bold.ttf")).unwrap(),
            32.0,
        );
        let mut fps2 = Node::new("FPS".to_string());
        fps2.add_component(ui_fps2);
        fps2.move_to(Vector2 { x: 100.0, y: 560.0 });
        fps2.set_is_static(true);
        fps2.set_id(50);

        game.get_scene_mut().unwrap().add_node(fps2);
    });
}
