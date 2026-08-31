# Render

The render function is called every frame.

It's used to draw the game to the screen.

## Getting delta and frame time

```rust
let now = Instant::now();
let dt = (now - self.last_frame_instant).as_secs_f32();
self.last_frame_instant = now;

self.frame_accum_time += dt;
self.frame_accum_count += 1;

if self.frame_accum_time >= 0.5 {
    self.fps = self.frame_accum_count as f32 / self.frame_accum_time;
    self.frame_accum_time = 0.0;
    self.frame_accum_count = 0;
}
```

It also calls `self.window.request_redraw()`, so rendering keeps looping.

If the surface hasn't been configured yet, it waits until the surface is configured.

```rust
if !self.is_surface_configured {
    return Ok(());
}
```

## Getting the surface texture

Then it asks the surface for the current texture to draw into.

```rust
let output = match self.surface.get_current_texture() {
    wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
    wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => surface_texture,
    wgpu::CurrentSurfaceTexture::Timeout
    | wgpu::CurrentSurfaceTexture::Occluded
    | wgpu::CurrentSurfaceTexture::Validation => {
        return Ok(());
    }
    wgpu::CurrentSurfaceTexture::Outdated => {
        self.surface.configure(&self.device, &self.config);
        return Ok(());
    }
    wgpu::CurrentSurfaceTexture::Lost => {
        anyhow::bail!("Lost device");
    }
};
```

Then it creates a `TextureView` from the texture, and a `CommandEncoder` to record the frame's commands into.

```rust
let view = output
    .texture
    .create_view(&wgpu::TextureViewDescriptor::default());

let mut encoder = self
    .device
    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });
```

## Beginning the render pass

Then it opens a single render pass for the whole frame.

It sets the background to `WindowConfig.background_color`.

```rust
let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    label: Some("Render Pass"),
    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: &view,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color {
                r: self.custom_config.background_color.r() as f64,
                g: self.custom_config.background_color.g() as f64,
                b: self.custom_config.background_color.b() as f64,
                a: self.custom_config.background_color.a() as f64,
            }),
            store: wgpu::StoreOp::Store,
        },
        ..
    })],
    ..
});
```

## Drawing sprites

It binds `sprite_buffer`, the per-instance data written in [update](./update.html), to vertex slot 1. Then it loops through `sprite_render_order` and draws one sprite at a time.

### Drawing text

For `ElementType::Text`, it looks up the cached glyph atlas for that node and font size.

Then it draws with `world_pipeline_text`, binding the glyph atlas texture, the world camera, and a dynamic-offset slice into `text_uniform_buffer`.

```rust
render_pass.set_pipeline(&self.world_pipeline_text);
render_pass.set_bind_group(0, &gpu_texture.bind_group, &[]);
render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
render_pass.set_bind_group(
    2,
    &self.text_uniform_bind_group,
    &[slot * self.text_uniform_stride as u32],
);
render_pass.set_vertex_buffer(0, self.glyph_instance_buffer.slice(..));
render_pass.draw(0..6, *first..*first + *count);
```

### Other Types

It generates the sprite's geometry with `sprite_geometry`, then writes the vertices and indices into `vertex_buffer` and `index_buffer`.

It bails with an error if either buffer would overflow.

```rust
if vertex_offset + vertex_padded_len as u64 > MAX_SPRITE_VERTEX_BUFFER_SIZE {
    anyhow::bail!("Sprite vertex buffer overflow");
}
if index_offset + index_padded_len as u64 > MAX_SPRITE_INDEX_BUFFER_SIZE {
    anyhow::bail!("Sprite index buffer overflow");
}
```

#### Textures

Then it picks `world_pipeline_texture`, and binds the sprite's `GpuTexture` and world camera. It sets the vertex and index buffer slices it wrote, and draws.

```rust
render_pass.set_pipeline(&self.world_pipeline_texture);
render_pass.set_bind_group(0, &gpu_texture.bind_group, &[]);
render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
```

#### Colors

Then it picks `world_pipeline_color`, and binds `color_bind_group` and world camera. It sets the vertex and index buffer slices and draws.

```rust
render_pass.set_pipeline(&self.world_pipeline_color);
render_pass.set_bind_group(0, &self.color_bind_group, &[]);
render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
```

## Drawing UI elements

Once all world sprites are drawn, the same process repeats for `ui_render_order`

Text elements use `ui_pipeline_text` with the UI camera bind group. Textures use `ui_pipeline_texture` and color use `ui_pipeline_color` with the same UI camera bind group.

## Submitting the frame

Once the render pass ends, it submits the recorded commands to the queue and presents the frame.

```rust
self.queue.submit(std::iter::once(encoder.finish()));
self.queue.present(output);
```