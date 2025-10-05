// Color conversion and adjustment utilities

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
