use vyxen::prelude::*;

fn main() {
    let mut game = Game::new();
    let mut scene = Scene::new();

    let mut sprite = Sprite::new();
    sprite.set_shape(Box::new(200.0, 20.0));
    sprite.set_draw_type(DrawType::Color(Color::from_rgb(0.2, 0.8, 0.3)));

    let mut node = Node::new("Foo".to_string());
    node.add_component(sprite);
    node.set_is_static(true);
    node.set_physics_process(move |node, _, dt, ctx| {
        let speed = 20.0 * dt;
        if ctx.is_held(KeyCode::KeyW) {
            node.move_by(Vector2 {
                x: 0.0,
                y: speed * dt,
            });
        }
        if ctx.is_held(KeyCode::KeyA) {
            node.move_by(Vector2 {
                x: -speed * dt,
                y: 0.0,
            });
        }
        if ctx.is_held(KeyCode::KeyS) {
            node.move_by(Vector2 {
                x: 0.0,
                y: -speed * dt,
            });
        }
        if ctx.is_held(KeyCode::KeyD) {
            node.move_by(Vector2 {
                x: speed * dt,
                y: 0.0,
            });
        }
    });
    scene.add_node(node);

    game.load_scene(scene);

    game.run_without_callback().unwrap();
}
