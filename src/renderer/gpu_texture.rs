use crate::{Texture, renderer::raws::TextureSettings};
use std::borrow::Cow;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct GpuTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub bind_group: wgpu::BindGroup,
    pub width: u32,
    pub height: u32,
}

impl GpuTexture {
    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        texture: &Texture,
        id: u64,
    ) -> Self {
        let rgba = texture.get_rgba();
        let dimensions = texture.get_dimensions();
        let id = id.to_string();
        let label = id.as_str();
        let grayscale = texture.grayscale;
        let invert = texture.invert;

        let size = wgpu::Extent3d {
            width: dimensions.x as u32,
            height: dimensions.y as u32,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.x as u32),
                rows_per_image: Some(dimensions.y as u32),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let texture_settings = TextureSettings {
            grayscale: if grayscale { 1.0 } else { 0.0 },
            invert: if invert { 1.0 } else { 0.0 },
            _padding: [0.0; 2],
            _data: [0.0; 4],
        };

        let texture_settings_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("texture_settings"),
            size: std::mem::size_of::<TextureSettings>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        queue.write_buffer(
            &texture_settings_buffer,
            0,
            bytemuck::bytes_of(&texture_settings),
        );

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &texture_settings_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
            label: Some(label),
        });

        Self {
            texture,
            view,
            sampler,
            bind_group,
            width: dimensions.x as u32,
            height: dimensions.y as u32,
        }
    }

    pub fn write_frame(&self, queue: &wgpu::Queue, rgba: &[u8], width: u32, height: u32) {
        debug_assert_eq!(
            (width, height),
            (self.width, self.height),
            "write_frame called with a size that doesn't match the existing texture; \
             recreate via from_image instead"
        );

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
    }
}

pub fn fit_to_device_limits<'a>(
    device: &wgpu::Device,
    rgba: &'a [u8],
    width: u32,
    height: u32,
) -> (Cow<'a, [u8]>, u32, u32) {
    let max_dim = device.limits().max_texture_dimension_2d;

    if width <= max_dim && height <= max_dim {
        return (Cow::Borrowed(rgba), width, height);
    }

    let scale = (max_dim as f32 / width as f32).min(max_dim as f32 / height as f32);
    let new_width = ((width as f32 * scale).floor() as u32).max(1);
    let new_height = ((height as f32 * scale).floor() as u32).max(1);

    let mut out = vec![0u8; (new_width * new_height * 4) as usize];
    for y in 0..new_height {
        let src_y = (y as u64 * height as u64 / new_height as u64) as u32;
        for x in 0..new_width {
            let src_x = (x as u64 * width as u64 / new_width as u64) as u32;
            let src_idx = ((src_y * width + src_x) * 4) as usize;
            let dst_idx = ((y * new_width + x) * 4) as usize;
            out[dst_idx..dst_idx + 4].copy_from_slice(&rgba[src_idx..src_idx + 4]);
        }
    }

    (Cow::Owned(out), new_width, new_height)
}
