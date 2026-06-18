use macroquad::prelude::*;
use vyxen::{Node, World, components::Collider, geometry::{Box, Circle as VyxenCircle, Polygon, ShapeType}, math::{Transform, Vector2}, physics2d::Rigid};

fn to_world_coords(v: Vector2) -> Vec2 {
    vec2(v.x, v.y)
}

fn to_world_coords_multi(vec: &[Vector2]) -> Vec<Vec2> {
    let mut res: Vec<Vec2> = vec![];
    for v in vec {
        res.push(to_world_coords(*v));
    }
    res
}

#[macroquad::main("Testbed")]
async fn main() {
    let mut camera_target = vec2(0.0, 10.0);
    let mut zoom = 2.0;

    let mut world = World::new();

    let mut ground_node = Node::new("Ground".to_string());
    ground_node.set_is_static(true);
    ground_node.move_to(Vector2 { x: 0.0, y: 0.0 });
    ground_node.add_component(Rigid::new(1.0, true, 0.5, Box::new(100.0, 5.0), 0.6, 0.4));
    ground_node.add_component(Collider::new(Box::new(100.0, 5.0)));
    world.add_node(ground_node);

    let mut slope_1_node = Node::new("Slope1".to_string());
    slope_1_node.set_is_static(true);
    slope_1_node.move_to(Vector2 { x: -10.0, y: 10.0 });
    slope_1_node.rotate_by(-0.3);
    slope_1_node.add_component(Rigid::new(1.0, true, 0.5, Box::new(20.0, 2.0), 0.6, 0.4));
    slope_1_node.add_component(Collider::new(Box::new(20.0, 2.0)));
    world.add_node(slope_1_node);

    let mut slope_2_node = Node::new("Slope2".to_string());
    slope_2_node.set_is_static(true);
    slope_2_node.move_to(Vector2 { x: 10.0, y: 20.0 });
    slope_2_node.rotate_by(0.3);
    slope_2_node.add_component(Rigid::new(1.0, true, 0.5, Box::new(20.0, 2.0), 0.6, 0.4));
    slope_2_node.add_component(Collider::new(Box::new(20.0, 2.0)));
    world.add_node(slope_2_node);

    for i in 0..100 {
        let mut node = Node::new("Circle".to_string());
        node.move_to(Vector2 { x: i as f32 * 0.75, y: 10.0 });
        node.add_component(Rigid::new(1.0, false, 1.0, VyxenCircle::new(1.0), 0.6, 0.4));
        node.add_component(Collider::new(VyxenCircle::new(1.0)));
        world.add_node(node);
    }

    loop {
        let dt = get_frame_time();

        let camera_speed = 50.0 / zoom;

        if is_key_down(KeyCode::W) {
            camera_target.y += camera_speed * dt;
        }

        if is_key_down(KeyCode::S) {
            camera_target.y -= camera_speed * dt;
        }

        if is_key_down(KeyCode::A) {
            camera_target.x -= camera_speed * dt;
        }

        if is_key_down(KeyCode::D) {
            camera_target.x += camera_speed * dt;
        }

        let (_, wheel_y) = mouse_wheel();

        if wheel_y != 0.0 {
            zoom *= 1.0 + wheel_y * 0.001;
            zoom = zoom.clamp(0.1, 10.0);
        }

        let camera = Camera2D {
            target: camera_target,

            zoom: vec2(
                zoom * 20.0 / screen_width(),
                -zoom * 20.0 / screen_height(),
            ),

            ..Default::default()
        };

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let world_pos = camera.screen_to_world(Vec2::new(mouse_pos.0, mouse_pos.1));
            let world_pos = Vector2 { x: world_pos.x, y: world_pos.y };

            let radius = rand::gen_range(1.0, 5.0);
            let density = rand::gen_range(1.0, 10.0);
            let restitution = rand::gen_range(0.0, 1.0);

            let static_friction = rand::gen_range(0.0, 1.0);
            let dynamic_friction = rand::gen_range(0.0, 1.0);

            let mut node = Node::new("Circle".to_string());
            node.move_to(world_pos);
            node.add_component(Rigid::new(density, false, restitution, VyxenCircle::new(radius), static_friction, dynamic_friction));
            node.add_component(Collider::new(VyxenCircle::new(radius)));
            world.add_node(node);
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            let mouse_pos = mouse_position();
            let world_pos = camera.screen_to_world(Vec2::new(mouse_pos.0, mouse_pos.1));
            let world_pos = Vector2 { x: world_pos.x, y: world_pos.y };

            let width = rand::gen_range(1.0, 5.0);
            let height = rand::gen_range(1.0, 5.0);
            let density = rand::gen_range(1.0, 10.0);
            let restitution = rand::gen_range(0.0, 1.0);

            let static_friction = rand::gen_range(0.0, 1.0);
            let dynamic_friction = rand::gen_range(0.0, 1.0);

            let mut node = Node::new("Box".to_string());
            node.move_to(world_pos);
            node.add_component(Rigid::new(density, false, restitution, Box::new(width, height), static_friction, dynamic_friction));
            node.add_component(Collider::new(Box::new(width, height)));
            world.add_node(node);
        }

        if is_mouse_button_pressed(MouseButton::Middle) {
            let mouse_pos = mouse_position();
            let world_pos = camera.screen_to_world(Vec2::new(mouse_pos.0, mouse_pos.1));
            let world_pos = Vector2 { x: world_pos.x, y: world_pos.y };

            let density = rand::gen_range(1.0, 10.0);
            let restitution = rand::gen_range(0.0, 1.0);

            let static_friction = rand::gen_range(0.0, 1.0);
            let dynamic_friction = rand::gen_range(0.0, 1.0);

            let mut vertices = vec![];
            let amount = rand::gen_range(3, 10);
            for i in 0..amount {
                let angle = (i as f32 / amount as f32) * std::f32::consts::TAU;
                let radius = rand::gen_range(1.0, 5.0);

                vertices.push(Vector2 {
                    x: angle.cos() * radius,
                    y: angle.sin() * radius,
                });
            }

            let mut node = Node::new("Polygon".to_string());
            node.move_to(world_pos);
            node.add_component(Rigid::new(density, false, restitution, Polygon::new_from_relative_vertices(&vertices), static_friction, dynamic_friction));
            node.add_component(Collider::new(Polygon::new_from_relative_vertices(&vertices)));
            world.add_node(node);
        }

        world.step(dt);

        set_camera(&camera);

        clear_background(BLACK);

        draw_line(-100000.0, 0.0, 100000.0, 0.0, 1.0 / zoom, RED);
        draw_line(0.0, -100000.0, 0.0, 100000.0, 1.0 / zoom, GREEN);

        let child_ids = world.get_root_mut().get_children_ids().clone();
        for id in child_ids {
            if let Some(node) = world.get_nodes_mut().get_mut(&id) {
                let pos = node.get_position();
                let rot = node.get_rotation();
                let is_static = node.is_static();
                let world_pos = to_world_coords(pos);

                if let Some(body) = node.get_component_mut::<Rigid>() {
                    match body.get_shape_mut() {
                        ShapeType::Circle(c) => {
                            draw_circle(world_pos.x, world_pos.y, c.get_radius(), if is_static { GRAY } else { BLUE });

                            let va = Vector2::zero();
                            let vb = Vector2 { x: c.get_radius(), y: 0.0 };
                            let transform = Transform::new(pos, rot);
                            let tva = va.transform(&transform);
                            let tvb = vb.transform(&transform);

                            draw_line(tva.x, tva.y, tvb.x, tvb.y, 0.1, WHITE);
                        }
                        ShapeType::Box(b) => {
                            let vertices = to_world_coords_multi(b.get_transformed_vertices(pos, rot));
                            if vertices.len() == 4 {
                                draw_triangle(
                                    vertices[0],
                                    vertices[1],
                                    vertices[2],
                                    if is_static { GRAY } else { YELLOW }
                                );

                                draw_triangle(
                                    vertices[0],
                                    vertices[2],
                                    vertices[3],
                                    if is_static { GRAY } else { YELLOW }
                                );
                            }
                        }
                        ShapeType::Polygon(p) => {
                            let triangles = Polygon::triangulate(p.get_vertices());
                            for mut polygon in triangles {
                                let vertices = to_world_coords_multi(polygon.get_transformed_vertices(pos, rot));
                                draw_triangle(
                                    vertices[0],
                                    vertices[1],
                                    vertices[2],
                                    if is_static { GRAY } else { PURPLE }
                                );
                            }
                        }
                        ShapeType::Concave(v) => {
                            for p in v {
                                let vertices = to_world_coords_multi(p.get_transformed_vertices(pos, rot));

                                if vertices.len() >= 3 {
                                    for i in 1..vertices.len() - 1 {
                                        draw_triangle(
                                            vertices[0],
                                            vertices[i],
                                            vertices[i + 1],
                                            if is_static { GRAY } else { ORANGE }
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        set_default_camera();

        draw_text(
            &get_fps().to_string(),
            20.0,
            40.0,
            40.0,
            YELLOW,
        );

        next_frame().await;
    }
}