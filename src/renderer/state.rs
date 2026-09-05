use crate::{
    Camera, ElementType, Node, Sprite, Texture, UiElement, Vector2, WindowConfig,
    error::Error,
    renderer::{
        gpu_texture::GpuTexture,
        raws::{
            CameraUniform, GlyphRaw, MAX_GLYPH_INSTANCES, MAX_SPRITE_INDEX_BUFFER_SIZE,
            MAX_SPRITE_VERTEX_BUFFER_SIZE, MAX_SPRITES, MAX_UI_ELEMENTS, SpriteRaw,
            UiCameraUniform, UiRaw, Vertex,
        },
        shape_geometry::{sprite_geometry, text_geometry},
    },
    resource::font::GlyphMap,
};
use std::{collections::HashMap, sync::Arc};
use wgpu::util::DeviceExt as _;
use winit::window::Window;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(target_arch = "wasm32")]
use web_time::Instant;

pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    window: Arc<Window>,
    custom_config: WindowConfig,

    camera: Camera,
    camera_uniform: CameraUniform,
    ui_camera_uniform: UiCameraUniform,

    texture_bind_group_layout: wgpu::BindGroupLayout,

    color_bind_group: wgpu::BindGroup,
    camera_bind_group: wgpu::BindGroup,
    ui_camera_bind_group: wgpu::BindGroup,
    text_uniform_bind_group: wgpu::BindGroup,

    world_pipeline_texture: wgpu::RenderPipeline,
    world_pipeline_color: wgpu::RenderPipeline,
    world_pipeline_text: wgpu::RenderPipeline,
    ui_pipeline_texture: wgpu::RenderPipeline,
    ui_pipeline_text: wgpu::RenderPipeline,
    ui_pipeline_color: wgpu::RenderPipeline,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    camera_buffer: wgpu::Buffer,
    sprite_buffer: wgpu::Buffer,
    ui_element_buffer: wgpu::Buffer,
    ui_camera_buffer: wgpu::Buffer,
    glyph_instance_buffer: wgpu::Buffer,
    text_uniform_buffer: wgpu::Buffer,

    last_frame_instant: Instant,
    fps: f32,
    frame_accum_time: f32,
    frame_accum_count: u32,

    texture_cache: HashMap<u64, GpuTexture>,
    sprites: HashMap<u64, Sprite>,
    raw_sprites: HashMap<u64, SpriteRaw>,
    ui_elements: HashMap<u64, UiElement>,
    raw_ui_elements: HashMap<u64, UiRaw>,
    text_uniform_stride: u64,
    raw_glyph_instances: Vec<GlyphRaw>,
    glyph_atlas_cache: HashMap<(u64, u32), (GlyphMap, GpuTexture, u32, u32)>,
    ui_render_order: Vec<u64>,
    sprite_render_order: Vec<u64>,
    text_uniform_index: HashMap<u64, u32>,
}

