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
    var normal = vec3<f32>(0.,0.,1.);   // normal derivatives are 0 at the boundary
    // If not on boundary use negative derivatives to construct normal
    // Had multiply by (-1) for unknown reason (texture coordinates?)
    if index.x != 0u && index.x != width - 1u {
        normal.x = (textureLoad(t_heightmap, index - vec2<u32>(1u,0u), 0).r - textureLoad(t_heightmap, index + vec2<u32>(1u,0u), 0).r)/2.; // -du/dx
    }
    if index.y != 0u && index.y != height - 1u {
        normal.y = (textureLoad(t_heightmap, index - vec2<u32>(0u,1u), 0).r - textureLoad(t_heightmap, index + vec2<u32>(0u,1u), 0).r)/2.; // -du/dy
    }

    let world_pos = model_matrix * vec4<f32>(model.position.x, model.position.y, posz, 1.0);
    let cam_to_vertex = normalize(world_pos - camera.view_pos).xyz;
    let normal_world = normalize((model_matrix * vec4<f32>(normal, 0.)).xyz);
    let refraction_index_water = 3./4.;
    let refracted = refract(cam_to_vertex, normal_world, refraction_index_water);
    var refracted_local = vec4<f32>(refracted, 0.) * model_matrix; // Transposed multiplication inverts rotations
    refracted_local.y *= -1.; // Texture y is inverse to coordinate y

    let pool_height = 20.;
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    //If the refraction did not fail (only a problem with refraction index > 1)
    if refracted_local.z <= 0. {
        out.tex_coords -= (refracted_local.xy * (posz + pool_height) / (refracted_local.z)) / vec2<f32>(dim);
    }
    out.clip_position = camera.view_proj * world_pos;
    return out;
}

// Fragment shader

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
