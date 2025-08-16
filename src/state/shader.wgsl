struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

struct Globals {
    hsv: vec3<f32>,
}

fn hue_shift_rgb(color: vec3<f32>, hue: f32) -> vec3<f32> {
    let k: vec3<f32> = vec3<f32>(0.57735, 0.57735, 0.57735); // (1 / sqrt(3))
    let cosAngle: f32 = cos(hue);
    let sinAngle: f32 = sin(hue);

    // Rodrigues' rotation formula in RGB space:
    return color * cosAngle
         + cross(k, color) * sinAngle
         + k * dot(k, color) * (1.0 - cosAngle);
}

// Vertex shader

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

 
// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;
@group(0) @binding(2)
var<uniform> globals: Globals;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let hue = globals.hsv.x;
    let tex_color = textureSample(t_diffuse, s_diffuse, in.tex_coords).rgb;
    let shifted = hue_shift_rgb(tex_color, hue);
    return vec4<f32>(shifted, 1.0);
}
