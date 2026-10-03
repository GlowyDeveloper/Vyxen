# Radio

## Playground

<link rel="stylesheet" type="text/css" href="../theme/wasm-style.css">

<div id="stage">
  <canvas id="canvas" width="800" height="600"></canvas>
  <div id="status">Loading…</div>
</div>

<script type="module">
  import load from "../theme/wasm-loader.js"
  load("../wasm/radio/radio.js")
</script>

## Code

```rust
{{#include ../../examples/radio/src/main.rs}}
```

## Explanation

At the top of the file, it imports vyxen prelude. This gives us access to a lot of Vyxen's types and functions that are commonly used.

```rust
{{#include ../../examples/radio/src/main.rs:1}}
```

Afterwards, it includes the bytes of all the assets used.

```rust
const RADIO: &[u8] = include_bytes!("resources/radio.png");
const CROTCHET: &[u8] = include_bytes!("resources/crotchet.png");
const QUAVER: &[u8] = include_bytes!("resources/quaver.png");
const MUSIC: &[u8] = include_bytes!("resources/radio.wav");
const ROBOTO: &[u8] = include_bytes!("resources/Roboto-Bold.ttf");
```

At the top of the `main` function, it initializes [console_error_panic_hook](https://crates.io/crates/console_error_panic_hook), this hooks panic messages to the browser console.

```rust
#[cfg(target_arch = "wasm32")]
console_error_panic_hook::set_once();
```

After that, it initializes [console_log](https://crates.io/crates/console_log), this hooks log messages to the browser console.

```rust
#[cfg(target_arch = "wasm32")]
console_log::init_with_level(log::Level::Debug).unwrap();
```

Then it creates a new `Game` and `Scene`.

```rust
let mut game = Game::new();
let mut scene = Scene::new();
```

Next, it creates a `Sprite` for the radio using the radio image as its texture.

```rust
let mut radio_sprite = Sprite::with_texture(load_data(RADIO).unwrap());
```

It sets the sprite's z value to `1.0`, then sets its shape to a `Box` measuring 200x328.

```rust
radio_sprite.set_z(1.0);
radio_sprite.set_shape(Box::new(200.0, 328.0));
```

> [!CAUTION]
> The use of `Box::new` here is Vyxen's `vyxen::geometry::Box` type, not the standard library's `std::boxed::Box` type. If you are using the standard library's box type in the same file, instead import it one of them with an alias. For example: `use std::boxed::Box as StdBox;`.

Then it creates a `Node` for the radio and adds the `Sprite` to it.

```rust
let mut radio_node = Node::new("Radio".to_string());
radio_node.add_component(radio_sprite.clone());
```

It makes the node static, so it isn't moved by physics, and moves it to the center of the world.

```rust
radio_node.set_is_static(true);
radio_node.move_to(Vector2 { x: 0.0, y: 0.0 });
```

It gives the node an id of `2`, then adds it to the scene.

```rust
radio_node.set_id(2);

scene.add_node(radio_node);
```

After the radio, it creates the text at the bottom of the screen.

```rust
let mut start_stop_text = Text::new(
    "Click/Tap to start/stop".to_string(),
    load_data(ROBOTO).unwrap(),
    48.0,
);
```

It sets the text's tint to black and anchors it to the center.

```rust
start_stop_text.set_tint(BLACK);
start_stop_text.set_anchor(TextAnchor::Center);
```

Then it creates a `Sprite` and sets its element type to the text using `ElementType::Text`.

```rust
let mut start_stop_sprite = Sprite::new();
start_stop_sprite.set_element_type(ElementType::Text(start_stop_text));
```

It creates a `Node` for the text, adds the `Sprite` to it, and moves it below the radio.

```rust
let mut start_stop = Node::new("StartStop".to_string());
start_stop.add_component(start_stop_sprite);
start_stop.move_to(Vector2 { x: 0.0, y: -250.0 });
```

It gives the node an id of `3`, so it can be found later, and adds it to the scene.

```rust
start_stop.set_id(3);

scene.add_node(start_stop);
```

Once both nodes are added, the scene is loaded.

```rust
game.load_scene(scene);
```

Then it creates a new window config, sets the title to "Radio" and the background color to white.

```rust
let mut window = WindowConfig::new();
window.set_title("Radio".to_string());
window.set_background_color(WHITE);

game.set_config(window);
```

Then it initializes some states for the event loop.

```rust
let mut sound_handle: Option<SoundHandle> = None;
let audio = load_data(MUSIC).unwrap();
let mut time_since_note = 0.0;
```

It then creates an event loop.

```rust
let _ = game.run(move |game, _, event, dt| { .. });
```

At the start of the event loop, it adds `dt` (delta time) to the timer.

```rust
time_since_note += dt;
```

Next, it checks if the user clicked or tapped and if those have been released.

```rust
let clicked = match event {
    Event::Touch(_, phase) => phase == TouchPhase::Ended,
    Event::MouseInput(_, state, _) => state == KeyState::Released,
    _ => false,
};
```

If the user clicked, it checks whether a sound is already playing by taking the handle out of `sound_handle`.

```rust
if clicked {
    if let Some(handle) = sound_handle.take() { .. } else { .. }
}
```

If there was a handle, the music is playing, so it stops the sound.

```rust
handle.stop();
```

It then gets the scene and collects the ids of the children of the text node.

```rust
let scene = game.get_scene_mut().unwrap();
let child_ids: Vec<_> = scene.get_node(3).unwrap().get_children_ids().to_vec();
```

And removes each of them from the scene. This clears all the music notes that are still on screen.

```rust
for id in child_ids {
    scene.remove_node_by_id(id).unwrap();
}
```

If there wasn't a handle, it plays the music and stores the new handle.

```rust
sound_handle = game.play_sound_with(&audio, 1.0, true);
```

After handling the click, it gets the scene.

```rust
let scene = game.get_scene_mut().unwrap();
```

Then it checks if enough time has passed to spawn a new note using a random number of seconds between `1.0` and `10.0`.

```rust
if time_since_note >= Random::from_time().range_f32(1.0..10.0) { .. }
```

Inside, it resets the timer.

```rust
time_since_note = 0.0;
```

It only spawns a note if the music is currently playing.

```rust
if sound_handle.is_some() { .. }
```

To spawn a note, it creates a new `Node`.

```rust
let mut note = Node::new("Note".to_string());
```

It randomly picks between the crotchet and quaver images.

```rust
let texture = if Random::from_time().range_u32(0..2) == 0 {
    CROTCHET
} else {
    QUAVER
};
```

Then it creates a `Sprite` from that texture, sets its shape to a 50x50 `Box`, and adds it to the node.

```rust
let mut sprite = Sprite::with_texture(load_data(texture).unwrap());
sprite.set_shape(Box::new(50.0, 50.0));
note.add_component(sprite);
```

It moves the note to a random x position between `-300` and `300`, at the bottom of the screen, and gives it a random rotation.

```rust
note.move_to(Vector2 { x: Random::from_time().range_f32(-300.0..300.0), y: -300.0 });
note.rotate_to(Random::from_time().range_f32(-1.0..1.0));
```

It then sets the note's physics process, which moves the note up the screen every physics step.

```rust
note.set_physics_process(|node, _, dt, _| {
    node.move_by(Vector2 { x: 0.0, y: dt * 100.0 });
});
```

Finally it sets the note as the child of the text node with id `3`.

```rust
let id = note.get_id();
scene.add_node(note);
scene.get_node_mut(3).unwrap().add_child(id);
```