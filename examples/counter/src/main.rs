use vyxen::prelude::*;

fn main() {
    let mut clicks = 0;
    
    let mut game = Game::new();
    
    let mut scene = Scene::new();

    let text = UiElement::with_text(
        format!("Clicks: {}", clicks),
        load_data(include_bytes!("Roboto-Bold.ttf")).unwrap(),
        64.0,
    );
    let mut node = Node::new("text".to_string());
    node.add_component(text);
    node.move_to(Vector2 { x: 0.0, y: 0.0 });
    node.set_is_static(true);
    node.set_id(2);

    scene.add_node(node);

    game.load_scene(scene);

    let _ = game.run(move |game, event, _| {
        match event {
            Event::MouseInput(_, state, _) => {
                if state == KeyState::Released {
                    clicks += 1;
                }
            }
            _ => {}
        };

        let _ = game.get_scene_mut().unwrap().remove_node_by_id(2);

        let camera = game.get_camera().unwrap();
        let pos = Vector2 { x: camera.get_width(), y: camera.get_height() } / 2.0;

        let text = UiElement::with_text(
            format!("Clicks: {}", clicks),
            load_data(include_bytes!("Roboto-Bold.ttf")).unwrap(),
            64.0,
        );
        let mut node = Node::new("text".to_string());
        node.add_component(text);
        node.move_to(pos);
        node.set_is_static(true);
        node.set_id(2);

        game.get_scene_mut().unwrap().add_node(node);
    });
}
