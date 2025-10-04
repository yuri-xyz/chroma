struct Uniforms {
    time: f32,
    resolution: vec2<f32>,
    frequency: f32,
    amplitude: f32,
    speed: f32,
    color_shift: f32,
    scale: f32,
    octaves: u32,
    noise_strength: f32,
    distort_amplitude: f32,
    noise_scale: f32,
    z_rate: f32,
    brightness: f32,
    contrast: f32,
    hue: f32,
    saturation: f32,
    gamma: f32,
    vignette: f32,
    vignette_softness: f32,
    glyph_sharpness: f32,
    background_tint: vec3<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(0) @binding(1)
var<storage, read_write> output_buffer: array<vec4<f32>>;

fn rgb_to_hsv(rgb: vec3<f32>) -> vec3<f32> {
    let max_val = max(max(rgb.r, rgb.g), rgb.b);
    let min_val = min(min(rgb.r, rgb.g), rgb.b);
    let delta = max_val - min_val;
    
    var hue = 0.0;
    if delta > 0.0001 {
        if max_val == rgb.r {
            hue = 60.0 * (((rgb.g - rgb.b) / delta) % 6.0);
        } else if max_val == rgb.g {
            hue = 60.0 * (((rgb.b - rgb.r) / delta) + 2.0);
        } else {
            hue = 60.0 * (((rgb.r - rgb.g) / delta) + 4.0);
        }
    }
    
    var saturation = 0.0;
    if max_val > 0.0001 {
        saturation = delta / max_val;
    }
    
    return vec3<f32>(hue, saturation, max_val);
}

fn hsv_to_rgb(hsv: vec3<f32>) -> vec3<f32> {
    let hue = hsv.x;
    let saturation = hsv.y;
    let value = hsv.z;
    
    let c = value * saturation;
    let x = c * (1.0 - abs(((hue / 60.0) % 2.0) - 1.0));
    let m = value - c;
    
    var rgb = vec3<f32>(0.0);
    
    if hue < 60.0 {
        rgb = vec3<f32>(c, x, 0.0);
    } else if hue < 120.0 {
        rgb = vec3<f32>(x, c, 0.0);
    } else if hue < 180.0 {
        rgb = vec3<f32>(0.0, c, x);
    } else if hue < 240.0 {
        rgb = vec3<f32>(0.0, x, c);
    } else if hue < 300.0 {
        rgb = vec3<f32>(x, 0.0, c);
    } else {
        rgb = vec3<f32>(c, 0.0, x);
    }
    
    return rgb + vec3<f32>(m);
}

fn apply_color_adjustments(color: vec3<f32>) -> vec3<f32> {
    var adjusted = color;
    
    adjusted = (adjusted - 0.5) * uniforms.contrast + 0.5;
    adjusted = adjusted * uniforms.brightness;
    
    var hsv = rgb_to_hsv(adjusted);
    hsv.x = (hsv.x + uniforms.hue) % 360.0;
    hsv.y = hsv.y * uniforms.saturation;
    adjusted = hsv_to_rgb(hsv);
    
    adjusted = pow(adjusted, vec3<f32>(1.0 / uniforms.gamma));
    
    adjusted = clamp(adjusted, vec3<f32>(0.0), vec3<f32>(1.0));
    
    return adjusted;
}

fn plasma_effect(position: vec2<f32>, time: f32) -> vec3<f32> {
    let uv = position * uniforms.scale;
    
    let value1 = sin(uv.x * uniforms.frequency + time);
    let value2 = sin(uniforms.frequency * (uv.x * sin(time / 2.0) + uv.y * cos(time / 3.0)) + time);
    
    let cx = uv.x + 0.5 * sin(time / 5.0) * uniforms.distort_amplitude;
    let cy = uv.y + 0.5 * cos(time / 3.0) * uniforms.distort_amplitude;
    let value3 = sin(sqrt(100.0 * (cx * cx + cy * cy) + 1.0) + time);
    
    let combined = (value1 + value2 + value3) / 3.0 * uniforms.amplitude;
    
    let red = sin(combined * 3.14159 + uniforms.color_shift) * 0.5 + 0.5;
    let green = sin(combined * 3.14159 + uniforms.color_shift + 2.094) * 0.5 + 0.5;
    let blue = sin(combined * 3.14159 + uniforms.color_shift + 4.189) * 0.5 + 0.5;
    
    var color = vec3<f32>(red, green, blue);
    
    color = apply_color_adjustments(color);
    
    if uniforms.vignette > 0.0 {
        let center_dist = distance(position, vec2<f32>(0.5, 0.5));
        let vignette_amount = smoothstep(
            uniforms.vignette,
            uniforms.vignette + uniforms.vignette_softness,
            1.0 - center_dist
        );
        color = mix(uniforms.background_tint, color, vignette_amount);
    }
    
    return color;
}

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let dimensions = vec2<u32>(u32(uniforms.resolution.x), u32(uniforms.resolution.y));
    
    if (global_id.x >= dimensions.x || global_id.y >= dimensions.y) {
        return;
    }
    
    let index = global_id.y * dimensions.x + global_id.x;
    
    let uv = vec2<f32>(
        f32(global_id.x) / uniforms.resolution.x,
        f32(global_id.y) / uniforms.resolution.y
    );
    
    let color = plasma_effect(uv, uniforms.time);
    
    output_buffer[index] = vec4<f32>(color, 1.0);
}
