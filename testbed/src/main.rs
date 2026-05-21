use macroquad::prelude::*;
use vyxen::{World, math::Vector2, physics2d::bodies::{Rigid, RigidType}};

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

    let mut world = World::new();

    world.add_body(Rigid::new_box(30.0, 30.0, Vector2 { x: 0.0, y: 0.0 }, 1.0, true, 0.5));

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

            world.add_body(Rigid::new_circle(radius, world_pos, density, false, restitution));
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            let mouse_pos = mouse_position();
            let world_pos = camera.screen_to_world(Vec2::new(mouse_pos.0, mouse_pos.1));
            let world_pos = Vector2 { x: world_pos.x, y: world_pos.y };

            let width = rand::gen_range(1.0, 5.0);
            let height = rand::gen_range(1.0, 5.0);
            let density = rand::gen_range(1.0, 10.0);
            let restitution = rand::gen_range(0.0, 1.0);

            world.add_body(Rigid::new_box(width, height, world_pos, density, false, restitution));
        }

        world.step(dt, 10);

        set_camera(&camera);

        clear_background(BLACK);

        draw_line(-100000.0, 0.0, 100000.0, 0.0, 1.0 / zoom, RED);
        draw_line(0.0, -100000.0, 0.0, 100000.0, 1.0 / zoom, GREEN);

        for i in 0..world.get_bodies_len() {
            let body = world.get_body_mut(i).unwrap();
            let world_pos = to_world_coords(body.get_position());
            match body.get_shape_type() {
                RigidType::Circle => {
                    draw_circle(world_pos.x, world_pos.y, body.get_radius(), if body.is_static() { GRAY } else { BLUE });
                }
                RigidType::Box => {
                    let vertices = to_world_coords_multi(body.get_transformed_vertices());
                    if vertices.len() == 4 {
                        draw_triangle(
                            vertices[0],
                            vertices[1],
                            vertices[2],
                            if body.is_static() { GRAY } else { YELLOW }
                        );

                        draw_triangle(
                            vertices[0],
                            vertices[2],
                            vertices[3],
                            if body.is_static() { GRAY } else { YELLOW }
                        );
                    }
                }
            }
        }

        next_frame().await;
    }
}