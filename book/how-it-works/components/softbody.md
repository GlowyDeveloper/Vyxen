# SoftBody

A `SoftBody` gives a node a soft body. Each point or vertex moves freely within the soft body.

> [!NOTE]
> A `SoftBody` moves the node, but [`Collider`](./collider.html) creates a hitbox. For a `SoftBody` to collide, you must also add a [`Collider`](./collider.html).

## Usage

```rust
let shape = Circle::new(10.0);
let softbody = SoftBody::new(2.0, 0.7, shape, 0.3, 0.5);
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
node.add_component(softbody);
```

## Methods

If you require examples with methods, check the [documentation](https://docs.rs/vyxen/latest/vyxen/physics2d/struct.SoftBody.html).

### new

A constructor for a soft body.

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

### new_with_points

Creates a new `SoftBody` with the set amount of points.

Only intended for use with circles.

```rust
pub fn new_with_points<T>(
    density: f32,
    restitution: f32,
    shape: T,
    static_friction: f32,
    dynamic_friction: f32,
    points_count: u32,
) -> Self
where
    T: Shape,
{
```

### solve_springs

Solves the spring forces between the points of the soft body.

```rust
pub(crate) fn solve_springs(&mut self, dt: f32) {
```

### get_points

A getter for the points of the soft body.

If you want the mutable version, refer to `get_points_mut()`

```rust
pub fn get_points(&self) -> &Vec<PointMass> {
```

### get_points_mut

A getter for the points of the soft body mutably.

```rust
pub fn get_points_mut(&mut self) -> &mut Vec<PointMass> {
```

### get_springs

A getter for the springs of the soft body.

```rust
pub fn get_springs(&self) -> &Vec<Spring> {
```

### get_density

A getter for the density of the soft body.

```rust
pub fn get_density(&self) -> f32 {
```

### get_mass

A getter for the mass of the soft body.

```rust
pub fn get_mass(&self) -> f32 {
```

### get_inverse_mass

A getter for the inverted mass of the soft body.

```rust
pub fn get_inverse_mass(&self) -> f32 {
```

### get_restitution

A getter for the restitution of the soft body.

```rust
pub fn get_restitution(&self) -> f32 {
```

### get_area

A getter for the area of the soft body.

```rust
pub fn get_area(&self) -> f32 {
```

### get_inertia

A getter for the rotational inertia of the soft body.

```rust
pub fn get_inertia(&self) -> f32 {
```

### get_inverse_inertia

A getter for the inverted rotational inertia of the soft body.

```rust
pub fn get_inverse_inertia(&self) -> f32 {
```

### get_static_friction

A getter for the static friction of the soft body.

```rust
pub fn get_static_friction(&self) -> f32 {
```

### get_dynamic_friction

A getter for the dynamic friction of the soft body.

```rust
pub fn get_dynamic_friction(&self) -> f32 {
```

## Fields

```rust
pub struct SoftBody {
    density: f32,
    mass: f32,
    inverse_mass: f32,
    restitution: f32,
    area: f32,

    inertia: f32,
    inverse_inertia: f32,

    static_friction: f32,
    dynamic_friction: f32,

    original_points: Vec<Vector2>,
    points: Vec<PointMass>,
    springs: Vec<Spring>,
}
```

- `density`: The density of the soft body.
- `mass`: The mass of the soft body.
- `inverse_mass`: The inverted mass of the soft body, 1 divided by the mass.
- `restitution`: The restitution of the soft body.
- `area`: The area of the soft body.
- `inertia`: The inertia of the soft body.
- `inverse_inertia`: The inverted inertia of the soft body, 1 divided by the inertia.
- `static_friction`: The static friction of the soft body.
- `dynamic_friction`: The dynamic friction of the soft body.
- `original_points`: The positions of the points when theh softbody was first created.
- `points`: The current points of the soft body.
- `springs`: The springs of the soft body.
