use std::{collections::HashMap, sync::Arc};

use vyxen_math::Vector2;
use wgpu::util::DeviceExt as _;
use winit::window::Window;

use crate::{
    Camera, Sprite,
    backend::{
        CameraUniform, GpuTexture, MAX_SPRITE_INDEX_BUFFER_SIZE, MAX_SPRITE_VERTEX_BUFFER_SIZE,
        MAX_SPRITES, SpriteRaw, Vertex, shape_geometry::sprite_geometry,
    },
};

pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    window: Arc<Window>,
    render_pipeline_texture: wgpu::RenderPipeline,
    render_pipeline_color: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    empty_bind_group: wgpu::BindGroup,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    texture_cache: HashMap<&'static str, GpuTexture>,
    sprites: Vec<Sprite>,
    sprite_buffer: wgpu::Buffer,
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<State> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
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
            zoom: 10.0,
            height: config.height as f32,
            width: config.width as f32,
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

        let texture_shader =
            device.create_shader_module(wgpu::include_wgsl!("../../../../shaders/texture.wgsl"));
        let color_shader =
            device.create_shader_module(wgpu::include_wgsl!("../../../../shaders/color.wgsl"));

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
                label: Some("Texture Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
                immediate_size: 0,
            });

        let color_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Color Render Pipeline Layout"),
                bind_group_layouts: &[&empty_bind_group_layout, &camera_bind_group_layout],
                immediate_size: 0,
            });

        let render_pipeline_texture =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline Texture"),
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

        let render_pipeline_color =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline Color"),
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
            render_pipeline_texture,
            render_pipeline_color,
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
            sprite_buffer,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = if cfg!(target_arch = "wasm32") {
                width
            } else {
                width.min(2048)
            };
            self.config.height = if cfg!(target_arch = "wasm32") {
                height
            } else {
                height.min(2048)
            };
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    pub fn update(&mut self) {
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        let sprite_data: Vec<SpriteRaw> = self.sprites.iter().map(Sprite::to_raw).collect();

        if !sprite_data.is_empty() {
            self.queue
                .write_buffer(&self.sprite_buffer, 0, bytemuck::cast_slice(&sprite_data));
        }

        for sprite in &self.sprites {
            if let crate::DrawType::Texture(texture) = &sprite.get_draw_type() {
                let label = texture.get_label();

                if !self.texture_cache.contains_key(label) {
                    let gpu_tex = GpuTexture::from_image(
                        &self.device,
                        &self.queue,
                        &self.texture_bind_group_layout,
                        texture.get_image(),
                        label,
                    )
                    .expect("Failed to create GpuTexture");

                    self.texture_cache.insert(label, gpu_tex);
                }
            }
        }
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        self.window.request_redraw();

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
            self.sprites.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
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

                let (vertices, indices) = match sprite_geometry(sprite) {
                    Some((v, i)) => (v, i),
                    None => anyhow::bail!(
                        "Failed to get geometry for sprite at index {}",
                        sprite_index
                    ),
                };
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
                        let label = texture.get_label();
                        let gpu_texture = match self.texture_cache.get(label) {
                            Some(g) => g,
                            None => {
                                anyhow::bail!("GpuTexture not found in cache for label: {}", label)
                            }
                        };

                        render_pass.set_pipeline(&self.render_pipeline_texture);
                        render_pass.set_bind_group(0, &gpu_texture.bind_group, &[]);
                        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
                    }
                    crate::DrawType::Color(_) => {
                        render_pass.set_pipeline(&self.render_pipeline_color);
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
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn get_sprites(&self) -> &Vec<Sprite> {
        &self.sprites
    }

    pub fn get_sprites_mut(&mut self) -> &mut Vec<Sprite> {
        &mut self.sprites
    }

    pub fn set_sprites(&mut self, sprites: Vec<Sprite>) {
        self.sprites = sprites;
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    pub fn get_window(&self) -> &Arc<Window> {
        &self.window
    }
}
