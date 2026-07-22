use vyxen::prelude::*;

fn main() {
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

    println!("{:?}", game.get_mouse_position());
    game.run(|_, _, e| match e {
        Event::MouseInput(i, _, _) => {
            println!("{:?}", i);
        },
        Event::MouseWheel(v, t) => {
            println!("{:?} {:?}", v, t);
        }
        _ => {}
    })
    .unwrap();
}
