use vyxen::prelude::*;

fn main() {
    let mut game = Game::new();
    let mut scene = Scene::new();

    scene.set_gravity(Vector2::zero());

    let mut sprite = Sprite::new();
    sprite.set_shape(Box::new(20.0, 2.0));
    sprite.set_draw_type(DrawType::Color(Color::from_rgb(0.2, 0.8, 0.3)));

    let mut node = Node::new("Foo".to_string());
    node.add_component(sprite);
    node.set_physics_process(move |node, _, _, ctx| {
        if ctx.is_held(KeyCode::KeyW) {
            node.add_force(Vector2 { x: 0.0, y: 0.2 });
        }
        if ctx.is_held(KeyCode::KeyA) {
            node.add_force(Vector2 { x: -0.2, y: 0.0 });
        }
        if ctx.is_held(KeyCode::KeyS) {
            node.add_force(Vector2 { x: 0.0, y: -0.2 });
        }
        if ctx.is_held(KeyCode::KeyD) {
            node.add_force(Vector2 { x: 0.2, y: 0.0 });
        }
    });
    scene.add_node(node);

    game.load_scene(scene);

    let mut config = WindowConfig::new();
    config.set_title("Hello".to_string());
    config.set_max_size(Vector2 { x: 400.0, y: 400.0 });
    config.set_min_size(Vector2 { x: 200.0, y: 200.0 });
    config.set_size(Vector2 { x: 300.0, y: 300.0 });
    config.set_background_color(LIGHT_BLUE);

    game.set_config(config);

    game.run_without_callback().unwrap();
}
