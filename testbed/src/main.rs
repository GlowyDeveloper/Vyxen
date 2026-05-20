use macroquad::prelude::*;

#[macroquad::main("Physics Viewer")]
async fn main() {
    let mut camera_target = vec2(0.0, 0.0);
    let mut zoom = 1.0;

    loop {
        let dt = get_frame_time();

        let camera_speed = 500.0 / zoom;

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
                zoom * 2.0 / screen_width(),
                -zoom * 2.0 / screen_height(),
            ),

            ..Default::default()
        };

        set_camera(&camera);

        clear_background(BLACK);

        draw_line(-100000.0, 0.0, 100000.0, 0.0, 5.0, RED);
        draw_line(0.0, -100000.0, 0.0, 100000.0, 5.0, GREEN);

        draw_circle(200.0, 100.0, 30.0, WHITE);

        next_frame().await;
    }
}