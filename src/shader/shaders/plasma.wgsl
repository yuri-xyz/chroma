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
    color_mode: u32,
    pattern_type: u32,
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

fn simple_hash(p: vec2<f32>) -> f32 {
    let p3 = fract(vec3<f32>(p.x, p.y, p.x) * 0.1031);
    let p3_dot = dot(p3, vec3<f32>(p3.y, p3.z, p3.x) + 33.33);
    return fract((p3.x + p3.y + p3.z) * p3_dot);
}

fn apply_color_mode(base_value: f32, gradient: f32, mode: u32) -> vec3<f32> {
    var color: vec3<f32>;
    
    if mode == 0u {
        let red = sin(base_value * 3.14159 + uniforms.color_shift) * 0.5 + 0.5;
        let green = sin(base_value * 3.14159 + uniforms.color_shift + 2.094) * 0.5 + 0.5;
        let blue = sin(base_value * 3.14159 + uniforms.color_shift + 4.189) * 0.5 + 0.5;
        color = vec3<f32>(red, green, blue);
    } else if mode == 1u {
        let gray = base_value * 0.5 + 0.5;
        color = vec3<f32>(gray);
    } else if mode == 2u {
        let t = base_value * 0.5 + 0.5;
        color = mix(vec3<f32>(0.1, 0.1, 0.2), vec3<f32>(0.9, 0.7, 0.5), t);
    } else if mode == 3u {
        let t = base_value * 0.5 + 0.5;
        color = vec3<f32>(
            0.8 + t * 0.2,
            0.4 + t * 0.3,
            0.2 + t * 0.1
        );
    } else if mode == 4u {
        let t = base_value * 0.5 + 0.5;
        color = vec3<f32>(
            0.2 + t * 0.3,
            0.5 + t * 0.3,
            0.7 + t * 0.3
        );
    } else if mode == 5u {
        let t = base_value * 0.5 + 0.5;
        let r = sin(t * 6.28) * 0.5 + 0.5;
        let g = sin(t * 6.28 + 2.0) * 0.5 + 0.5;
        let b = sin(t * 6.28 + 4.0) * 0.5 + 0.5;
        color = vec3<f32>(r * 1.2, g * 1.2, b * 1.2);
    } else if mode == 6u {
        let t = base_value * 0.5 + 0.5;
        color = vec3<f32>(
            0.7 + t * 0.2,
            0.6 + t * 0.2,
            0.7 + t * 0.2
        );
    } else if mode == 7u {
        let t = base_value * 0.5 + 0.5;
        let r = step(0.5, fract(t * 3.0)) * 0.9;
        let g = sin(t * 10.0) * 0.3 + 0.5;
        let b = cos(t * 15.0) * 0.3 + 0.7;
        color = vec3<f32>(r, g, b);
    } else {
        let base_gray = base_value * 0.5 + 0.5;
        let edge_detect = abs(gradient) * 3.0;
        let color_amount = smoothstep(0.3, 0.8, edge_detect);
        
        let hue_shift = base_value * 2.0;
        let r = sin(hue_shift + uniforms.time * 0.5) * color_amount;
        let g = sin(hue_shift + uniforms.time * 0.5 + 2.0) * color_amount;
        let b = sin(hue_shift + uniforms.time * 0.5 + 4.0) * color_amount;
        
        color = vec3<f32>(base_gray + r * 0.3, base_gray + g * 0.3, base_gray + b * 0.3);
    }
    
    return color;
}

fn compute_pattern(uv: vec2<f32>, time: f32, pattern_type: u32) -> vec2<f32> {
    var value: f32;
    var gradient: f32;
    
    if pattern_type == 0u {
        let v1 = sin(uv.x * uniforms.frequency + time);
        let v2 = sin(uniforms.frequency * (uv.x * sin(time / 2.0) + uv.y * cos(time / 3.0)) + time);
        let cx = uv.x + 0.5 * sin(time / 5.0) * uniforms.distort_amplitude;
        let cy = uv.y + 0.5 * cos(time / 3.0) * uniforms.distort_amplitude;
        let v3 = sin(sqrt(100.0 * (cx * cx + cy * cy) + 1.0) + time);
        value = (v1 + v2 + v3) / 3.0;
        gradient = v1 - v2;
    } else if pattern_type == 1u {
        value = sin(uv.x * uniforms.frequency + time) * cos(uv.y * uniforms.frequency * 0.7 + time * 0.8);
        gradient = cos(uv.x * uniforms.frequency + time);
    } else if pattern_type == 2u {
        let center = vec2<f32>(0.5 + sin(time * 0.3) * 0.2, 0.5 + cos(time * 0.4) * 0.2);
        let dist = distance(uv, center);
        value = sin(dist * uniforms.frequency * 10.0 - time * 2.0);
        gradient = cos(dist * uniforms.frequency * 10.0);
    } else if pattern_type == 3u {
        let angle = atan2(uv.y - 0.5, uv.x - 0.5);
        let radius = distance(uv, vec2<f32>(0.5));
        value = sin(angle * uniforms.frequency + radius * 10.0 - time);
        gradient = cos(angle * uniforms.frequency);
    } else if pattern_type == 4u {
        let n1 = simple_hash(uv * uniforms.frequency + vec2<f32>(time * 0.5, 0.0));
        let n2 = simple_hash(uv * uniforms.frequency * 0.7 + vec2<f32>(0.0, time * 0.3));
        value = (n1 + n2) * 2.0 - 1.0;
        gradient = n1 - 0.5;
    } else {
        let grid_x = floor(uv.x * uniforms.frequency) + sin(time * 0.5);
        let grid_y = floor(uv.y * uniforms.frequency) + cos(time * 0.3);
        value = simple_hash(vec2<f32>(grid_x, grid_y)) * 2.0 - 1.0;
        gradient = fract(uv.x * uniforms.frequency) - 0.5;
    }
    
    return vec2<f32>(value * uniforms.amplitude, gradient);
}

fn plasma_effect(position: vec2<f32>, time: f32) -> vec3<f32> {
    let uv = position * uniforms.scale;
    
    let pattern_result = compute_pattern(uv, time, uniforms.pattern_type);
    let combined = pattern_result.x;
    let gradient = pattern_result.y;
    
    var color = apply_color_mode(combined, gradient, uniforms.color_mode);
    
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
