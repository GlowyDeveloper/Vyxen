use macroquad::prelude::*;
use vyxen_physics2d::{bodies::{Rigid, RigidType}, collision::{Collision, intersect_circles, intersect_polygon_circle, intersect_polygons}};
use vyxen_math::Vector2;

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
    let mut camera_target = vec2(0.0, 0.0);
    let mut zoom = 1.0;

    let mut bodies: Vec<Rigid> = Vec::new();

    bodies.push(Rigid::new_circle(5.0, Vector2 { x: 10.0, y: 10.0 }, 1.0, false, 0.5));
    bodies.push(Rigid::new_circle(5.0, Vector2 { x: -10.0, y: -10.0 }, 1.0, false, 0.5));
    bodies.push(Rigid::new_box(5.0, 5.0, Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5));
    bodies.push(Rigid::new_box(5.0, 10.0, Vector2 { x: 10.0, y: -10.0 }, 1.0, false, 0.5));

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

        let mut dx = 0.0;
        let mut dy = 0.0;
        let speed = 16.0;

        if is_key_down(KeyCode::Up) {dy += 1.0;}
        if is_key_down(KeyCode::Down) {dy -= 1.0;}
        if is_key_down(KeyCode::Left) {dx -= 1.0;}
        if is_key_down(KeyCode::Right) {dx += 1.0;}

        if dx != 0.0 || dy != 0.0 {
            let movement = Vector2 { x: dx, y: dy }.normalize();
            let velocity = movement * speed * dt;
            bodies[0].move_by(velocity);
        }

        let len = bodies.len();
        for i in 0..len {
            for j in (i + 1)..len {
                let (left, right) = bodies.split_at_mut(j);
                let body_a = &mut left[i];
                let body_b = &mut right[0];

                let collision = match (body_a.get_shape_type(), body_b.get_shape_type()) {
                    (RigidType::Circle, RigidType::Circle) => intersect_circles(body_a.get_position(), body_a.get_radius(), body_b.get_position(), body_b.get_radius()),
                    (RigidType::Box, RigidType::Box) => intersect_polygons(body_a.get_transformed_vertices(), body_b.get_transformed_vertices()),
                    (RigidType::Box, RigidType::Circle) => intersect_polygon_circle(body_b.get_position(), body_b.get_radius(), body_a.get_transformed_vertices()).map(|c| Collision { normal: -c.normal, depth: c.depth }),
                    (RigidType::Circle, RigidType::Box) => intersect_polygon_circle(body_a.get_position(), body_a.get_radius(), body_b.get_transformed_vertices()),
                };

                if let Some(collision) = collision {
                    let (left, right) = bodies.split_at_mut(j);

                    let body_a = &mut left[i];
                    let body_b = &mut right[0];

                    body_a.move_by(-collision.normal * collision.depth / 2.0);
                    body_b.move_by(collision.normal * collision.depth / 2.0);
                }
            }
        }

        set_camera(&camera);

        clear_background(BLACK);

        draw_line(-100000.0, 0.0, 100000.0, 0.0, 1.0 / zoom, RED);
        draw_line(0.0, -100000.0, 0.0, 100000.0, 1.0 / zoom, GREEN);

        for i in 0..bodies.len() {
            let body = bodies.get_mut(i).unwrap();
            let world_pos = to_world_coords(body.get_position());
            match body.get_shape_type() {
                RigidType::Circle => {
                    draw_circle(world_pos.x, world_pos.y, body.get_radius(), BLUE);
                }
                RigidType::Box => {
                    let vertices = to_world_coords_multi(body.get_transformed_vertices());
                    if vertices.len() == 4 {
                        draw_triangle(
                            vertices[0],
                            vertices[1],
                            vertices[2],
                            YELLOW
                        );

                        draw_triangle(
                            vertices[0],
                            vertices[2],
                            vertices[3],
                            YELLOW
                        );
                    }
                }
            }
        }

        next_frame().await;
    }
}