use macroquad::prelude::*;
use vyxen::{World, geometry::shapes::{Polygon, Box, Circle as VyxenCircle}, math::{Transform, Vector2}, physics2d::bodies::{Rigid, RigidType}};

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

#[macroquad::main("Physics Viewer")]
async fn main() {
    let mut camera_target = vec2(0.0, 10.0);
    let mut zoom = 2.0;

    let mut world = World::new();

    world.add_body(Rigid::new(Vector2 { x: 0.0, y: 0.0 }, 1.0, true, 0.5, Box::new(100.0, 5.0), 0.6, 0.4));

    let mut slope_1 = Rigid::new(Vector2 { x: -10.0, y: 10.0 }, 1.0, true, 0.5, Box::new(20.0, 2.0), 0.6, 0.4);
    slope_1.rotate_by(210.0);
    world.add_body(slope_1);

    let mut slope_2 = Rigid::new(Vector2 { x: 10.0, y: 20.0 }, 1.0, true, 0.5, Box::new(20.0, 2.0), 0.6, 0.4);
    slope_2.rotate_by(-210.0);
    world.add_body(slope_2);

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

            world.add_body(Rigid::new(world_pos, density, false, restitution, VyxenCircle::new(radius), static_friction, dynamic_friction));
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

            world.add_body(Rigid::new(world_pos, density, false, restitution, Box::new(width, height), static_friction, dynamic_friction));
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
                let draw_convex_only = true;
                if draw_convex_only == true {
                    let angle = (i as f32 / amount as f32) * std::f32::consts::TAU;
                    let radius = rand::gen_range(1.0, 5.0);

                    vertices.push(Vector2 {
                        x: angle.cos() * radius,
                        y: angle.sin() * radius,
                    });
                } else {
                    let x = rand::gen_range(-5.0, 5.0);
                    let y = rand::gen_range(-5.0, 5.0);

                    vertices.push(Vector2 { x: x, y: y });
                }
            }

            world.add_body(Rigid::new(world_pos, density, false, restitution, Polygon::new_from_relative_vertices(&vertices), static_friction, dynamic_friction));
        }

        world.step(dt, 10);

        let bottom_left = camera.screen_to_world(vec2(0.0, screen_height()));

        let mut bodies_to_remove: Vec<Rigid> = vec![];
        for i in 0..world.get_bodies_len() {
            let body = world.get_body_mut(i).unwrap();
            let aabb = body.get_aabb();

            if aabb.get_max().y < bottom_left.y {
                bodies_to_remove.push(body.clone());
            }
        }
        for body in bodies_to_remove {
            world.remove_body(&body);
        }

        set_camera(&camera);

        clear_background(BLACK);

        draw_line(-100000.0, 0.0, 100000.0, 0.0, 1.0 / zoom, RED);
        draw_line(0.0, -100000.0, 0.0, 100000.0, 1.0 / zoom, GREEN);

        for i in 0..world.get_bodies_len() {
            let body = world.get_body_mut(i).unwrap();
            let pos = body.get_position();
            let rot = body.get_rotation();
            let is_static = body.is_static();
            let world_pos = to_world_coords(pos);
            
            match body.get_shape_mut() {
                RigidType::Circle(c) => {
                    draw_circle(world_pos.x, world_pos.y, c.get_radius(), if is_static { GRAY } else { BLUE });

                    let va = Vector2::zero();
                    let vb = Vector2 { x: c.get_radius(), y: 0.0 };
                    let transform = Transform::new(body.get_position(), body.get_rotation());
                    let tva = va.transform(&transform);
                    let tvb = vb.transform(&transform);

                    draw_line(tva.x, tva.y, tvb.x, tvb.y, 0.1, WHITE);
                }
                RigidType::Box(b) => {
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
                RigidType::Polygon(p) => {
                    let vertices = to_world_coords_multi(p.get_transformed_vertices(pos, rot));

                    if vertices.len() >= 3 {
                        for i in 1..vertices.len() - 1 {
                            draw_triangle(
                                vertices[0],
                                vertices[i],
                                vertices[i + 1],
                                if is_static { GRAY } else { PURPLE }
                            );
                        }
                    }
                }
                RigidType::Concave(v) => {
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

        next_frame().await;
    }
}