use vyxen::prelude::*;

fn main() {
    let mut game = Game::new();

    let mut scene = Scene::new();

    scene.set_gravity(Vector2 { x: 0.0, y: -150.0 });

    let bottom_box_size = Box::new(400.0, 40.0);
    let mut bottom_sprite = Sprite::with_color(DARK_GRAY);
    bottom_sprite.set_shape(bottom_box_size);
    let mut bottom_wall = Node::new("bottom".to_string());
    bottom_wall.add_component(bottom_sprite);
    bottom_wall.add_component(Collider::new(bottom_box_size));
    bottom_wall.add_component(RigidBody::new(1.0, true, 1.0, bottom_box_size, 0.0, 0.0));
    bottom_wall.move_to(Vector2 { x: 0.0, y: -200.0 });
    bottom_wall.set_is_static(true);

    scene.add_node(bottom_wall);

    let top_box_size = Box::new(400.0, 40.0);
    let mut top_sprite = Sprite::with_color(DARK_GRAY);
    top_sprite.set_shape(top_box_size);
    let mut top_wall = Node::new("top".to_string());
    top_wall.add_component(top_sprite);
    top_wall.add_component(Collider::new(top_box_size));
    top_wall.add_component(RigidBody::new(1.0, true, 1.0, top_box_size, 0.0, 0.0));
    top_wall.move_to(Vector2 { x: 0.0, y: 200.0 });
    top_wall.set_is_static(true);

    scene.add_node(top_wall);

    let left_box_size = Box::new(40.0, 440.0);
    let mut left_sprite = Sprite::with_color(DARK_GRAY);
    left_sprite.set_shape(left_box_size);
    let mut left_wall = Node::new("left".to_string());
    left_wall.add_component(left_sprite);
    left_wall.add_component(Collider::new(left_box_size));
    left_wall.add_component(RigidBody::new(1.0, true, 1.0, left_box_size, 0.0, 0.0));
    left_wall.move_to(Vector2 { x: -200.0, y: 0.0 });
    left_wall.set_is_static(true);

    scene.add_node(left_wall);

    let right_box_size = Box::new(40.0, 440.0);
    let mut right_sprite = Sprite::with_color(DARK_GRAY);
    right_sprite.set_shape(right_box_size);
    let mut right_wall = Node::new("right".to_string());
    right_wall.add_component(right_sprite);
    right_wall.add_component(Collider::new(right_box_size));
    right_wall.add_component(RigidBody::new(1.0, true, 1.0, right_box_size, 0.0, 0.0));
    right_wall.move_to(Vector2 { x: 200.0, y: 0.0 });
    right_wall.set_is_static(true);

    scene.add_node(right_wall);

    game.load_scene(scene);

    let _ = game.run(|game, event, _| {
        if let Event::MouseInput(input, state, pos) = event {
            if state == KeyState::Released {
                let pos = game.screen_to_world(pos).unwrap();
                let scene = game.get_scene_mut().unwrap();
                if state == KeyState::Released {
                    if input == MouseInput::Left {
                        let circle = Circle::new(20.0);
                        let mut circle_sprite = Sprite::with_color(BLUE);
                        circle_sprite.set_shape(circle);
                        let mut circle_node = Node::new("circle".to_string());
                        circle_node.add_component(circle_sprite);
                        circle_node.add_component(Collider::new(circle));
                        circle_node
                            .add_component(RigidBody::new(1.0, false, 0.7, circle, 0.2, 0.5));
                        circle_node.move_to(pos);

                        scene.add_node(circle_node);
                    } else if input == MouseInput::Right {
                        let circle = Circle::new(20.0);
                        let mut circle_sprite = Sprite::with_color(GREEN);
                        circle_sprite.set_shape(circle);
                        let mut circle_node = Node::new("circle".to_string());
                        circle_node.add_component(circle_sprite);
                        circle_node.add_component(Collider::new(circle));
                        circle_node.add_component(SoftBody::new(1.0, false, 0.7, circle, 0.2, 0.5));
                        circle_node.move_to(pos);

                        scene.add_node(circle_node);
                    }
                }
            }
        }

        game.get_scene_mut().unwrap().remove_node_by_id(50).unwrap();

        let ui_fps2 = UiElement::with_text(
            format!("FPS: {}", game.get_fps().unwrap_or_default().round()),
            load_data(include_bytes!("Roboto-Bold.ttf")).unwrap(),
            16.0,
        );
        let mut fps2 = Node::new("FPS".to_string());
        fps2.add_component(ui_fps2);
        fps2.move_to(Vector2 { x: 35.0, y: 10.0 });
        fps2.set_is_static(true);
        fps2.set_id(50);

        game.get_scene_mut().unwrap().add_node(fps2);
    });
}
