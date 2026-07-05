use std::{collections::HashMap, sync::Arc};
use vyxen_math::{Matrix4, Vector2};
use wgpu::util::DeviceExt;
use winit::{
    application::ApplicationHandler, event::*, event_loop::{ActiveEventLoop, EventLoop}, keyboard::{KeyCode, PhysicalKey}, window::Window,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;

use crate::{Sprite, backend::{gpu_texture::GpuTexture, shape_geoemtry::sprite_geometry}};

pub mod shape_geoemtry;
pub mod gpu_texture;

pub const OPENGL_TO_WGPU_MATRIX: Matrix4 = Matrix4 {
    m: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 0.5, 0.0],
        [0.0, 0.0, 0.5, 1.0],
    ],
};

const MAX_SPRITE_VERTEX_BUFFER_SIZE: u64 = 1 << 20;
const MAX_SPRITE_INDEX_BUFFER_SIZE: u64 = 1 << 20;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpriteRaw {
    pub matrix: [[f32; 4]; 4],
    pub color: [f32; 4],
}

impl SpriteRaw {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<SpriteRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[[f32;4];4]>() as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub struct Camera {
    pub position: Vector2,
    pub rotation: f32,
    pub zoom: f32,
    pub width: f32,
    pub height: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> Matrix4 {
        let view = Matrix4::translation(-self.position.x, -self.position.y, 0.0);

        let half_width = self.width / self.zoom / 2.0;
        let half_height = self.height / self.zoom / 2.0;

        let left = -half_width;
        let right = half_width;
        let bottom = -half_height;
        let top = half_height;

        let projection = Matrix4::orthographic(left, right, bottom, top, -1.0, 1.0);

        OPENGL_TO_WGPU_MATRIX * projection * view
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        Self {
            view_proj: Matrix4::identity().into(),
        }
    }

    fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

struct CameraController {
    speed: f32,
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    fn new(speed: f32) -> Self {
        Self {
            speed,
            is_up_pressed: false,
            is_down_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    fn handle_key(&mut self, code: KeyCode, is_pressed: bool) -> bool {
        match code {
            KeyCode::KeyW | KeyCode::ArrowUp => {
                self.is_up_pressed = is_pressed;
                true
            }
            KeyCode::KeyA | KeyCode::ArrowLeft => {
                self.is_left_pressed = is_pressed;
                true
            }
            KeyCode::KeyS | KeyCode::ArrowDown => {
                self.is_down_pressed = is_pressed;
                true
            }
            KeyCode::KeyD | KeyCode::ArrowRight => {
                self.is_right_pressed = is_pressed;
                true
            }
            _ => false,
        }
    }

    fn update_camera(&self, camera: &mut Camera) {
        if self.is_down_pressed {
            camera.position.y -= self.speed;
        }
        if self.is_up_pressed {
            camera.position.y += self.speed;
        }
        if self.is_left_pressed {
            camera.position.x -= self.speed;
        }
        if self.is_right_pressed {
            camera.position.x += self.speed;
        }
    }
}

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
    camera_controller: CameraController,
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
            zoom: 20.0,
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

        const MAX_SPRITES: usize = 8192;
        let sprite_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Sprite Buffer"),
            size: (std::mem::size_of::<SpriteRaw>() * MAX_SPRITES) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let texture_shader = device.create_shader_module(wgpu::include_wgsl!("../../../../shaders/texture.wgsl"));
        let color_shader = device.create_shader_module(wgpu::include_wgsl!("../../../../shaders/color.wgsl"));

        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let empty_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let color_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Color Render Pipeline Layout"),
            bind_group_layouts: &[&empty_bind_group_layout, &camera_bind_group_layout],
            immediate_size: 0,
        });

        let render_pipeline_texture = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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

        let render_pipeline_color = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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

        let camera_controller = CameraController::new(1.0);

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
            camera_controller,
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

    fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        if code == KeyCode::Escape && is_pressed {
            event_loop.exit();
        } else {
            self.camera_controller.handle_key(code, is_pressed);
        }
    }

    fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        let sprite_data: Vec<SpriteRaw> = self.sprites.iter().map(Sprite::to_raw).collect();

        if !sprite_data.is_empty() {
            self.queue.write_buffer(
                &self.sprite_buffer,
                0,
                bytemuck::cast_slice(&sprite_data),
            );
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
                    ).expect("Failed to create GpuTexture");
                    
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
            self.sprites
                .sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());

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
                    None => anyhow::bail!("Failed to get geometry for sprite at index {}", sprite_index),
                };
                let vertex_bytes = bytemuck::cast_slice(&vertices);
                let index_bytes = bytemuck::cast_slice(&indices);

                if vertex_offset + vertex_bytes.len() as u64 > MAX_SPRITE_VERTEX_BUFFER_SIZE {
                    anyhow::bail!("Sprite vertex buffer overflow");
                }
                if index_offset + index_bytes.len() as u64 > MAX_SPRITE_INDEX_BUFFER_SIZE {
                    anyhow::bail!("Sprite index buffer overflow");
                }

                self.queue.write_buffer(&self.vertex_buffer, vertex_offset, vertex_bytes);
                self.queue.write_buffer(&self.index_buffer, index_offset, index_bytes);

                match &sprite.draw_type {
                    crate::DrawType::Texture(texture) => {
                        let label = texture.get_label();
                        let gpu_texture = match self.texture_cache.get(label) {
                            Some(g) => g,
                            None => anyhow::bail!("GpuTexture not found in cache for label: {}", label),
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

                render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(vertex_offset..vertex_offset + vertex_bytes.len() as u64));
                render_pass.set_index_buffer(self.index_buffer.slice(index_offset..index_offset + index_bytes.len() as u64), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..indices.len() as u32, 0, sprite_index as u32..sprite_index as u32 + 1);

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
}

pub struct App {
    #[cfg(target_arch = "wasm32")]
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
    state: Option<State>,
    sprites: Vec<Sprite>,
}

impl App {
    #[allow(unused)]
    pub fn new(event_loop: &EventLoop<State>) -> Self {
        #[cfg(target_arch = "wasm32")]
        let proxy = Some(event_loop.create_proxy());

        Self {
            state: None,
            #[cfg(target_arch = "wasm32")]
            proxy,
            sprites: Vec::new(),
        }
    }

    pub fn get_state_mut(&mut self) -> Option<&mut State> {
        self.state.as_mut()
    }

    pub fn get_sprites_mut(&mut self) -> &mut Vec<Sprite> {
        &mut self.sprites
    }

    pub fn set_sprites(&mut self, sprites: Vec<Sprite>) {
        self.sprites = sprites;
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;

            const CANVAS_ID: &str = "canvas";

            let window = wgpu::web_sys::window().unwrap_throw();
            let document = window.document().unwrap_throw();
            let canvas = document.get_element_by_id(CANVAS_ID).unwrap_throw();
            let html_canvas_element = canvas.unchecked_into();
            window_attributes = window_attributes.with_canvas(Some(html_canvas_element));
        }

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut new_state = pollster::block_on(State::new(window)).unwrap();
            new_state.set_sprites(self.sprites.clone()); 
            self.state = Some(new_state);
        }

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(proxy) = self.proxy.take() {
                let sprites = self.pending_sprites.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let state = State::new(window).await?;
                    let _ = proxy.send_event(AppAction::StateInitialized(state, sprites));
                });
            }
        }
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        #[cfg(target_arch = "wasm32")]
        {
            event.window.request_redraw();
            event.resize(
                event.window.inner_size().width,
                event.window.inner_size().height,
            );
        }

        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("{e}");
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => state.handle_key(event_loop, code, key_state.is_pressed()),
            _ => {}
        }
    }
}