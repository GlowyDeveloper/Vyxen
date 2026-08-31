# Update

The update function is called every frame.

It's used to update the game state and write buffers.

First it updates the cameras and writes the camera buffers.

```rust
self.camera_uniform.update_view_proj(&self.camera);
self.queue.write_buffer(
    &self.camera_buffer,
    0,
    bytemuck::cast_slice(&[self.camera_uniform]),
);

self.ui_camera_uniform.update_view_proj(&self.camera);
self.queue.write_buffer(
    &self.ui_camera_buffer,
    0,
    bytemuck::cast_slice(&[self.ui_camera_uniform]),
);
```

Then, it loops through every node in the scene and caches them.

```rust
for (node_id, node) in nodes {
    if let Some(sprite) = node.get_component::<Sprite>() {
        match sprite.get_element_type() { .. }

        self.raw_sprites.insert(
            *node_id,
            SpriteRaw::gen_raw(sprite, node.get_position(), node.get_rotation()),
        );
        self.sprites.insert(*node_id, sprite.clone());
    }
    if let Some(ui) = node.get_component::<UiElement>() {
        match ui.get_element_type() { .. }

        self.raw_ui_elements.insert(
            *node_id,
            UiRaw::gen_raw(ui, node.get_position(), node.get_rotation()),
        );
        self.ui_elements.insert(*node_id, ui.clone());
    }
}
```

## Caching Textures

```rust
ElementType::Texture(texture) => {
    if !self.texture_cache.contains_key(node_id) {
        let gpu_tex = GpuTexture::from_image(
            &self.device,
            &self.queue,
            &self.texture_bind_group_layout,
            texture,
            *node_id,
        )
        .expect("Failed to create GpuTexture");

        self.texture_cache.insert(*node_id, gpu_tex);
    }
}
```

First, the function checks if the texture is already cached.

If the texture is not cached, it creates a `GpuTexture` from the image and inserts it into the cache.

## Caching Text

```rust
ElementType::Text(text) => {
    let key = (node.get_id(), text.get_size() as u32);
    let (glyph_map, gpu_tex) = if !self.glyph_atlas_cache.contains_key(&key) {
        ..
    } else {
        ..
    };

    ..
}
```

First, the function checks if the text is already cached.

If it is, then it gets the already cached `GlyphMap` and `GpuTexture`

```rust
let (glyph_map, gpu_tex, _, _) =
    self.glyph_atlas_cache.get(&key).unwrap().clone();
(glyph_map, gpu_tex)
```

If it's not, then it generates a new `GlyphMap`.

```rust
let glyph_map = text
    .get_font()
    .generate_glyph_map(text.get_size())
    .expect("Failed to generate glyph map");
```

Then it creates a new `GpuTexture`.

```rust
let atlas_texture = Texture::from_raw(
    Vector2 {
        x: glyph_map.atlas_width() as f32,
        y: glyph_map.atlas_height() as f32,
    },
    glyph_map.rgba().clone(),
);

let gpu_tex = GpuTexture::from_image(
    &self.device,
    &self.queue,
    &self.texture_bind_group_layout,
    &atlas_texture,
    *node_id,
)
.expect("Failed to create GpuTexture");

(glyph_map, gpu_tex)
```

If it's cached or not, it generates the locations of the rest of the glyphs.

```rust
let glyphs = text_geometry(&glyph_map, text.get_text());
let first = self.raw_glyph_instances.len() as u32;
let count = glyphs.len() as u32;

self.raw_glyph_instances.extend(glyphs);
self.glyph_atlas_cache
    .insert(key, (glyph_map, gpu_tex, first, count));
```

## Unloading unused textures and texts

```rust
self.sprites.retain(|id, _| nodes.contains_key(id));
self.raw_sprites.retain(|id, _| nodes.contains_key(id));
self.ui_elements.retain(|id, _| nodes.contains_key(id));
self.raw_ui_elements.retain(|id, _| nodes.contains_key(id));

self.texture_cache.retain(|id, _| nodes.contains_key(id));

self.glyph_atlas_cache
    .retain(|(node_id, _), _| nodes.contains_key(node_id));
```

It loops through all caches and checks if the coresponding id is in the scene

## Writing the buffers

Due to `HashMap`s not saving order, we need to generate the correct order.

```rust
let mut sprite_order: Vec<u64> = self.sprites.keys().copied().collect();
sprite_order.sort_by(|a, b| {
    let za = self.sprites[a].get_z();
    let zb = self.sprites[b].get_z();
    za.partial_cmp(&zb)
        .unwrap_or(std::cmp::Ordering::Equal)
        .then_with(|| a.cmp(b))
});
self.sprite_render_order = sprite_order;
```

And then it's written to the buffer in that order.

```rust
if !self.raw_sprites.is_empty() {
    let raw: Vec<SpriteRaw> = self
        .sprite_render_order
        .iter()
        .map(|id| self.raw_sprites[id])
        .collect();
    self.queue
        .write_buffer(&self.sprite_buffer, 0, bytemuck::cast_slice(&raw));
}
```
