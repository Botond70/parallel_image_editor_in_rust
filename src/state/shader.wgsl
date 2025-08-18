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
    let h = hsv.x * 6.0;
    let s = hsv.y;
    let v = hsv.z;

    let i = floor(h);
    let f = h - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));
    if i == 0.0 { return vec3<f32>(v, t, p); }
    if i == 1.0 { return vec3<f32>(q, v, p); }
    if i == 2.0 { return vec3<f32>(p, v, t); }
    if i == 3.0 { return vec3<f32>(p, q, v); }
    if i == 4.0 { return vec3<f32>(t, p, v); }
    return vec3<f32>(v, p, q);
}

fn rgb2hsv(c: vec3<f32>) -> vec3<f32> {
    let mx = max(c.x, max(c.y, c.z));
    let mn = min(c.x, min(c.y, c.z));
    let d = mx - mn;
    var h = 0.0;
    if d != 0.0 {
        if mx == c.x {
            h = (c.y - c.z) / d;
        } else if mx == c.y {
            h = 2.0 + (c.z - c.x) / d;
        } else {
            h = 4.0 + (c.x - c.y) / d;
        }
        h = h / 6.0;
        if h < 0.0 { h = h + 1.0; }
    };


    let v = mx;
    if mx == 0.0 { return vec3<f32>(h, 0.0, v);} else { return vec3<f32>(h, d / mx, v);};
}

fn hue_shift_rgb(color: vec3<f32>, hue: f32) -> vec3<f32> {
    let k: vec3<f32> = vec3<f32>(0.57735, 0.57735, 0.57735); // (1 / sqrt(3))
    let cosAngle: f32 = cos(hue);
    let sinAngle: f32 = sin(hue);

    // Rodrigues' rotation formula in RGB space:
    return color * cosAngle + cross(k, color) * sinAngle + k * dot(k, color) * (1.0 - cosAngle);
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
    var hsv_out = rgb2hsv(shifted);
    hsv_out.y *= globals.hsv.y + 0.9;
    hsv_out.z *= globals.hsv.z + 1.0;
    let rgb_out = hsv2rgb(hsv_out);
    return vec4<f32>(rgb_out, 1.0);
}
