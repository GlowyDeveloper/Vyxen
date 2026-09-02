# Collider

A `Collider` gives a hitbox to each node.

> [!NOTE]
> A `Collider` only creates a hitbox, it does not set how collisions are resolved, that's done by [`RigidBody`](./rigidbody.md) and [`SoftBody`](./softbody.md)

## Usage

```rust
let shape = Circle::new(10.0);
let collider = Collider::new(shape);
```

The `new` function takes in a `Shape`, these can be:
 - `Circle`
 - `Box`
 - `Polygon`

Then you attach it to the node using `add_component`:

```rust
let node = Node::new("Node");
node.add_component(collider);
```

## Methods

If you require examples with methods, check the [documentation](https://docs.rs/vyxen/latest/vyxen/struct.Collider.html).

### new

Creates a collider

```rust
pub fn new<T>(hitbox: T) -> Self
where
    T: Shape,
{
```

### get_hitbox

Gets the hitbox

For a mutable reference, refer to [`get_hitbox_mut`](#get_hitbox_mut)

```rust
pub fn get_hitbox(&self) -> &ShapeType {
```

### get_hitbox_mut

Gets the hitbox as a mutable reference

```rust
pub fn get_hitbox_mut(&mut self) -> &mut ShapeType {
```

### get_aabb

Generates an AABB of the Collider

```rust
pub fn get_aabb(&mut self, pos: Vector2, rot: f32) -> AABB {
```

### set_uninitilized

Sets aabb to uninitialized

```rust
pub(crate) fn set_uninitilized(&mut self) {
```

## Fields

```rust
pub struct Collider {
    hitbox: ShapeType,
    aabb: AABB,
    old_pos: Vector2,
    old_rot: f32,
    aabb_initialized: bool,
}
```

 - hitbox: The hitbox of the collider
 - aabb: The axis aligned bounding box of the collider
 - old_pos: The old position of the collider
 - old_rot: The old rotation of the collider
 - aabb_initialized: Whether the aabb has been initialized or not

The old_pos and old_rot are being saved because sometimes an AABB and fall into NAN and cause lag.
