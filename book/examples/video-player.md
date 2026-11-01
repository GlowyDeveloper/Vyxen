# Video Player

## Playground

<link rel="stylesheet" type="text/css" href="../theme/wasm-style.css">

<div id="stage">
  <canvas id="canvas" width="800" height="600"></canvas>
  <div id="status">Loading…</div>
</div>

<script type="module">
  import load from "../theme/wasm-loader.js"
  load("../wasm/video-player/video-player.js")
</script>

## Code

```rust
{{#include ../../examples/video-player/src/main.rs}}
```

## Explanation

At the top of the file, it imports things to be used throughout the file.

```rust
{{#include ../../examples/video-player/src/main.rs:1:3}}
```

Then it saves the Roboto font.

```rust
const ROBOTO: &[u8; 159900] = include_bytes!("Roboto-Bold.ttf");
```

Following that, it has some helper functions for later.

It has two `spawn_future` functions, one for wasm and another for native.

This is due to wasm not being able to be blocked on its main thread.

```rust
#[cfg(not(target_arch = "wasm32"))]
fn spawn_future(fut: impl std::future::Future<Output = ()> + Send + 'static) {
    std::thread::spawn(move || pollster::block_on(fut));
}

#[cfg(target_arch = "wasm32")]
fn spawn_future(fut: impl std::future::Future<Output = ()> + 'static) {
    wasm_bindgen_futures::spawn_local(fut);
}
```

Another function written makes the file picker pop up using `AsyncFileDialog`.

```rust
fn pick_video(video_bytes: &Arc<Mutex<Option<Vec<u8>>>>) {
    let video_bytes = Arc::clone(video_bytes);
    spawn_future(async move {
        let file = AsyncFileDialog::new()
            .add_filter("Video Files", &["mp4"])
            .pick_file()
            .await;
        if let Some(file) = file {
            let bytes = file.read().await;
            *video_bytes.lock().unwrap() = Some(bytes);
        }
    });
}
```

Final function written gets all the control nodes and sets them to a tint.

```rust
fn set_control_tint(game: &mut Game, tint: Color) {
    if let Some(ui) = game
        .get_scene_mut()
        .unwrap()
        .get_node_mut(3)
        .unwrap()
        .get_component_mut::<UiElement>()
    {
        if let ElementType::Text(text) = ui.get_element_type_mut() {
            text.set_tint(tint);
        }
    }

    ..
}
```

