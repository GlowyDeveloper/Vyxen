# Ball pit

## Playground

<canvas id="canvas" width="640" height="480"></canvas>
<script type="module">
  import init from "../wasm/ball-pit/ball-pit.js";
  init();
</script>

## Code

```rust
{{#include ../../examples/ball-pit/src/main.rs}}
```

## Explanation

At the top of the file, it imports vyxen prelude. This gives us access to a lot of Vyxen's types and functions that are commonly used.

```rust
{{#include ../../examples/ball-pit/src/main.rs:1}}
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

Once the `Game` and `Scene` are created, it increases the scene's gravity by about 15x.

```rust
scene.set_gravity(Vector2 { x: 0.0, y: -150.0 });
```

The default gravity is `Vector2 { x: 0.0, y: -9.81 }`.

Then it creates the walls of the box. Let's take the top wall as an example.

It starts by creating the size of the wall.

```rust
let top_box_size = Box::new(400.0, 40.0);
```

> [!CAUTION]
> The use of `Box::new` here is Vyxen's `vyxen::geometry::Box` type, not the standard library's `std::boxed::Box` type. If you are using the standard library's box type in the same file, instead import it one of them with an alias. For example: `use std::boxed::Box as StdBox;`.

After, it creates the `Sprite` for the wall, sets its color to white, and sets its shape to the box size. This makes the wall be rendered to the screen.

```rust
let mut top_sprite = Sprite::with_color(WHITE);
top_sprite.set_shape(top_box_size);
```

Then it creates the `Node` for the wall and adds the `Sprite` to it.

```rust
let mut top_wall = Node::new("top".to_string());
top_wall.add_component(top_sprite);
```

It also adds a `Collider` component to the wall. This is what makes the wall collide with other objects.

```rust
top_wall.add_component(Collider::new(top_box_size));
```

It adds one last component to the wall, a `RigidBody`.

```rust
top_wall.add_component(RigidBody::new(1.0, 1.0, top_box_size, 0.0, 0.0));
```

This means the `RigidBody` will have:
- density: 1.0
- restitution: 1.0
- size: top_box_size
- static friction: 0.0
- dynamic friction: 0.0

It sets the position of the wall to `(0, 200)`.

```rust
top_wall.move_to(Vector2 { x: 0.0, y: 200.0 });
```

It makes the wall static. This makes it not move when other objects collide with it or be affected by gravity.

```rust
top_wall.set_is_static(true);
```

Lastly, it adds the wall to the scene.

```rust
scene.add_node(top_wall);
```

Once all the walls are added, the scene is loaded.

```rust
game.load_scene(scene);
```

Then it creates a new window config and sets the title to "Ball Pit".

```rust
let mut window = WindowConfig::new();
window.set_title("Ball Pit".to_string());

game.set_config(window);
```

It then creates an event loop.

```rust
let _ = game.run(|game, event, dt| { .. });
```

Inside the event loop, it checks if the event is a `Event::MouseInput` and checks if the state is `KeyState::Released`.

```rust
if let Event::MouseInput(input, state, pos) = event {
    if state == KeyState::Released { .. }
}
```

It then converts the mouse position to world coordinates and gets the scene.

```rust
let pos = game.screen_to_world(pos).unwrap();
let scene = game.get_scene_mut().unwrap();
```

It then checks if the input was a `MouseButton::Left` click.

```rust
if input == MouseButton::Left {
    let circle = Circle::new(20.0);
    let mut circle_sprite = Sprite::with_color(BLUE);
    circle_sprite.set_shape(circle);
    let mut circle_node = Node::new("circle".to_string());
    circle_node.add_component(circle_sprite);
    circle_node.add_component(Collider::new(circle));
    circle_node
        .add_component(RigidBody::new(1.0, 0.7, circle, 0.2, 0.5));
    circle_node.move_to(pos);

    scene.add_node(circle_node);
}
```

And also checks if the input was a `MouseButton::Right` click.

```rust
if input == MouseButton::Right {
    let circle = Circle::new(20.0);
    let mut circle_sprite = Sprite::with_color(GREEN);
    circle_sprite.set_shape(circle);
    let mut circle_node = Node::new("circle".to_string());
    circle_node.add_component(circle_sprite);
    circle_node.add_component(Collider::new(circle));
    circle_node.add_component(SoftBody::new(1.0, 0.7, circle, 0.2, 0.5));
    circle_node.move_to(pos);
    
    scene.add_node(circle_node);
}
```

At the end of the event loop, it creates an fps counter.

First it removes the old fps counter node, if it exists.

```rust
game.get_scene_mut().unwrap().remove_node_by_id(50).unwrap();
```

Then it creates a new ui element to display the fps counter.

```rust
let ui_fps2 = UiElement::with_text(
    format!("FPS: {}", game.get_fps().unwrap_or_default().round()),
    load_data(include_bytes!("Roboto-Bold.ttf")).unwrap(),
    16.0,
);
```

It creates a new node to hold the ui element and adds it to the scene.

```rust
let mut fps2 = Node::new("FPS".to_string());
fps2.add_component(ui_fps2);
fps2.move_to(Vector2 { x: 35.0, y: 10.0 });
fps2.set_is_static(true);
fps2.set_id(50);

game.get_scene_mut().unwrap().add_node(fps2);
```