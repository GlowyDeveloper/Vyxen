# Sprite

A `Sprite` makes a node visible in the rendering process.

A `Sprite` is dependent on the camera. If you want something to stay in the same spot when the camera moves, look at [`UiElement`](./uielement.html).

## Usage

```rust
let mut sprite = Sprite::new();
sprite.set_element_type(ElementType::Color(Color::from_rgb(0.2, 0.5, 0.7)));
```

Then you attach it to the node using `add_component`:

```rust
let node = Node::new("Node");
node.add_component(sprite);
```

## Methods

If you require examples with methods, check the [documentation](https://docs.rs/vyxen/latest/vyxen/struct.UiElement.html).

### new

Gives a sprite to render to the screen.

```rust
pub fn new() -> Self {
```

### with_color

Short for `Sprite::new().set_element_type(ElementType::Color(..))`

```rust
pub fn with_color(color: Color) -> Self {
```

### with_font

Short for `Sprite::new().set_element_type(ElementType::Font(..))`

```rust
pub fn with_font(text: String, font: Font, size: f32) -> Self {
```

### with_texture

Short for `Sprite::new().set_element_type(ElementType::Texture(..))`

```rust
pub fn with_texture(texture: Texture) -> Self {
```

### set_element_type

Sets how this sprite should be rendered.

```rust
pub fn set_element_type(&mut self, element_type: ElementType) {
```

### set_shape

Sets the shape used to render this sprite.

```rust
pub fn set_shape<T>(&mut self, shape: T)
where
    T: Shape,
{
```

### get_vertices

Returns `Some` if the shape is assigned, `None` if not.

```rust
pub fn get_vertices(&self) -> Option<&ShapeType> {
```

### get_element_type

Returns the current element type of this sprite.

```rust
pub fn get_element_type(&self) -> &ElementType {
```

### set_z

Sets the z-index of this sprite.

```rust
pub fn set_z(&mut self, z: f32) {
```

### get_z

Returns the z-index of this sprite.

```rust
pub fn get_z(&self) -> f32 {
```

## Fields

```rust
pub struct Sprite {
    element_type: ElementType,
    vertices: Option<ShapeType>,
    z: f32,
}
```

- `element_type`: The type of element this sprite should be rendered as.
- `vertices`: The shape used to render this sprite.
- `z`: The z-index of this sprite.
