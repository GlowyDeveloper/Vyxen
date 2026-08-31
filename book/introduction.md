# Introduction

Vyxen is performance focused game engine.

Vyxen's api is designed to be similar to godot, while being powerful.

View the [documentation](https://docs.rs/vyxen) for more information.

View the [examples](./examples/examples.html) to see how to use Vyxen.

## Installation

To get started, add the following to your `Cargo.toml` file:

```toml
vyxen = "0.1.0"
```

## Creating a game

Vyxen has a root `game`.

```rust
use vyxen::prelude::*;

let game = Game::new();
```

## Creating and loading a scene

Scenes are used to hold all nodes, like godot.

```rust
use vyxen::prelude::*;

let scene = Scene::new();
```

Scenes are then loaded into the game.

```rust
use vyxen::prelude::*;

let mut game = Game::new();
let scene = Scene::new();

game.load_scene(scene);
```

## Adding nodes

Nodes are the main focus of Vyxen.

Nodes are generic. There's no pre-made nodes.

```rust
use vyxen::prelude::*;

let mut scene = Scene::new();

let node = Node::new("Foo".to_string());
scene.add_node(node);
```

## Components

Components are used to add behavior and data, such as colliders, to a node.

The currently implemented components are:
 - Collider
 - RigidBody
 - SoftBody
 - Sprite

```rust
use vyxen::prelude::*;

let mut node = Node::new("Foo".to_string());
let collider = Collider::new(Circle::new(1.0));

node.add_component(collider);
```

## Scripts

Scripts let you customize node behavior.

The overridable methods are:
 - on_ready
 - process
 - physics_process
 - on_collision

```rust
use vyxen::prelude::*;

let mut node = Node::new("Foo".to_string());
node.set_physics_process(|_, _, _, _| {
    println!("Processing...");
});
```

## Rendering

The currently supported OS:

|API       |Windows|Linux/Android|MacOs/iOS|Web|
|----------|-------|-------------|---------|---|
|Vulkan    |✅    |✅           |1️⃣      |   |
|Metal     |       |             |✅      |   |
|DirectX 12|✅    |             |         |   |
|OpenGL    |✅    |✅           |2️⃣      |✅|
|WebGPU    |       |             |         |❗|
|Tested    |✅    |✅           |         |✅|

✅ = Works
1️⃣ = MoltenVK required
2️⃣ = ANGLE required
❗ = Unsupported on some browsers or platforms

To render the scene:

```rust, no_run
use vyxen::prelude::*;

let mut game = Game::new();
let mut scene = Scene::new();

let mut sprite = Sprite::new();
sprite.set_shape(Box::new(200.0, 20.0));
sprite.set_element_type(ElementType::Color(GREEN));

let mut node = Node::new("Foo".to_string());
node.add_component(sprite);
node.set_is_static(true);
scene.add_node(node);

game.load_scene(scene);

let _ = game.run_without_callback();
```

> [!CAUTION]
> The use of `Box::new` here is Vyxen's `vyxen::geometry::Box` type, not the standard library's `std::boxed::Box` type. If you are using the standard library's box type in the same file, instead import it one of them with an alias. For example: `use std::boxed::Box as StdBox;`.

If you would like a callback, instead use:

```rust, no_run
use vyxen::prelude::*;

let mut game = Game::new();
let mut scene = Scene::new();

let mut sprite = Sprite::new();
sprite.set_shape(Box::new(200.0, 20.0));
sprite.set_element_type(ElementType::Color(GREEN));

let mut node = Node::new("Foo".to_string());
node.add_component(sprite);
node.set_is_static(true);
scene.add_node(node);

game.load_scene(scene);

let _ = game.run(|_game, _event_loop, _window_event| {
    // Callback here
});
```

After you'll get this window:

<img width="50%" src="https://raw.githubusercontent.com/GlowyDeveloper/Vyxen/refs/heads/master/docs/Example-1.png">

Please note that,
 - (0, 0) is the center of the window
 - The position of the node is in the center of the node.

## Window Config

You can change many things by a single type.

```rust, no_run
use vyxen::prelude::*;

let mut game = Game::new();

let mut config = WindowConfig::new();
config.set_title("Hello".to_string());
config.set_max_size(Vector2 { x: 400.0, y: 400.0 });
config.set_min_size(Vector2 { x: 200.0, y: 200.0 });
config.set_size(Vector2 { x: 300.0, y: 300.0 });
config.set_background_color(LIGHT_BLUE);

game.set_config(config);

let _ = game.run_without_callback();
```

Once that is added, the window is changed to this:

<img width="30%" src="https://raw.githubusercontent.com/GlowyDeveloper/Vyxen/refs/heads/master/docs/Example-2.png">

## Inputs

### Keyboard

There's two methods of getting keyboard inputs:

```rust, no_run
use vyxen::prelude::*;

let mut game = Game::new();

let _ = game.run(|game, _event_loop, _window_event| {
    if game.is_just_pressed(KeyCode::KeyW) {
        println!("W was just pressed!");
    }
    if game.is_just_released(KeyCode::KeyW) {
        println!("W was just released!");
    }
    if game.is_held(KeyCode::KeyW) {
        println!("W is being held!");
    } else {
        println!("W is not being held!");
    }
});
```

But if you are using scripts, you must use `Context`.

```rust
use vyxen::prelude::*;

let mut node = Node::new("Foo".to_string());
node.set_physics_process(move |node, _, dt, ctx| {
    if ctx.is_held(KeyCode::KeyW) {
        println!("W is held!");
    }
    if ctx.is_held(KeyCode::KeyA) {
        println!("A is held!");
    }
    if ctx.is_held(KeyCode::KeyS) {
        println!("S is held!");
    }
    if ctx.is_held(KeyCode::KeyD) {
        println!("D is held!");
    }
});
```

### Mouse

Mouse inputs can only come from events.

```rust, no_run
use vyxen::prelude::*;

let mut game = Game::new();

let _ = game.run(|_, event, _| {
    match event {
        Event::MouseInput(button, state, position) => {
            println!("Button: {:?}", button);
            println!("Position: {:?}", position);
            println!("State: {:?}", state);
        },
        _ => {}
    }
});
```