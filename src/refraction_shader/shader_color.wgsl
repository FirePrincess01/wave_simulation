// Vertex shader
struct CameraUniform {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(2) @binding(0)
var t_heightmap: texture_2d<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

// Ported from Colorous package
fn cubehelix_to_rgb(c: vec3<f32>) -> vec3<f32> {
    let h = radians(c.x + 120.);
    let l = c.z;
    let a = c.y * l * (1.0 - l);
    let cos_h = cos(h);
    let sin_h = sin(h);
    let r = min(l - a * (0.14861 * cos_h - 1.78277 * sin_h), 1.);
    let g = min(l - a * (0.29227 * cos_h + 0.90649 * sin_h), 1.);
    let b = min(l + a * (1.97294 * cos_h), 1.);
    return vec3<f32>(r, g, b);
}

@vertex 
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    let dim: vec2<u32> = textureDimensions(t_heightmap);
    let width = dim.x;
    let height = dim.y;
    let index = vec2<u32>(vertex_index % width, vertex_index / width);

    let pos_rgb: vec4<f32> = textureLoad(t_heightmap, index, 0);
    let posz = pos_rgb.r;

    let scale: f32 = clamp((posz * 0.4 + 1.) / 2., 0., 1.);
    let hsl_color = (1. - scale) * vec3<f32>(260.0, 0.75, 0.35) + scale * vec3<f32>(80.0, 1.5, 0.8);

    var out: VertexOutput;
    out.color = cubehelix_to_rgb(hsl_color);
    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(model.position.x, model.position.y, posz, 1.0);
    return out;
}

// Fragment shader

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.);
}
