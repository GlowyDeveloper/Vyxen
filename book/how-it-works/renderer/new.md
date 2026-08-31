# New

The new function is called once, when the renderer is created.

It's used to set up the GPU and build everything the renderer needs every frame.

## General initialization

First it creates the `wgpu::Instance`.

```rust
let mut instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
    backends: custom_config.render_mode.into(),
    flags: Default::default(),
    memory_budget_thresholds: Default::default(),
    backend_options: Default::default(),
    display: None,
});
```

On WASM, it creates `wgpu::Surface`, and if it fails, it instead uses WebGL.

```rust
#[cfg(target_arch = "wasm32")]
let surface = match instance.create_surface(window.clone()) {
    Ok(s) => s,
    Err(_e) => {
        instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::GL,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });
        instance.create_surface(window.clone())?
    }
};
```

If it's not WASM, it just creates a `wgpu::Surface` and returns the error.

```rust
#[cfg(not(target_arch = "wasm32"))]
let surface = instance.create_surface(window.clone())?;
```

Then it requests an `wgpu::Adapter`, `wgpu::Device` and `wgpu::Queue`.

```rust
let adapter = instance
    .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
        ..
    })
    .await?;

let (device, queue) = adapter
    .request_device(&wgpu::DeviceDescriptor {
        required_limits: if cfg!(target_arch = "wasm32") {
            wgpu::Limits::downlevel_webgl2_defaults()
        } else {
            wgpu::Limits::default()
        },
        ..
    })
    .await?;
```

It picks sRGB surface format if possible, and builds the `SurfaceConfiguration`.

```rust
let surface_caps = surface.get_capabilities(&adapter);
let surface_format = surface_caps
    .formats
    .iter()
    .find(|f| f.is_srgb())
    .copied()
    .unwrap_or(surface_caps.formats[0]);

let config = wgpu::SurfaceConfiguration {
    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    format: surface_format,
    width: size.width,
    height: size.height,
    present_mode: surface_caps.present_modes[0],
    alpha_mode: surface_caps.alpha_modes[0],
    view_formats: vec![],
    desired_maximum_frame_latency: 2,
    color_space: wgpu::SurfaceColorSpace::Auto,
};
```

## Creating the camera buffers

### World camera buffers

It starts by creating the `Camera` and `CameraUniform`.

```rust
let camera = Camera {
    position: Vector2 { x: 0.0, y: 0.0 },
    rotation: 0.0,
    zoom: 1.0,
    height: config.height.max(1) as f32,
    width: config.width.max(1) as f32,
};

let mut camera_uniform = CameraUniform::new();
camera_uniform.update_view_proj(&camera);
```

Then it creates the camera's `wgpu::Buffer` and `wgpu::BindGroupLayout` and `wgpu::BindGroup` for the camera.

```rust
let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    label: Some("Camera Buffer"),
    contents: bytemuck::cast_slice(&[camera_uniform]),
    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
});

let camera_bind_group_layout =
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: Some("camera_bind_group_layout"),
    });

let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
    layout: &camera_bind_group_layout,
    entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: camera_buffer.as_entire_binding(),
    }],
    label: Some("camera_bind_group"),
});
```

### Ui camera buffers

The ui camera only needs a `UiCameraUniform`

```rust
let mut ui_camera_uniform = UiCameraUniform::new();
ui_camera_uniform.update_view_proj(&camera);
```

Then like the world camera, it creates a `wgpu::Buffer` and `wgpu::BindGroupLayout` and `wgpu::BindGroup` for the ui camera.

```rust
let ui_camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    label: Some("Ui Camera Buffer"),
    contents: bytemuck::cast_slice(&[ui_camera_uniform]),
    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
});

let ui_camera_bind_group_layout =
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: Some("ui_camera_bind_group_layout"),
    });

let ui_camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
    layout: &ui_camera_bind_group_layout,
    entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: ui_camera_buffer.as_entire_binding(),
    }],
    label: Some("ui_camera_bind_group"),
});
```

## Creating buffers

It creates the sprite and ui elements buffers.

```rust
let sprite_buffer = device.create_buffer(&wgpu::BufferDescriptor {
    label: Some("Sprite Buffer"),
    size: (std::mem::size_of::<SpriteRaw>() * MAX_SPRITES) as wgpu::BufferAddress,
    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    mapped_at_creation: false,
});

let ui_element_buffer = device.create_buffer(&wgpu::BufferDescriptor {
    label: Some("UI Element Buffer"),
    size: (std::mem::size_of::<UiRaw>() * MAX_UI_ELEMENTS) as wgpu::BufferAddress,
    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    mapped_at_creation: false,
});
```

It also creates a glyph and text buffers.

