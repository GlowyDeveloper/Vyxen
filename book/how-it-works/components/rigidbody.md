# RigidBody

A `RigidBody` gives a node a rigid body.

> [!NOTE]
> A `RigidBody` moves the node, but [`Collider`](./collider.html) creates a hitbox. For a `RigidBody` to collide, you must also add a [`Collider`](./collider.html).

## Usage

```rust
let shape = Circle::new(10.0);
let rigidbody = RigidBody::new(2.0, 0.7, shape, 0.3, 0.5);
```

The `new` function takes in multiple fields:
 - density
 - restitution
 - shape
   - `Circle`
   - `Box`
   - `Polygon`
 - static_friction
 - dynamic_friction

Then you attach it to the node using `add_component`:

```rust
let node = Node::new("Node");
node.add_component(rigidbody);
```

## Methods

If you require examples with methods, check the [documentation](https://docs.rs/vyxen/latest/vyxen/physics2d/struct.RigidBody.html).

### new

A constructor for a rigid body.

```rust
pub fn new<T>(
    density: f32,
    restitution: f32,
    shape: T,
    static_friction: f32,
    dynamic_friction: f32,
) -> Self
where
    T: Shape,
{
```

### get_density

A getter for the density of the rigid body.

```rust
pub fn get_density(&self) -> f32 {
```

### get_mass

A getter for the mass of the rigid body.

```rust
pub fn get_mass(&self) -> f32 {
```

### get_inverse_mass

A getter for the inverted mass of the rigid body.

```rust
pub fn get_inverse_mass(&self) -> f32 {
```

### get_restitution

A getter for the restitution of the rigid body.

```rust
pub fn get_restitution(&self) -> f32 {
```

### get_area

A getter for the area of the rigid body.

```rust
pub fn get_area(&self) -> f32 {
```

### get_inertia

A getter for the rotational inertia of the rigid body.

```rust
pub fn get_inertia(&self) -> f32 {
```

### get_inverse_inertia

A getter for the inverted rotational inertia of the rigid body.

```rust
pub fn get_inverse_inertia(&self) -> f32 {
```

### get_shape

A getter for the shape of the rigid body.

If you want the mutable version, refer to `get_shape_mut()`

```rust
pub fn get_shape(&self) -> ShapeType {
```

### get_shape_mut

A getter for the shape of the rigid body as a mutable reference.

```rust
pub fn get_shape(&mut self) -> &mut ShapeType {
```

### get_circle

Returns `None` if the shape is a box or polygon, return `Some(Circle)` if the shape is a circle

```rust
pub fn get_circle(&self) -> Option<Circle> {
```

### get_box

Returns `None` if the shape is a circle or polygon, return `Some(Box)` if the shape is a box

```rust
pub fn get_box(&self) -> Option<Box> {
```

### get_convex_polygon

Returns `None` if the shape is a circle or box, return `Some(Polygon)` if the shape is a **convex** polygon

```rust
pub fn get_convex_polygon(&self) -> Option<Polygon> {
```

### get_concave_polygon

Returns `None` if the shape is a circle or box or **convex** polygon, return `Some(Polygon)` if the shape is a **concave** polygon

```rust
pub fn get_concave_polygon(&self) -> Option<Vec<Polygon>> {
```

### get_static_friction

A getter for the static friction of the rigid body.

```rust
pub fn get_static_friction(&self) -> f32 {
```

### get_dynamic_friction

A getter for the dynamic friction of the rigid body.

```rust
pub fn get_dynamic_friction(&self) -> f32 {
```

## Fields

```rust
pub struct RigidBody {
    density: f32,
    mass: f32,
    inverse_mass: f32,
    restitution: f32,
    area: f32,

    inertia: f32,
    inverse_inertia: f32,

    static_friction: f32,
    dynamic_friction: f32,

    shape: ShapeType,
}
```

 - density: The density of the rigid body.
 - mass: The mass of the rigid body.
 - inverse_mass: The inverse mass of the rigid body, 1 divided by the mass.
 - restitution: The restitution of the rigid body.
 - area: The area of the rigid body.
 - inertia: The inertia of the rigid body.
 - inverse_inertia: The inverse inertia of the rigid body, 1 divided by the inertia.
 - static_friction: The static friction of the rigid body.
 - dynamic_friction: The dynamic friction of the rigid body.
 - shape: The shape of the rigid body. It can be a circle, box, polygon, or concave polygon.
