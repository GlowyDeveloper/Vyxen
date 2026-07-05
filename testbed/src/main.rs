//fn main() {
//    let _ = vyxen::renderer::run();
//}

use vyxen::{Collider, Game, Node, Scene, geometry::Box, math::Vector2, physics2d::RigidBody, renderer::{Color, DrawType, Sprite, Texture}};

fn main() {
    let mut game = Game::new().unwrap();

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
    slope_2_sprite.set_draw_type(DrawType::Texture(Texture::from_bytes(include_bytes!("test-img.png"), "image").unwrap()));
    slope_2_node.add_component(slope_2_sprite);
    scene.add_node(slope_2_node);

    game.load_scene(scene);

    game.run().unwrap();
}
