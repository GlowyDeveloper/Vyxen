enable primitive_index;

// Vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
    @location(9) color: vec4<f32>,
};

struct DebugTriangle {
    a: vec3<f32>,
    _pad_a: f32,

    b: vec3<f32>,
    _pad_b: f32,

    c: vec3<f32>,
    _pad_c: f32,
};

struct DebugUniform {
    triangle_offset: u32,
    _padding: vec3<u32>,
};

@group(2) @binding(0)
var<storage, read> debug_triangles: array<DebugTriangle>;

@group(2) @binding(1)
var<uniform> debug: DebugUniform;

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(model.position, 1.0);
    out.color = instance.color;
    return out;
}

// Fragment shader
fn hash_color(triangle: DebugTriangle) -> vec3<f32> {
    let p = triangle.a
        + triangle.b * 2.0
        + triangle.c * 3.0;

    return fract(
        sin(
            vec3<f32>(
                dot(p, vec3<f32>(127.1, 311.7, 74.7)),
                dot(p, vec3<f32>(269.5, 183.3, 246.1)),
                dot(p, vec3<f32>(113.5, 271.9, 124.6))
            )
        ) * 43758.5453
    );
}

@fragment
fn fs_main(
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec4<f32>,
    @builtin(primitive_index) primitive_index: u32,
) -> @location(0) vec4<f32> {
    let triangle =
        debug_triangles[debug.triangle_offset + primitive_index];

    let debug_color = hash_color(triangle);

    return vec4<f32>(debug_color, 1.0);
}