use vyxen::prelude::*;

fn main() {
    let mut game = Game::new();
    let mut scene = Scene::new();

    let mut sprite1 = Sprite::with_texture(load_path("testbed/src/test-img.png").unwrap());
    sprite1.set_shape(Box::new(20.0, 20.0));

    let mut node1 = Node::new("Foo".to_string());
    node1.add_component(sprite1);
    node1.set_is_static(true);
    node1.set_physics_process(move |node, _, _, ctx| {
        if ctx.is_held(KeyCode::KeyW) {
            node.move_by(Vector2 { x: 0.0, y: 0.2 });
        }
        if ctx.is_held(KeyCode::KeyA) {
            node.move_by(Vector2 { x: -0.2, y: 0.0 });
        }
        if ctx.is_held(KeyCode::KeyS) {
            node.move_by(Vector2 { x: 0.0, y: -0.2 });
        }
        if ctx.is_held(KeyCode::KeyD) {
            node.move_by(Vector2 { x: 0.2, y: 0.0 });
        }
    });
    scene.add_node(node1);

    let mut sprite2 = Sprite::with_texture(load_data(include_bytes!("test-img.png")).unwrap());
    sprite2.set_shape(Box::new(10.0, 10.0));

    let mut node2 = Node::new("Foo".to_string());
    node2.add_component(sprite2);
    node2.set_is_static(true);
    node2.set_physics_process(move |node, _, _, ctx| {
        if ctx.is_held(KeyCode::ArrowUp) {
            node.move_by(Vector2 { x: 0.0, y: 0.2 });
        }
        if ctx.is_held(KeyCode::ArrowLeft) {
            node.move_by(Vector2 { x: -0.2, y: 0.0 });
        }
        if ctx.is_held(KeyCode::ArrowDown) {
            node.move_by(Vector2 { x: 0.0, y: -0.2 });
        }
        if ctx.is_held(KeyCode::ArrowRight) {
            node.move_by(Vector2 { x: 0.2, y: 0.0 });
        }
    });
    scene.add_node(node2);

    game.load_scene(scene);

    let _ = game.run(|_, _, event| match event {
        Event::MouseInput(button, state, position) => {
            println!("Button: {:?}", button);
            println!("Position: {:?}", position);
            println!("State: {:?}", state);
        }
        _ => {}
    });
}
