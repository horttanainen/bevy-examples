struct CameraUniform {
  view_proj: mat4x4<f32>,
}
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct RotationUniform {
  rotation: mat3x3<f32>,
};
@group(2) @binding(0)
var<uniform> rotation: RotationUniform;

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
};

struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
  model: VertexInput,
  instance: InstanceInput
) -> VertexOutput {
 let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

  var out: VertexOutput;
  out.tex_coords = model.tex_coords;
  var rotated_model = rotation.rotation * model.position;
  out.clip_position = camera.view_proj * model_matrix * vec4<f32>(rotated_model, 1.0);
  return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}

@group(0) @binding(0)
var t_shadow: texture_2d<f32>;
@group(0) @binding(1)
var s_shadow: sampler;

@fragment
fn fs_shadow(in: VertexOutput) -> @location(0) vec4<f32> {
    let near = 0.1;
    let far = 100.0;
    let depth = textureSample(t_shadow, s_shadow, in.tex_coords).x;
    let r = (2.0 * near) / (far + near - depth * (far - near));
    return vec4<f32>(vec3<f32>(r), 1.0);
}
