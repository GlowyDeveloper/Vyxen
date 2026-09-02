# UiElement

A `UiElement` makes a node visible in the rendering process.

A `UiElement` stays in the same position at all times, and it does not move with the camera. If you want it too be dependant on the camera, look at [`Sprite`](./sprite.html).

## Usage

```rust
let mut ui = UiElement::new();
ui.set_element_type(ElementType::Color(Color::from_rgb(0.2, 0.5, 0.7)));
```

Then you attach it to the node using `add_component`:

```rust
let node = Node::new("Node");
node.add_component(ui);
```

## Methods

If you require examples with methods, check the [documentation](https://docs.rs/vyxen/latest/vyxen/struct.UiElement.html).

### new

Creates a new UI element.

```rust
pub fn new() -> Self {
```

### with_color

Short for `UiElement::new().set_element_type(ElementType::Color(..))`

```rust
pub fn with_color(color: Color) -> Self {
```

### with_font

Short for `UiElement::new().set_element_type(ElementType::Font(..))`

```rust
pub fn with_font(text: String, font: Font, size: f32) -> Self {
```

### with_texture

Short for `UiElement::new().set_element_type(ElementType::Texture(..))`

```rust
pub fn with_texture(texture: Texture) -> Self {
```

### set_element_type

Sets how this element should be rendered.

```rust
pub fn set_element_type(&mut self, element_type: ElementType) {
```

### get_element_type

Returns the current type of this element.

```rust
pub fn get_element_type(&self) -> &ElementType {
```

### set_z

Sets the z-coordinate of this element.

```rust
pub fn set_z(&mut self, z: f32) {
```

### get_z

Returns the current z-coordinate of this element.

```rust
pub fn get_z(&self) -> f32 {
```

### set_vertices

Sets the vertices of this element.

```rust
pub fn set_vertices(&mut self, vertices: Option<ShapeType>) {
```

### get_vertices

Returns the current vertices of this element.

```rust
pub fn get_vertices(&self) -> Option<&ShapeType> {
```

### set_shape

Sets the shape used to render this element.

```rust
pub fn set_shape<T>(&mut self, shape: T)
where
    T: Shape,
{
```

## Fields

```rust
pub struct UiElement {
    element_type: ElementType,
    vertices: Option<ShapeType>,
    z: f32,
}
```

- `element_type`: The type of element this sprite should be rendered as.
- `vertices`: The shape used to render this sprite.
- `z`: The z-index of this sprite.
