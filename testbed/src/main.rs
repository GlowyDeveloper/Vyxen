use vyxen::prelude::*;

fn main() {
    /*let mut game = Game::new();

    let mut scene = Scene::new();

    let mut ground_sprite = Sprite::new();
    ground_sprite.set_shape(Box::new(100.0, 5.0));
    ground_sprite.set_draw_type(DrawType::Color(Color::from_rgb(0.8, 0.3, 0.2)));

    let mut ground_node = Node::new("Ground".to_string());
    ground_node.set_is_static(true);
    ground_node.move_to(Vector2 { x: 0.0, y: 0.0 });
    ground_node.add_component(RigidBody::new(
        1.0,
        true,
        0.5,
        Box::new(100.0, 5.0),
        0.6,
        0.4,
    ));
    ground_node.add_component(Collider::new(Box::new(100.0, 5.0)));
    ground_node.add_component(ground_sprite);
    scene.add_node(ground_node);

    let mut slope_1_node = Node::new("Slope1".to_string());
    slope_1_node.set_is_static(true);
    slope_1_node.move_to(Vector2 { x: -10.0, y: 10.0 });
    slope_1_node.rotate_by(-0.3);
    slope_1_node.add_component(RigidBody::new(
        1.0,
        true,
        0.5,
        Box::new(20.0, 2.0),
        0.6,
        0.4,
    ));
    slope_1_node.add_component(Collider::new(Box::new(20.0, 2.0)));
    let mut slope_1_sprite = Sprite::new();
    slope_1_sprite.set_shape(Box::new(20.0, 2.0));
    slope_1_sprite.set_draw_type(DrawType::Color(Color::from_rgb(0.2, 0.8, 0.3)));
    slope_1_node.add_component(slope_1_sprite);
    scene.add_node(slope_1_node);

    let mut slope_2_node = Node::new("Slope2".to_string());
    slope_2_node.set_is_static(true);
    slope_2_node.move_to(Vector2 { x: 10.0, y: 20.0 });
    slope_2_node.rotate_by(0.3);
    slope_2_node.add_component(RigidBody::new(
        1.0,
        true,
        0.5,
        Box::new(20.0, 2.0),
        0.6,
        0.4,
    ));
    slope_2_node.add_component(Collider::new(Box::new(20.0, 2.0)));
    let mut slope_2_sprite = Sprite::new();
    slope_2_sprite.set_shape(Box::new(20.0, 2.0));
    slope_2_sprite.set_draw_type(DrawType::Texture(
        Texture::from_bytes(include_bytes!("test-img.png"), "image").unwrap(),
    ));
    slope_2_node.add_component(slope_2_sprite);
    scene.add_node(slope_2_node);

    game.load_scene(scene);

    let mut conf = WindowConfig::new();
    conf.set_title("Hello".to_string());
    conf.set_max_size(Vector2 { x: 400.0, y: 400.0 });
    conf.set_min_size(Vector2 { x: 200.0, y: 200.0 });
    conf.set_size(Vector2 { x: 300.0, y: 300.0 });
    conf.set_position(Vector2 {
        x: 1000.0,
        y: 1000.0,
    });
    conf.set_decorations(false);

    game.set_config(conf);

    let _ = game.run_without_callback();*/

    let mut game = Game::new();
    let mut scene = Scene::new();

    let mut sprite = Sprite::new();
    sprite.set_shape(Box::new(20.0, 2.0));
    sprite.set_draw_type(DrawType::Color(Color::from_rgb(0.2, 0.8, 0.3)));

    let mut node = Node::new("Foo".to_string());
    node.add_component(sprite);
    node.set_is_static(true);
    scene.add_node(node);

    game.load_scene(scene);

    let mut config = WindowConfig::new();
    config.set_title("Hello".to_string());
    config.set_max_size(Vector2 { x: 400.0, y: 400.0 });
    config.set_min_size(Vector2 { x: 200.0, y: 200.0 });
    config.set_size(Vector2 { x: 300.0, y: 300.0 });
    config.set_background_color(LIGHT_BLUE);

    game.set_config(config);

    let _ = game.run_without_callback();
}