At the top of the `main` function, it checks if this is building for android and ios, due to [rfd](https://crates.io/crates/rfd) not supporting those platforms for file pickers.

```rust
#[cfg(any(target_os = "android", target_os = "ios"))]
panic!("RFD (File dialog library) isn't supported on android and ios");
```

Following that, it initializes [console_error_panic_hook](https://crates.io/crates/console_error_panic_hook), this hooks panic messages to the browser console.

```rust
#[cfg(target_arch = "wasm32")]
console_error_panic_hook::set_once();
```

After that, it initializes [console_log](https://crates.io/crates/console_log), this hooks log messages to the browser console.
```rust
#[cfg(target_arch = "wasm32")]
console_log::init_with_level(log::Level::Debug).unwrap();
```

Then it creates a new `Game` and `Scene` and some data.

```rust
let mut game = Game::new();
let mut scene = Scene::new();
let mut picker_visible = true;
let video_bytes: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
```

Then it create the file picker text, as a `Sprite` with a font.

```rust
let mut picker = Node::new("picker".to_string());
picker.add_component(Sprite::with_font(
    "Click to open file picker".to_string(),
    load_data(ROBOTO).unwrap(),
    32.0,
));
picker.set_id(1);
scene.add_node(picker);
```

Following it creates the node for the camera.

Since the video hasn't been picked yet, it's with a transparent color.

```rust
let mut video_node = Node::new("video".to_string());
video_node.add_component(Sprite::with_color(TRANSPARENT));
video_node.set_id(2);
scene.add_node(video_node);
```

Then it creates the control buttons. As an example, here's the stop text.

```rust
let mut stop_text = Node::new("stop_text".to_string());
stop_text.add_component(UiElement::with_text(
    "Stop".to_string(),
    load_data(ROBOTO).unwrap(),
    32.0,
));
stop_text.set_id(3);
stop_text.move_to(Vector2 { x: 50.0, y: 30.0 });
scene.add_node(stop_text);
```

The scene is then loaded, and sets the control tint.

```rust
game.load_scene(scene);

set_control_tint(&mut game, TRANSPARENT);
```

Then it creates a new window config and sets the title to "Video Player".

```rust
let mut window = WindowConfig::new();
window.set_title("Video Player".to_string());

game.set_config(window);
```

It then creates an event loop and a `time` variable outside of the loop and `handle_event` inside of the loop.

```rust
let mut time = 0.0;
let _ = game.run(|game, _context, event, dt| {
    let mut handle_input = None;
    ..
});
```

Inside the event loop, it progresses the time variable `dt` (delta time).

```rust
time += dt;
```

Then it looks at the event and handle them.

```rust
match event {
    _ => {}
}
```

For `Event::MouseInput`, it resets time and calles `pick_video` if the picker is visible, and sets `handle_event` to the position of the cursor.

```rust
Event::MouseInput(_, state, pos) => {
    time = 0.0;
    if state == KeyState::Released {
        if picker_visible {
            pick_video(&video_bytes);
        } else {
            handle_input = Some(pos);
        }
    }
}
```

Then it does the same thing for `Event::Touch`.

```rust
Event::Touch(pos, phase) => {
    time = 0.0;
    if phase == TouchPhase::Ended {
        if picker_visible {
            pick_video(&video_bytes);
        } else {
            handle_input = Some(pos);
        }
    }
}
```

Lastly it resets the time for `Event::CursorMoved`.

```rust
Event::CursorMoved(_) => {
    time = 0.0;
}
```

Once all the events are handled, it sees if `handle_input` is set.

```rust
if let Some(pos) = handle_input { .. }
```

And if it is, it gets the camera.

```rust
if let Some(camera) = game.get_camera() { .. }
```

Next, it needs to convert mouse position to something in the world.

This is due to mouse position's `0,0` is in the **top** right, when a ui element's `0,0` is based on the **bottom** right.

```rust
let h = camera.get_height();
let final_pos = Vector2 {
    x: pos.x,
    y: -(pos.y - h),
};
```

Now that we have the true position, we can calculate what button was pressed.

Vyxen doesn't have a built-in way to process button's (yet), so we need to calculate positions.

```rust
if final_pos.x >= 100.0 && final_pos.x <= 215.0 && final_pos.y <= 60.0 {
    // Restart button
} else if final_pos.x >= 215.0 && final_pos.x <= 290.0 && final_pos.y <= 60.0 {
    // Exit button
} else {
    // Either stop button or somewhere else on the screen.
}
```

If the restart button was pressed, it gets the video node.

```rust
if let Some(sprite) = game
    .get_scene_mut()
    .unwrap()
    .get_node_mut(2)
    .unwrap()
    .get_component_mut::<Sprite>()
{ .. }
```

And then get it's video and calles the restart function.

```rust
if let ElementType::Video(video) = sprite.get_element_type() {
    video.restart();
}
```

If the exit button is pressed it needs to do a little more,

It first sets the control bbuttons to invisible.

```rust
set_control_tint(game, TRANSPARENT);
```

Make the variable that the picker is visible.

```rust
picker_visible = true;
```

And sets the video bytes to nothing.

```rust
video_bytes.lock().unwrap().take();
```

Then it makes the picker text visible.

```rust
if let Some(sprite) = game
    .get_scene_mut()
    .unwrap()
    .get_node_mut(1)
    .unwrap()
    .get_component_mut::<Sprite>()
{
    if let ElementType::Text(text) = sprite.get_element_type_mut() {
        text.set_tint(WHITE);
    }
}
```

Lastly makes video node invisible.

```rust
if let Some(sprite) = game
    .get_scene_mut()
    .unwrap()
    .get_node_mut(2)
    .unwrap()
    .get_component_mut::<Sprite>()
{
    sprite.set_element_type(ElementType::Color(TRANSPARENT));
}
```

For the stop button or the input somewhere else on the screen, it stops the video if it is playing, and plays the video if it's not.

```rust
if let Some(sprite) = game
    .get_scene_mut()
    .unwrap()
    .get_node_mut(2)
    .unwrap()
    .get_component_mut::<Sprite>()
{
    if let ElementType::Video(video) = sprite.get_element_type() {
        if video.is_playing() {
            video.stop();
        } else {
            video.play();
        }
    }
}
```

Outside of the `handle_input` statement, it checks if video_bytes has been set.

```rust
if let Some(bytes) = video_bytes.lock().unwrap().take() { .. }
```

Then it sets the picker to invisible.

```rust
picker_visible = false;
if let Some(sprite) = game
    .get_scene_mut()
    .unwrap()
    .get_node_mut(1)
    .unwrap()
    .get_component_mut::<Sprite>()
{
    if let ElementType::Text(text) = sprite.get_element_type_mut() {
        text.set_tint(TRANSPARENT);
    }
}
```

Then it sets the control bbuttons to visible.

```rust
set_control_tint(game, WHITE);
```

Then it loads the video and starts playing.

```rust
if let Some(sprite) = game
    .get_scene_mut()
    .unwrap()
    .get_node_mut(2)
    .unwrap()
    .get_component_mut::<Sprite>()
{
    let video = Video::new(bytes);
    video.play();
    sprite.set_element_type(ElementType::Video(video));
}
```

Outside of the `video_bytes` statement, it checks if the camera is set, gets the width and height and drops the camera due to borrow checking issues.

```rust
if let Some(camera) = game.get_camera() {
    let h = camera.get_height();
    let w = camera.get_width();

    let _ = camera;

    ..
}
```


Then it gets the video's width and height.

```rust
if let Some(sprite) = game
    .get_scene_mut()
    .unwrap()
    .get_node_mut(2)
    .unwrap()
    .get_component_mut::<Sprite>()
{
    if let ElementType::Video(video) = sprite.get_element_type() {
        if let Some(video_w) = video.get_width()
            && let Some(video_h) = video.get_height()
        { .. }
    }
}
```

Lastly is calculates the largest size that the video can fit inside of the camera's view.

```rust
let scale = (w / video_w as f32).min(h / video_h as f32);

let fit_w = video_w as f32 * scale;
let fit_h = video_h as f32 * scale;
sprite.set_shape(Box::new(fit_w, fit_h));
```

> [!CAUTION]
> The use of `Box::new` here is Vyxen's `vyxen::Box` type, not the standard library's `std::boxed::Box` type. If you are using the standard library's box type in the same file, instead import it one of them with an alias. For example: `use std::boxed::Box as StdBox;`.


For the last statement, it does the animation of the control nodes fading out.

```rust
if !picker_visible {
    if time <= 0.0 {
        // Nodes become visible
        set_control_tint(game, Color::from_rgba(1.0, 1.0, 1.0, 1.0));
    } else if time >= 6.0 {
        // Finished animation
        set_control_tint(game, Color::from_rgba(1.0, 1.0, 1.0, 0.0));
    } else if time >= 5.0 {
        // Starting animation
        let t = time - 5.0;
        let alpha = (1.0 - t).clamp(0.0, 1.0);
        set_control_tint(game, Color::from_rgba(1.0, 1.0, 1.0, alpha));
    }
}
```