struct UiCameraUniform {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: UiCameraUniform;

struct TextUniform {
    matrix: mat4x4<f32>,
    color: vec4<f32>,
};
@group(2) @binding(0)
var<uniform> text: TextUniform;

struct InstanceInput {
    @location(0) rect: vec4<f32>,
    @location(1) uv_rect: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    instance: InstanceInput,
) -> VertexOutput {
    let corner_ids = array<u32, 6>(0u, 2u, 1u, 1u, 2u, 3u);
    let corner = corner_ids[vertex_index];

    let is_right = (corner == 1u || corner == 3u);
    let is_bottom = (corner == 2u || corner == 3u);

    let x = select(instance.rect.x, instance.rect.z, is_right);
    let y = select(instance.rect.y, instance.rect.w, is_bottom);
    let u = select(instance.uv_rect.x, instance.uv_rect.z, is_right);
    let v = select(instance.uv_rect.y, instance.uv_rect.w, is_bottom);

    var out: VertexOutput;
    out.tex_coords = vec2<f32>(u, v);
    out.clip_position = camera.view_proj * text.matrix * vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let sampled = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    return sampled;
}