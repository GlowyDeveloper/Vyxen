use std::{collections::HashMap, sync::Arc};
use vyxen_math::Vector2;
use vyxen_resource::{GlyphMap, Texture};
use vyxen_ui::{UiElement, UiType};
use wgpu::util::DeviceExt as _;
use winit::window::Window;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(target_arch = "wasm32")]
use web_time::Instant;

use crate::{
    Camera, Sprite, WindowConfig,
    backend::{
        CameraUniform, GlyphRaw, GpuTexture, MAX_GLYPH_INSTANCES, MAX_SPRITE_INDEX_BUFFER_SIZE,
        MAX_SPRITE_VERTEX_BUFFER_SIZE, MAX_SPRITES, MAX_UI_ELEMENTS, SpriteRaw, UiCameraUniform,
        UiRaw, Vertex,
        shape_geometry::{sprite_geometry, text_geometry},
    },
};

/// The main state struct for the renderer backend.
pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    window: Arc<Window>,
    world_pipeline_texture: wgpu::RenderPipeline,
    world_pipeline_color: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    empty_bind_group: wgpu::BindGroup,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    texture_cache: HashMap<u64, GpuTexture>,
    sprites: Vec<Sprite>,
    raw_sprites: Vec<SpriteRaw>,
    sprite_buffer: wgpu::Buffer,
    ui_elements: Vec<UiElement>,
    ui_element_buffer: wgpu::Buffer,
    raw_ui_elements: Vec<UiRaw>,
    ui_pipeline_texture: wgpu::RenderPipeline,
    ui_camera_uniform: UiCameraUniform,
    ui_camera_buffer: wgpu::Buffer,
    ui_camera_bind_group: wgpu::BindGroup,
    ui_pipeline_text: wgpu::RenderPipeline,
    glyph_instance_buffer: wgpu::Buffer,
    text_uniform_buffer: wgpu::Buffer,
    text_uniform_bind_group: wgpu::BindGroup,
    text_uniform_stride: u64,
    raw_glyph_instances: Vec<GlyphRaw>,
    text_draw_ranges: Vec<Option<(u32, u32)>>,
    glyph_atlas_cache: HashMap<(u64, u32, String), (GpuTexture, GlyphMap)>,
    custom_config: WindowConfig,
    last_frame_instant: Instant,
    fps: f32,
    frame_accum_time: f32,
    frame_accum_count: u32,
}

