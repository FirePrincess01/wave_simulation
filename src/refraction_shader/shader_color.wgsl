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
    @location(0) tex_coords: vec2<f32>,
    @location(1) height: f32,
};

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

    var out: VertexOutput;
    out.height = posz;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(model.position.x, model.position.y, posz, 1.0);
    return out;
}

// Fragment shader

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

//Thanks to sam hocevar
fn hsv2rgb(c: vec3<f32>) -> vec3<f32>
{
    let K: vec4<f32> = vec4<f32>(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    let p: vec3<f32> = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, vec3<f32>(0.0), vec3<f32>(1.0)), c.y);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    //return textureSample(t_diffuse, s_diffuse, in.tex_coords);
    let scale: f32 = clamp((in.height + 1.) / 2.5, 0., 1.);
    return vec4<f32>( hsv2rgb(vec3<f32>(4./6., 1., 0.5) * (1. - scale) + vec3<f32>(1.1 /6., 0.6, 1.) * scale), 1.);//+ textureSample(t_diffuse, s_diffuse, in.tex_coords)) / 2.;
}