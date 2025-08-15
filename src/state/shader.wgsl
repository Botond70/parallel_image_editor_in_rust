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

fn hsv2rgb(hsv: vec3<f32>) -> vec3<f32> {
    let h = hsv.x;
    let s = hsv.y;
    let v = hsv.z;

    let c = v * s;
    let h_ = (h * 6.0) % 6.0;
    let x = c * (1.0 - abs(h_ % 2.0 - 1.0));
    let m = v - c;

    var rgb: vec3<f32>;

    if h_ < 1.0 {
        rgb = vec3<f32>(c, x, 0.0);
    } else if h_ < 2.0 {
        rgb = vec3<f32>(x, c, 0.0);
    } else if h_ < 3.0 {
        rgb = vec3<f32>(0.0, c, x);
    } else if h_ < 4.0 {
        rgb = vec3<f32>(0.0, x, c);
    } else if h_ < 5.0 {
        rgb = vec3<f32>(x, 0.0, c);
    } else {
        rgb = vec3<f32>(c, 0.0, x);
    }
    return rgb + vec3<f32>(m);
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
    let rgb_tint = hsv2rgb(globals.hsv);
    let tex_color = textureSample(t_diffuse, s_diffuse, in.tex_coords).rgb;
    let result_color = tex_color * rgb_tint;
    let color = vec3<f32>(globals.hsv.x, globals.hsv.y, globals.hsv.z);
    return vec4<f32>(result_color, 1.0);
}