impl State {
    pub async fn new(window: Arc<Window>, custom_config: WindowConfig) -> Result<State, Error> {
        let size = window.inner_size();

        #[allow(unused_mut)]
        let mut instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: custom_config.render_mode.into(),
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        #[cfg(target_arch = "wasm32")]
        let surface = match instance.create_surface(window.clone()) {
            Ok(s) => s,
            Err(e) => {
                log::warn!(
                    "Failed to create surface, attempting to use GL instead: {}",
                    e
                );
                instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
                    backends: wgpu::Backends::GL,
                    flags: Default::default(),
                    memory_budget_thresholds: Default::default(),
                    backend_options: Default::default(),
                    display: None,
                });
                instance
                    .create_surface(window.clone())
                    .map_err(|e| Error::SurfaceCreation(e.to_string()))?
            }
        };

        #[cfg(not(target_arch = "wasm32"))]
        let surface = instance
            .create_surface(window.clone())
            .map_err(|e| Error::SurfaceCreation(e.to_string()))?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                apply_limit_buckets: true,
            })
            .await
            .map_err(|e| Error::RequestingAdapter(e.to_string()))?;

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
            .await
            .map_err(|e| Error::RequestingDevice(e.to_string()))?;

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
            device.create_shader_module(wgpu::include_wgsl!("shaders/texture.wgsl"));

        let color_shader = device.create_shader_module(wgpu::include_wgsl!("shaders/color.wgsl"));

        let text_shader = device.create_shader_module(wgpu::include_wgsl!("shaders/text.wgsl"));

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

        let texture_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Texture World Pipeline Layout"),
                bind_group_layouts: &[
                    Some(&texture_bind_group_layout),
                    Some(&camera_bind_group_layout),
                ],
                immediate_size: 0,
            });

        let color_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Color World Pipeline Layout"),
                bind_group_layouts: &[
                    Some(&color_bind_group_layout),
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

        let world_pipeline_color = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("World Pipeline Color"),
            layout: Some(&color_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &color_shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(Vertex::desc()), Some(SpriteRaw::desc())],
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

        let world_text_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Text World Pipeline Layout"),
                bind_group_layouts: &[
                    Some(&texture_bind_group_layout),
                    Some(&camera_bind_group_layout),
                    Some(&text_uniform_bind_group_layout),
                ],
                immediate_size: 0,
            });

        let world_pipeline_text = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("World Pipeline Text"),
            layout: Some(&world_text_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &text_shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(GlyphRaw::desc())],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &text_shader,
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

        let mut ui_camera_uniform = UiCameraUniform::new();
        ui_camera_uniform.update_view_proj(&camera);

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

        let ui_texture_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Texture Ui Pipeline Layout"),
                bind_group_layouts: &[
                    Some(&texture_bind_group_layout),
                    Some(&ui_camera_bind_group_layout),
                ],
                immediate_size: 0,
            });

        let ui_pipeline_texture = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Ui Pipeline Texture"),
            layout: Some(&ui_texture_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &texture_shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(Vertex::desc()), Some(UiRaw::desc())],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &texture_shader,
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

        let ui_text_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Text Ui Pipeline Layout"),
                bind_group_layouts: &[
                    Some(&texture_bind_group_layout),
                    Some(&ui_camera_bind_group_layout),
                    Some(&text_uniform_bind_group_layout),
                ],
                immediate_size: 0,
            });

        let ui_pipeline_text = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Ui Pipeline Text"),
            layout: Some(&ui_text_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &text_shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(GlyphRaw::desc())],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &text_shader,
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

        let ui_color_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Ui Color Pipeline Layout"),
                bind_group_layouts: &[
                    Some(&color_bind_group_layout),
                    Some(&ui_camera_bind_group_layout),
                ],
                immediate_size: 0,
            });

        let ui_pipeline_color = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Ui Pipeline Color"),
            layout: Some(&ui_color_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &color_shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(Vertex::desc()), Some(UiRaw::desc())],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &color_shader,
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
    }

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

    pub fn update(&mut self, nodes: &HashMap<u64, Node>) -> Result<(), Error> {
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

        self.raw_glyph_instances.clear();

        for (node_id, node) in nodes {
            if let Some(sprite) = node.get_component::<Sprite>() {
                match sprite.get_element_type() {
                    ElementType::Texture(texture) => {
                        if !self.texture_cache.contains_key(node_id) {
                            let gpu_tex = GpuTexture::from_image(
                                &self.device,
                                &self.queue,
                                &self.texture_bind_group_layout,
                                texture,
                                *node_id,
                            );

                            self.texture_cache.insert(*node_id, gpu_tex);
                        }
                    }
                    ElementType::Text(text) => {
                        let key = (node.get_id(), text.get_size() as u32);
                        let (glyph_map, gpu_tex) = if !self.glyph_atlas_cache.contains_key(&key) {
                            let glyph_map = text.get_font().generate_glyph_map(text.get_size())?;

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
                            );

                            (glyph_map, gpu_tex)
                        } else {
                            let (glyph_map, gpu_tex, _, _) =
                                self.glyph_atlas_cache.get(&key).unwrap().clone();
                            (glyph_map, gpu_tex)
                        };

                        let glyphs = text_geometry(&glyph_map, text.get_text(), text.get_anchor());
                        let first = self.raw_glyph_instances.len() as u32;
                        let count = glyphs.len() as u32;

                        self.raw_glyph_instances.extend(glyphs);
                        self.glyph_atlas_cache
                            .insert(key, (glyph_map, gpu_tex, first, count));
                    }
                    _ => {}
                }

                self.raw_sprites.insert(
                    *node_id,
                    SpriteRaw::gen_raw(sprite, node.get_position(), node.get_rotation()),
                );
                self.sprites.insert(*node_id, sprite.clone());
            }
            if let Some(ui) = node.get_component::<UiElement>() {
                match ui.get_element_type() {
                    ElementType::Texture(texture) => {
                        if !self.texture_cache.contains_key(node_id) {
                            let gpu_tex = GpuTexture::from_image(
                                &self.device,
                                &self.queue,
                                &self.texture_bind_group_layout,
                                texture,
                                *node_id,
                            );

                            self.texture_cache.insert(*node_id, gpu_tex);
                        }
                    }
                    ElementType::Text(text) => {
                        let key = (node.get_id(), text.get_size() as u32);
                        let (glyph_map, gpu_tex) = if !self.glyph_atlas_cache.contains_key(&key) {
                            let glyph_map = text.get_font().generate_glyph_map(text.get_size())?;

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
                            );

                            (glyph_map, gpu_tex)
                        } else {
                            let (glyph_map, gpu_tex, _, _) =
                                self.glyph_atlas_cache.get(&key).unwrap().clone();
                            (glyph_map, gpu_tex)
                        };

                        let glyphs = text_geometry(&glyph_map, text.get_text(), text.get_anchor());
                        let first = self.raw_glyph_instances.len() as u32;
                        let count = glyphs.len() as u32;

                        self.raw_glyph_instances.extend(glyphs);
                        self.glyph_atlas_cache
                            .insert(key, (glyph_map, gpu_tex, first, count));
                    }
                    _ => {}
                }

                self.raw_ui_elements.insert(
                    *node_id,
                    UiRaw::gen_raw(ui, node.get_position(), node.get_rotation()),
                );
                self.ui_elements.insert(*node_id, ui.clone());
            }
        }

        self.sprites.retain(|id, _| nodes.contains_key(id));
        self.raw_sprites.retain(|id, _| nodes.contains_key(id));
        self.ui_elements.retain(|id, _| nodes.contains_key(id));
        self.raw_ui_elements.retain(|id, _| nodes.contains_key(id));

        self.texture_cache.retain(|id, _| nodes.contains_key(id));

        self.glyph_atlas_cache
            .retain(|(node_id, _), _| nodes.contains_key(node_id));

        let mut sprite_order: Vec<u64> = self.sprites.keys().copied().collect();
        sprite_order.sort_by(|a, b| {
            let za = self.sprites[a].get_z();
            let zb = self.sprites[b].get_z();
            za.partial_cmp(&zb)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.cmp(b))
        });
        self.sprite_render_order = sprite_order;

        if !self.raw_sprites.is_empty() {
            let raw: Vec<SpriteRaw> = self
                .sprite_render_order
                .iter()
                .map(|id| self.raw_sprites[id])
                .collect();
            self.queue
                .write_buffer(&self.sprite_buffer, 0, bytemuck::cast_slice(&raw));
        }

        let mut ui_order: Vec<u64> = self.ui_elements.keys().copied().collect();
        ui_order.sort_by(|a, b| {
            let za = self.ui_elements[a].get_z();
            let zb = self.ui_elements[b].get_z();
            za.partial_cmp(&zb)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.cmp(b))
        });
        self.ui_render_order = ui_order;

        if !self.raw_ui_elements.is_empty() {
            let raw: Vec<UiRaw> = self
                .ui_render_order
                .iter()
                .map(|id| self.raw_ui_elements[id])
                .collect();
            self.queue
                .write_buffer(&self.ui_element_buffer, 0, bytemuck::cast_slice(&raw));
        }

        if !self.raw_glyph_instances.is_empty() {
            self.queue.write_buffer(
                &self.glyph_instance_buffer,
                0,
                bytemuck::cast_slice(&self.raw_glyph_instances),
            );
        }

        let mut text_order: Vec<u64> = self
            .sprites
            .iter()
            .filter(|(_, s)| matches!(s.get_element_type(), ElementType::Text(_)))
            .map(|(id, _)| *id)
            .chain(
                self.ui_elements
                    .iter()
                    .filter(|(_, u)| matches!(u.get_element_type(), ElementType::Text(_)))
                    .map(|(id, _)| *id),
            )
            .collect();
        text_order.sort_unstable();

        self.text_uniform_index = text_order
            .iter()
            .enumerate()
            .map(|(i, id)| (*id, i as u32))
            .collect();

        let mut padded = vec![0u8; (self.text_uniform_stride as usize) * text_order.len().max(1)];
        for (i, id) in text_order.iter().enumerate() {
            let offset = i * self.text_uniform_stride as usize;
            if let Some(raw) = self.raw_ui_elements.get(id) {
                let bytes = bytemuck::bytes_of(raw);
                padded[offset..offset + bytes.len()].copy_from_slice(bytes);
            } else if let Some(raw) = self.raw_sprites.get(id) {
                let bytes = bytemuck::bytes_of(raw);
                padded[offset..offset + bytes.len()].copy_from_slice(bytes);
            }
        }
        self.queue
            .write_buffer(&self.text_uniform_buffer, 0, &padded);

        Ok(())
    }

    pub fn render(&mut self) -> Result<(), Error> {
        self.window.request_redraw();

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

        if !self.is_surface_configured {
            return Ok(());
        }

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
                return Err(Error::DeviceLost);
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

            for (sprite_index, id) in self.sprite_render_order.iter().enumerate() {
                let sprite = &self.sprites[id];
                match sprite.get_element_type() {
                    ElementType::None => continue,
                    ElementType::Text(text) => {
                        let (gpu_texture, first, count) =
                            match self.glyph_atlas_cache.get(&(*id, text.get_size() as u32)) {
                                Some((_, g, f, c)) => (g, f, c),
                                None => {
                                    log::error!("Failed to find glyph atlas for text: {:?}", text);
                                    continue;
                                }
                            };

                        if *count == 0 {
                            continue;
                        }

                        let slot = self.text_uniform_index[id];

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
                    }
                    _ => {
                        let (vertices, indices) = sprite_geometry(sprite.get_vertices());
                        let vertex_bytes = bytemuck::cast_slice(&vertices);
                        let index_bytes = bytemuck::cast_slice(&indices);

                        let vertex_padded_len = (vertex_bytes.len() + 3) & !3;
                        let index_padded_len = (index_bytes.len() + 3) & !3;

                        if vertex_offset + vertex_padded_len as u64 > MAX_SPRITE_VERTEX_BUFFER_SIZE
                        {
                            return Err(Error::IndexOverflow(
                                *id,
                                vertex_offset + vertex_padded_len as u64,
                                MAX_SPRITE_VERTEX_BUFFER_SIZE,
                            ));
                        }
                        if index_offset + index_padded_len as u64 > MAX_SPRITE_INDEX_BUFFER_SIZE {
                            return Err(Error::IndexOverflow(
                                *id,
                                index_offset + index_padded_len as u64,
                                MAX_SPRITE_INDEX_BUFFER_SIZE,
                            ));
                        }

                        let vertex_start = vertex_offset;
                        let index_start = index_offset;

                        vertex_offset += write_buffer_padded(
                            &self.queue,
                            &self.vertex_buffer,
                            vertex_start,
                            vertex_bytes,
                        );

                        index_offset += write_buffer_padded(
                            &self.queue,
                            &self.index_buffer,
                            index_start,
                            index_bytes,
                        );

                        if let ElementType::Texture(_) = &sprite.element_type {
                            let gpu_texture = match self.texture_cache.get(id) {
                                Some(g) => g,
                                None => {
                                    log::error!("GpuTexture not found in cache for id: {}", id);
                                    continue;
                                }
                            };

                            render_pass.set_pipeline(&self.world_pipeline_texture);
                            render_pass.set_bind_group(0, &gpu_texture.bind_group, &[]);
                            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
                        } else {
                            render_pass.set_pipeline(&self.world_pipeline_color);
                            render_pass.set_bind_group(0, &self.color_bind_group, &[]);
                            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
                        }

                        render_pass.set_vertex_buffer(
                            0,
                            self.vertex_buffer
                                .slice(vertex_start..vertex_start + vertex_bytes.len() as u64),
                        );

                        render_pass.set_index_buffer(
                            self.index_buffer
                                .slice(index_start..index_start + index_bytes.len() as u64),
                            wgpu::IndexFormat::Uint16,
                        );

                        render_pass.draw_indexed(
                            0..indices.len() as u32,
                            0,
                            sprite_index as u32..sprite_index as u32 + 1,
                        );
                    }
                }
            }

            vertex_offset = 0;
            index_offset = 0;

            render_pass.set_vertex_buffer(1, self.ui_element_buffer.slice(..));

            for (ui_index, id) in self.ui_render_order.iter().enumerate() {
                let ui = &self.ui_elements[id];
                match ui.get_element_type() {
                    ElementType::None => continue,
                    ElementType::Text(text) => {
                        let (gpu_texture, first, count) =
                            match self.glyph_atlas_cache.get(&(*id, text.get_size() as u32)) {
                                Some((_, g, f, c)) => (g, f, c),
                                None => {
                                    log::error!("Failed to find glyph atlas for text: {:?}", text);
                                    continue;
                                }
                            };

                        if *count == 0 {
                            continue;
                        }

                        let slot = self.text_uniform_index[id];

                        render_pass.set_pipeline(&self.ui_pipeline_text);
                        render_pass.set_bind_group(0, &gpu_texture.bind_group, &[]);
                        render_pass.set_bind_group(1, &self.ui_camera_bind_group, &[]);
                        render_pass.set_bind_group(
                            2,
                            &self.text_uniform_bind_group,
                            &[slot * self.text_uniform_stride as u32],
                        );
                        render_pass.set_vertex_buffer(0, self.glyph_instance_buffer.slice(..));
                        render_pass.draw(0..6, *first..*first + *count);
                    }
                    _ => {
                        let (vertices, indices) = sprite_geometry(ui.get_vertices());
                        let vertex_bytes = bytemuck::cast_slice(&vertices);
                        let index_bytes = bytemuck::cast_slice(&indices);

                        let vertex_padded_len = (vertex_bytes.len() + 3) & !3;
                        let index_padded_len = (index_bytes.len() + 3) & !3;

                        if vertex_offset + vertex_padded_len as u64 > MAX_SPRITE_VERTEX_BUFFER_SIZE
                        {
                            return Err(Error::IndexOverflow(
                                *id,
                                vertex_offset + vertex_padded_len as u64,
                                MAX_SPRITE_VERTEX_BUFFER_SIZE,
                            ));
                        }
                        if index_offset + index_padded_len as u64 > MAX_SPRITE_INDEX_BUFFER_SIZE {
                            return Err(Error::IndexOverflow(
                                *id,
                                index_offset + index_padded_len as u64,
                                MAX_SPRITE_INDEX_BUFFER_SIZE,
                            ));
                        }

                        let vertex_start = vertex_offset;
                        let index_start = index_offset;

                        vertex_offset += write_buffer_padded(
                            &self.queue,
                            &self.vertex_buffer,
                            vertex_start,
                            vertex_bytes,
                        );

                        index_offset += write_buffer_padded(
                            &self.queue,
                            &self.index_buffer,
                            index_start,
                            index_bytes,
                        );

                        if let ElementType::Texture(_) = &ui.get_element_type() {
                            let gpu_texture = match self.texture_cache.get(id) {
                                Some(g) => g,
                                None => {
                                    log::error!("GpuTexture not found in cache for id: {}", id);
                                    continue;
                                }
                            };

                            render_pass.set_pipeline(&self.ui_pipeline_texture);
                            render_pass.set_bind_group(0, &gpu_texture.bind_group, &[]);
                            render_pass.set_bind_group(1, &self.ui_camera_bind_group, &[]);
                        } else {
                            render_pass.set_pipeline(&self.ui_pipeline_color);
                            render_pass.set_bind_group(0, &self.color_bind_group, &[]);
                            render_pass.set_bind_group(1, &self.ui_camera_bind_group, &[]);
                        }

                        render_pass.set_vertex_buffer(
                            0,
                            self.vertex_buffer
                                .slice(vertex_start..vertex_start + vertex_bytes.len() as u64),
                        );

                        render_pass.set_index_buffer(
                            self.index_buffer
                                .slice(index_start..index_start + index_bytes.len() as u64),
                            wgpu::IndexFormat::Uint16,
                        );

                        render_pass.draw_indexed(
                            0..indices.len() as u32,
                            0,
                            ui_index as u32..ui_index as u32 + 1,
                        );
                    }
                }
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.present(output);

        Ok(())
    }

    pub fn get_window(&self) -> &Arc<Window> {
        &self.window
    }

    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    pub fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn get_fps(&self) -> f32 {
        self.fps
    }
}

fn write_buffer_padded(
    queue: &wgpu::Queue,
    buffer: &wgpu::Buffer,
    offset: u64,
    data: &[u8],
) -> u64 {
    let alignment = wgpu::COPY_BUFFER_ALIGNMENT as usize;
    let padded_len = (data.len() + alignment - 1) & !(alignment - 1);

    if padded_len == data.len() {
        queue.write_buffer(buffer, offset, data);
    } else {
        let mut padded = vec![0u8; padded_len];
        padded[..data.len()].copy_from_slice(data);
        queue.write_buffer(buffer, offset, &padded);
    }

    padded_len as u64
}
