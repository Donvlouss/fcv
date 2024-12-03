struct CameraUniform {
    view_proj: mat4x4f
}
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3f,
    @location(1) color: vec4f,
}
struct InstanceInput {
    @location(5) model_matrix_0: vec4f,
    @location(6) model_matrix_1: vec4f,
    @location(7) model_matrix_2: vec4f,
    @location(8) model_matrix_3: vec4f,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) color: vec4f,
};

@vertex
fn vs_main(
    model: VertexInput,
    transform: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4f(
        transform.model_matrix_0,
        transform.model_matrix_1,
        transform.model_matrix_2,
        transform.model_matrix_3,
    );

    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = camera.view_proj * model_matrix * vec4f(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4f {
    return vec4f(in.color);
}