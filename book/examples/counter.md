# Counter

## Playground

<canvas id="canvas" width="640" height="480"></canvas>
<script type="module">
  import init from "../wasm/counter/counter.js";
  init();
</script>

## Code

```rust
{{#include ../../examples/counter/src/main.rs}}
```

## Explanation

At the top of the file, it imports vyxen prelude. This gives us access to a lot of Vyxen's types and functions that are commonly used.

```rust
{{#include ../../examples/counter/src/main.rs:1}}
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

Then it creates a new `Game` and `Scene` and a counter variable.

```rust
let mut counter = 0;

let mut game = Game::new();

let mut scene = Scene::new();
```

Next, it creates a `UiElement` to display the current count as text.

```rust
let text = UiElement::with_text(
    format!("Clicks: {}", clicks),
    load_data(include_bytes!("Roboto-Bold.ttf")).unwrap(),
    64.0,
);
```

Then it creates a `Node` for the text, adds the `UiElement` to it.

```rust
let mut node = Node::new("text".to_string());
node.add_component(text);
```

It then sets the node's position to the center of the screen.

```rust
node.move_to(Vector2 { x: 0.0, y: 0.0 });
```

It makes the node static, and gives it an id of `2` so it can be found and removed later.

```rust
node.set_is_static(true);
node.set_id(2);
```

The node is then added to the scene.

```rust
scene.add_node(node);
```

Once the node is added, the scene is loaded.

```rust
game.load_scene(scene);
```

Then it creates a new window config and sets the title to "Counter".

```rust
let mut window = WindowConfig::new();
window.set_title("Counter".to_string());

game.set_config(window);
```

It then creates an event loop.

```rust
let _ = game.run(move |game, event, _| { .. });
```

Inside the event loop, it checks if the event is a `Event::MouseInput` and checks if the state is `KeyState::Released`.

```rust
if let Event::MouseInput(_, state, _) = event {
    if state == KeyState::Released { .. }
}
```

And increases `clicks` if the state is `KeyState::Released`.

```rust
clicks += 1;
```

The old text is removed.

```rust
let _ = game.get_scene_mut().unwrap().remove_node_by_id(2);
```

It then gets the camera, returning early if one doesn't exist.

```rust
let camera = match game.get_camera() {
    Some(camera) => camera,
    None => return,
};
```

It calculates the center of the screen with the camera's dimensions.

```rust
let pos = Vector2 {
    x: camera.get_width(),
    y: camera.get_height(),
} / 2.0;
```

Finally, it recreates the text `UiElement` with the updated click count, and adds it back to the scene.

```rust
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
```