impl State {
    /// Creates a new state.
    pub async fn new(window: Arc<Window>, custom_config: WindowConfig) -> anyhow::Result<State> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: custom_config.render_mode.into(),
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

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
        };

        let camera = Camera {
            position: Vector2 { x: 0.0, y: 0.0 },
            rotation: 0.0,
            zoom: 1.0,
            height: config.height.max(1) as f32,
            width: config.width.max(1) as f32,
        };

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

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

        let texture_shader =
            device.create_shader_module(wgpu::include_wgsl!("../../shaders/texture.wgsl"));

        let color_shader =
            device.create_shader_module(wgpu::include_wgsl!("../../shaders/color.wgsl"));

        let ui_texture_shader =
            device.create_shader_module(wgpu::include_wgsl!("../../shaders/ui_texture.wgsl"));

        let ui_text_shader =
            device.create_shader_module(wgpu::include_wgsl!("../../shaders/ui_text.wgsl"));

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

        let empty_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[],
                label: Some("empty_bind_group_layout"),
            });

        let empty_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &empty_bind_group_layout,
            entries: &[],
            label: Some("empty_bind_group"),
        });

        let texture_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Texture World Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
                immediate_size: 0,
            });

        let color_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Color World Pipeline Layout"),
                bind_group_layouts: &[&empty_bind_group_layout, &camera_bind_group_layout],
                immediate_size: 0,
            });

        let world_pipeline_texture =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("World Pipeline Texture"),
                layout: Some(&texture_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &texture_shader,
                    entry_point: Some("vs_main"),
                    buffers: &[Vertex::desc(), SpriteRaw::desc()],
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

        let world_pipeline_color = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("World Pipeline Color"),
            layout: Some(&color_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &color_shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc(), SpriteRaw::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &color_shader,
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

        let mut ui_camera_uniform = UiCameraUniform::new();
        ui_camera_uniform.update_view_proj(&camera);

        let ui_camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
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
            label: Some("camera_bind_group"),
        });

        let ui_texture_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Texture Ui Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &ui_camera_bind_group_layout],
                immediate_size: 0,
            });

        let ui_pipeline_texture = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Ui Pipeline Texture"),
            layout: Some(&ui_texture_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &ui_texture_shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc(), UiRaw::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &ui_texture_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
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

        let text_uniform_alignment = device.limits().min_uniform_buffer_offset_alignment as u64;
        let text_uniform_stride = (std::mem::size_of::<UiRaw>() as u64 + text_uniform_alignment
            - 1)
            / text_uniform_alignment
            * text_uniform_alignment;

        let text_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Text Uniform Buffer"),
            size: text_uniform_stride * MAX_UI_ELEMENTS as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

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

        let glyph_instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Glyph Instance Buffer"),
            size: (std::mem::size_of::<GlyphRaw>() * MAX_GLYPH_INSTANCES) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let ui_text_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Text Ui Pipeline Layout"),
                bind_group_layouts: &[
                    &texture_bind_group_layout,
                    &ui_camera_bind_group_layout,
                    &text_uniform_bind_group_layout,
                ],
                immediate_size: 0,
            });

        let ui_pipeline_text = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Ui Pipeline Text"),
            layout: Some(&ui_text_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &ui_text_shader,
                entry_point: Some("vs_main"),
                buffers: &[GlyphRaw::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &ui_text_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
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
            empty_bind_group,
            texture_bind_group_layout,
            texture_cache: HashMap::new(),
            sprites: Vec::new(),
            raw_sprites: Vec::new(),
            sprite_buffer,
            ui_elements: Vec::new(),
            raw_ui_elements: Vec::new(),
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
            raw_glyph_instances: Vec::new(),
            glyph_atlas_cache: HashMap::new(),
            text_draw_ranges: Vec::new(),
            custom_config,
            last_frame_instant: Instant::now(),
            fps: 0.0,
            frame_accum_time: 0.0,
            frame_accum_count: 0,
        })
    }

    /// Resizes the renderer to the given width and height.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = if cfg!(target_arch = "wasm32") {
                width.min(2048)
            } else {
                width
            };
            self.config.height = if cfg!(target_arch = "wasm32") {
                height.min(2048)
            } else {
                height
            };

            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;

            self.camera.set_width(self.config.width as f32);
            self.camera.set_height(self.config.height as f32);
        }
    }

    /// Updates the state of the renderer.
    pub fn update(&mut self) {
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

        if !self.raw_sprites.is_empty() {
            self.queue.write_buffer(
                &self.sprite_buffer,
                0,
                bytemuck::cast_slice(&self.raw_sprites),
            );
        }

        if !self.raw_ui_elements.is_empty() {
            self.queue.write_buffer(
                &self.ui_element_buffer,
                0,
                bytemuck::cast_slice(&self.raw_ui_elements),
            );
        }

        for sprite in &self.sprites {
            if let crate::DrawType::Texture(texture) = &sprite.get_draw_type() {
                if !self.texture_cache.contains_key(&texture.get_id()) {
                    let gpu_tex = GpuTexture::from_image(
                        &self.device,
                        &self.queue,
                        &self.texture_bind_group_layout,
                        texture,
                    )
                    .expect("Failed to create GpuTexture");

                    self.texture_cache.insert(texture.get_id(), gpu_tex);
                }
            }
        }

        for ui in &self.ui_elements {
            match ui.get_ui_type() {
                UiType::Image(texture) => {
                    if !self.texture_cache.contains_key(&texture.get_id()) {
                        let gpu_tex = GpuTexture::from_image(
                            &self.device,
                            &self.queue,
                            &self.texture_bind_group_layout,
                            texture,
                        )
                        .expect("Failed to create GpuTexture");

                        self.texture_cache.insert(texture.get_id(), gpu_tex);
                    }
                }
                UiType::Text(text) => {
                    let key = (
                        text.font().id(),
                        text.size() as u32,
                        text.text().to_string(),
                    );
                    if !self.glyph_atlas_cache.contains_key(&key) {
                        let glyph_map = text
                            .font()
                            .generate_glyph_map(text.text().to_string(), text.size())
                            .expect("Failed to generate glyph map");

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
                        )
                        .expect("Failed to create GpuTexture");

                        self.glyph_atlas_cache.insert(key, (gpu_tex, glyph_map));
                    }
                }
                _ => {}
            }
        }

        self.raw_glyph_instances.clear();
        self.text_draw_ranges.clear();

        for ui in &self.ui_elements {
            match ui.get_ui_type() {
                UiType::Text(text) => {
                    let key = (
                        text.font().id(),
                        text.size() as u32,
                        text.text().to_string(),
                    );
                    let (_, glyph_map) = self.glyph_atlas_cache.get(&key).expect(&format!(
                        "glyph atlas has not been built for: {}",
                        text.font().id()
                    ));

                    let glyphs = text_geometry(glyph_map, text.text());
                    let first = self.raw_glyph_instances.len() as u32;
                    let count = glyphs.len() as u32;
                    self.raw_glyph_instances.extend(glyphs);
                    self.text_draw_ranges.push(Some((first, count)));
                }
                _ => self.text_draw_ranges.push(None),
            }
        }

        if !self.raw_glyph_instances.is_empty() {
            self.queue.write_buffer(
                &self.glyph_instance_buffer,
                0,
                bytemuck::cast_slice(&self.raw_glyph_instances),
            );
        }

        let mut padded =
            vec![0u8; (self.text_uniform_stride as usize) * self.raw_ui_elements.len().max(1)];
        for (i, raw) in self.raw_ui_elements.iter().enumerate() {
            let bytes = bytemuck::bytes_of(raw);
            let offset = i * self.text_uniform_stride as usize;
            padded[offset..offset + bytes.len()].copy_from_slice(bytes);
        }
        self.queue
            .write_buffer(&self.text_uniform_buffer, 0, &padded);
    }

    /// Renders the scene to the screen.
    pub fn render(&mut self) -> anyhow::Result<()> {
        self.window.request_redraw();

        let now = Instant::now();
        let dt = (now - self.last_frame_instant).as_secs_f32();
        self.last_frame_instant = now;

        self.frame_accum_time += dt;
        self.frame_accum_count += 1;

        if self.frame_accum_time >= 1.0 {
            self.fps = self.frame_accum_count as f32 / self.frame_accum_time;
            self.frame_accum_time = 0.0;
            self.frame_accum_count = 0;
        }

        if !self.is_surface_configured {
            return Ok(());
        }

        let output = match self.surface.get_current_texture() {
            Err(wgpu::SurfaceError::Outdated) => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            Err(wgpu::SurfaceError::Lost) => {
                anyhow::bail!("Lost device");
            }
            Err(wgpu::SurfaceError::Timeout) => {
                return Ok(());
            }

            Err(e) => {
                anyhow::bail!(e)
            }

            Ok(surface) => {
                if surface.suboptimal {
                    self.surface.configure(&self.device, &self.config);
                }
                surface
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: self.custom_config.background_color.r() as f64,
                            g: self.custom_config.background_color.g() as f64,
                            b: self.custom_config.background_color.b() as f64,
                            a: self.custom_config.background_color.a() as f64,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            render_pass.set_vertex_buffer(1, self.sprite_buffer.slice(..));

            let mut vertex_offset: u64 = 0;
            let mut index_offset: u64 = 0;

            for (sprite_index, sprite) in self.sprites.iter().enumerate() {
                if let crate::DrawType::None = sprite.draw_type {
                    continue;
                }

                let (vertices, indices) = sprite_geometry(sprite.get_vertices());
                let vertex_bytes = bytemuck::cast_slice(&vertices);
                let index_bytes = bytemuck::cast_slice(&indices);

                if vertex_offset + vertex_bytes.len() as u64 > MAX_SPRITE_VERTEX_BUFFER_SIZE {
                    anyhow::bail!("Sprite vertex buffer overflow");
                }
                if index_offset + index_bytes.len() as u64 > MAX_SPRITE_INDEX_BUFFER_SIZE {
                    anyhow::bail!("Sprite index buffer overflow");
                }

                self.queue
                    .write_buffer(&self.vertex_buffer, vertex_offset, vertex_bytes);
                self.queue
                    .write_buffer(&self.index_buffer, index_offset, index_bytes);

                match &sprite.draw_type {
                    crate::DrawType::Texture(texture) => {
                        let id = texture.get_id();
                        let gpu_texture = match self.texture_cache.get(&id) {
                            Some(g) => g,
                            None => {
                                anyhow::bail!("GpuTexture not found in cache for id: {}", id)
                            }
                        };

                        render_pass.set_pipeline(&self.world_pipeline_texture);
                        render_pass.set_bind_group(0, &gpu_texture.bind_group, &[]);
                        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
                    }
                    crate::DrawType::Color(_) => {
                        render_pass.set_pipeline(&self.world_pipeline_color);
                        render_pass.set_bind_group(0, &self.empty_bind_group, &[]);
                        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
                    }
                    crate::DrawType::None => continue,
                }

                render_pass.set_vertex_buffer(
                    0,
                    self.vertex_buffer
                        .slice(vertex_offset..vertex_offset + vertex_bytes.len() as u64),
                );
                render_pass.set_index_buffer(
                    self.index_buffer
                        .slice(index_offset..index_offset + index_bytes.len() as u64),
                    wgpu::IndexFormat::Uint16,
                );
                render_pass.draw_indexed(
                    0..indices.len() as u32,
                    0,
                    sprite_index as u32..sprite_index as u32 + 1,
                );

                vertex_offset += vertex_bytes.len() as u64;
                index_offset += index_bytes.len() as u64;
            }

            vertex_offset = 0;
            index_offset = 0;

            render_pass.set_vertex_buffer(1, self.ui_element_buffer.slice(..));

            for (ui_index, ui_element) in self.ui_elements.iter().enumerate() {
                if let UiType::Button = ui_element.get_ui_type() {
                    continue;
                }

                match ui_element.get_ui_type() {
                    UiType::Image(texture) => {
                        let id = texture.get_id();
                        let gpu_texture = self.texture_cache.get(&id).ok_or_else(|| {
                            anyhow::anyhow!("GpuTexture not found in cache for id: {}", id)
                        })?;

                        let (vertices, indices) = sprite_geometry(ui_element.get_vertices());
                        let vertex_bytes = bytemuck::cast_slice(&vertices);
                        let index_bytes = bytemuck::cast_slice(&indices);

                        if vertex_offset + vertex_bytes.len() as u64 > MAX_SPRITE_VERTEX_BUFFER_SIZE
                        {
                            anyhow::bail!("Sprite vertex buffer overflow");
                        }
                        if index_offset + index_bytes.len() as u64 > MAX_SPRITE_INDEX_BUFFER_SIZE {
                            anyhow::bail!("Sprite index buffer overflow");
                        }

                        self.queue
                            .write_buffer(&self.vertex_buffer, vertex_offset, vertex_bytes);
                        self.queue
                            .write_buffer(&self.index_buffer, index_offset, index_bytes);

                        render_pass.set_pipeline(&self.ui_pipeline_texture);
                        render_pass.set_bind_group(0, &gpu_texture.bind_group, &[]);
                        render_pass.set_bind_group(1, &self.ui_camera_bind_group, &[]);
                        render_pass.set_vertex_buffer(
                            0,
                            self.vertex_buffer
                                .slice(vertex_offset..vertex_offset + vertex_bytes.len() as u64),
                        );
                        render_pass.set_index_buffer(
                            self.index_buffer
                                .slice(index_offset..index_offset + index_bytes.len() as u64),
                            wgpu::IndexFormat::Uint16,
                        );
                        render_pass.draw_indexed(
                            0..indices.len() as u32,
                            0,
                            ui_index as u32..ui_index as u32 + 1,
                        );

                        vertex_offset += vertex_bytes.len() as u64;
                        index_offset += index_bytes.len() as u64;
                    }
                    UiType::Text(text) => {
                        let key = (
                            text.font().id(),
                            text.size() as u32,
                            text.text().to_string(),
                        );
                        let (gpu_texture, _glyph_map) =
                            self.glyph_atlas_cache.get(&key).ok_or_else(|| {
                                anyhow::anyhow!(
                                    "Glyph atlas not found for font: {}",
                                    text.font().id()
                                )
                            })?;

                        let Some((first, count)) = self.text_draw_ranges[ui_index] else {
                            continue;
                        };
                        if count == 0 {
                            continue;
                        }

                        render_pass.set_pipeline(&self.ui_pipeline_text);
                        render_pass.set_bind_group(0, &gpu_texture.bind_group, &[]);
                        render_pass.set_bind_group(1, &self.ui_camera_bind_group, &[]);
                        render_pass.set_bind_group(
                            2,
                            &self.text_uniform_bind_group,
                            &[(ui_index as u32) * self.text_uniform_stride as u32],
                        );
                        render_pass.set_vertex_buffer(0, self.glyph_instance_buffer.slice(..));
                        render_pass.draw(0..6, first..first + count);
                    }
                    _ => continue,
                }
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    /// Sets the sprites to be rendered.
    pub fn set_sprites(&mut self, sprites: Vec<Sprite>) {
        let mut sprites = sprites;
        sprites.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());
        self.sprites = sprites;

        self.raw_sprites = self.sprites.iter().map(Sprite::to_raw).collect();
    }

    /// Sets the UI elements to be rendered.
    pub fn set_ui_elements(&mut self, elements: Vec<UiElement>) {
        let mut elements = elements;
        elements.sort_by(|a, b| a.get_z().partial_cmp(&b.get_z()).unwrap());
        self.ui_elements = elements;

        self.raw_ui_elements = self.ui_elements.iter().map(UiRaw::gen_raw).collect();
    }

    /// Sets the camera to be used for rendering.
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
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
    }

    /// Returns a reference to the window.
    pub fn get_window(&self) -> &Arc<Window> {
        &self.window
    }

    /// Sets the window configuration.
    pub fn set_config(&mut self, config: WindowConfig) {
        self.custom_config = config;
    }

    /// Returns a reference to the camera.
    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    /// Returns a mutable reference to the camera.
    pub fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    /// Returns the current FPS.
    pub fn get_fps(&self) -> f32 {
        self.fps
    }
}
