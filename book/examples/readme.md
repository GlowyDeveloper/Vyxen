# Readme

## Playground

<canvas id="canvas" width="640" height="480"></canvas>
<script type="module">
  import init from "../wasm/readme/readme.js";
  init();
</script>

## Code

```rust
{{#include ../../examples/readme/src/main.rs}}
```

## Explanation

At the top of the file, it imports vyxen prelude. This gives us access to a lot of Vyxen's types and functions that are commonly used.

```rust
{{#include ../../examples/readme/src/main.rs:1}}
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

Next, it creates a `Sprite` and sets its shape to a `Box` measuring 200x20.

```rust
let mut sprite = Sprite::new();
sprite.set_shape(Box::new(200.0, 20.0));
```

> [!CAUTION]
> The use of `Box::new` here is Vyxen's `vyxen::geometry::Box` type, not the standard library's `std::boxed::Box` type. If you are using the standard library's box type in the same file, instead import it one of them with an alias. For example: `use std::boxed::Box as StdBox;`.

It then sets the sprite's element type to a solid color using `ElementType::Color`.

```rust
sprite.set_element_type(ElementType::Color(Color::from_rgb(0.2, 0.8, 0.3)));
```

Then it creates a `Node` and adds the `Sprite` to it.

```rust
let mut node = Node::new("Foo".to_string());
node.add_component(sprite);
```

It makes the node static, so it will be moved manually via input.

```rust
node.set_is_static(true);
```

It then sets the node's physics process, a function that runs every physics step.

```rust
node.set_physics_process(move |node, _, dt, ctx| { .. });
```

Inside the closure, it calculates a speed value scaled by `dt`.

```rust
let speed = 20.0 * dt;
```

It then checks which movement keys are currently held, moving the node up, down, left or right.

```rust
if ctx.is_held(KeyCode::KeyW) {
    node.move_by(Vector2 {
        x: 0.0,
        y: speed * dt,
    });
}
if ctx.is_held(KeyCode::KeyA) {
    node.move_by(Vector2 {
        x: -speed * dt,
        y: 0.0,
    });
}
if ctx.is_held(KeyCode::KeyS) {
    node.move_by(Vector2 {
        x: 0.0,
        y: -speed * dt,
    });
}
if ctx.is_held(KeyCode::KeyD) {
    node.move_by(Vector2 {
        x: speed * dt,
        y: 0.0,
    });
}
```

Once the physics process is set, the node is added to the scene, and the scene is loaded.

```rust
scene.add_node(node);

game.load_scene(scene);
```

Then it creates a new window config and sets the title to "README example".

```rust
let mut window = WindowConfig::new();
window.set_title("README example".to_string());

game.set_config(window);
```

Finally, since this example doesn't need any custom per-frame logic, it runs the game with `run_without_callback`.

```rust
game.run_without_callback().unwrap();
```