```rust
let text_uniform_alignment = device.limits().min_uniform_buffer_offset_alignment as u64;
let text_uniform_stride = (std::mem::size_of::<UiRaw>() as u64)
    .div_ceil(text_uniform_alignment)
    * text_uniform_alignment;

let text_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
    label: Some("Text Uniform Buffer"),
    size: text_uniform_stride * MAX_UI_ELEMENTS as u64,
    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    mapped_at_creation: false,
});

let glyph_instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
    label: Some("Glyph Instance Buffer"),
    size: (std::mem::size_of::<GlyphRaw>() * MAX_GLYPH_INSTANCES) as wgpu::BufferAddress,
    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    mapped_at_creation: false,
});
```

## Creating the bind group layouts and bind groups

It creates the bind group layouts that the pipelines share.

### texture_bind_group_layout

`texture_bind_group_layout` is a texture and a sampler for `GpuTexture`.

```rust
let texture_bind_group_layout =
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
        label: Some("texture_bind_group_layout"),
    });
```

### color_bind_group_layout and color_bind_group

`color_bind_group_layout` and `color_bind_group` are for when coloring a sprite on ui elements.

```rust
let color_bind_group_layout =
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[],
        label: Some("color_bind_group_layout"),
    });

let color_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
    layout: &color_bind_group_layout,
    entries: &[],
    label: Some("color_bind_group"),
});
```

### text_uniform_bind_group_layout and text_uniform_bind_group

`text_uniform_bind_group_layout` and `text_uniform_bind_group` are for when putting text on a sprite or ui elements.

```rust
let text_uniform_bind_group_layout =
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: true,
                min_binding_size: std::num::NonZeroU64::new(
                    std::mem::size_of::<UiRaw>() as u64
                ),
            },
            count: None,
        }],
        label: Some("text_uniform_bind_group_layout"),
    });

let text_uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
    layout: &text_uniform_bind_group_layout,
    entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
            buffer: &text_uniform_buffer,
            offset: 0,
            size: std::num::NonZeroU64::new(std::mem::size_of::<UiRaw>() as u64),
        }),
    }],
    label: Some("text_uniform_bind_group"),
});
```

## Creating the pipelines

Then it creates the six render pipelines, they are
 - world_pipeline_texture
 - world_pipeline_color
 - world_pipeline_text
 - ui_pipeline_texture
 - ui_pipeline_color
 - ui_pipeline_text

For example, here's the `world_pipeline_texture`:

```rust
let texture_shader =
    device.create_shader_module(wgpu::include_wgsl!("shaders/texture.wgsl"));

let texture_pipeline_layout =
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Texture World Pipeline Layout"),
        bind_group_layouts: &[
            Some(&texture_bind_group_layout),
            Some(&camera_bind_group_layout),
        ],
        immediate_size: 0,
    });

let world_pipeline_texture =
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("World Pipeline Texture"),
        layout: Some(&texture_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &texture_shader,
            entry_point: Some("vs_main"),
            buffers: &[Some(Vertex::desc()), Some(SpriteRaw::desc())],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &texture_shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview_mask: None,
        cache: None,
    });
```

## Creating the vertex and index buffer

Next, we create the vertex and index buffers.

```rust
let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
    label: Some("Vertex Buffer"),
    size: MAX_SPRITE_VERTEX_BUFFER_SIZE,
    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    mapped_at_creation: false,
});

let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
    label: Some("Index Buffer"),
    size: MAX_SPRITE_INDEX_BUFFER_SIZE,
    usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
    mapped_at_creation: false,
});
```

## Returning the State

Finally, it assembles the `State` struct and returns it.

```rust
Ok(Self {
    surface,
    device,
    queue,
    config,
    is_surface_configured: false,
    window,
    world_pipeline_texture,
    world_pipeline_color,
    vertex_buffer,
    index_buffer,
    camera,
    camera_uniform,
    camera_buffer,
    camera_bind_group,
    color_bind_group,
    texture_bind_group_layout,
    texture_cache: HashMap::new(),
    sprite_buffer,
    ui_camera_uniform,
    ui_camera_buffer,
    ui_camera_bind_group,
    ui_pipeline_texture,
    ui_element_buffer,
    ui_pipeline_text,
    glyph_instance_buffer,
    text_uniform_buffer,
    text_uniform_bind_group,
    text_uniform_stride,
    ui_pipeline_color,
    custom_config,
    world_pipeline_text,
    last_frame_instant: Instant::now(),
    fps: 0.0,
    frame_accum_time: 0.0,
    frame_accum_count: 0,
    sprites: HashMap::new(),
    raw_sprites: HashMap::new(),
    ui_elements: HashMap::new(),
    raw_ui_elements: HashMap::new(),
    raw_glyph_instances: Vec::new(),
    glyph_atlas_cache: HashMap::new(),
    ui_render_order: Vec::new(),
    sprite_render_order: Vec::new(),
    text_uniform_index: HashMap::new(),
})
```
