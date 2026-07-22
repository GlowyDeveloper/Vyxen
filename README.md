<div align="center">

# Vyxen

**A Godot-style game engine library written in Rust**

</div>

**Vyxen isn't on <https://crates.io> yet!**

If you to install it into you're project, add this into the Cargo.toml:

```toml
vyxen = { git = "https://github.com/GlowyDeveloper/Vyxen/tree/master/crates/vyxen" }
```

## Creating a game

Vyxen has a root `game`.

```rust
use vyxen::prelude::*;

let game = Game::new();
```

## Creating and loading a scene.

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
|OpenGL    |✅    |✅           |2️⃣      |   |
|WebGPU    |       |             |         |✅|
|Tested    |✅    |✅           |         |   |

✅ = Works
1️⃣ = MoltenVK required
2️⃣ = ANGLE required

To render the scene:

```rust, no_run
use vyxen::prelude::*;

let mut game = Game::new();
let mut scene = Scene::new();

let mut sprite = Sprite::new();
sprite.set_shape(Box::new(20.0, 2.0));
sprite.set_draw_type(DrawType::Color(GREEN));

let mut node = Node::new("Foo".to_string());
node.add_component(sprite);
node.set_is_static(true);
scene.add_node(node);

game.load_scene(scene);

let _ = game.run_without_callback();
```

If you would like a callback, instead use:

```rust, no_run
use vyxen::prelude::*;

let mut game = Game::new();
let mut scene = Scene::new();

let mut sprite = Sprite::new();
sprite.set_shape(Box::new(20.0, 2.0));
sprite.set_draw_type(DrawType::Color(GREEN));

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

<img width="50%" src="https://raw.githubusercontent.com/GlowyDeveloper/Vyxen/refs/heads/master/docs/Example-image-1.png">

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

<img width="30%" src="https://raw.githubusercontent.com/GlowyDeveloper/Vyxen/refs/heads/master/docs/Example-image-2.png">

## License

This project is licensed under either of

 - Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/license/mit>)

at your option.