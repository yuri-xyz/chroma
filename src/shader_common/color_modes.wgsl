// Color mode functions and colormaps

// Warped colormap functions (for Color Mode 8)
fn warp_colormap_red(x: f32) -> f32 {
    if x < 0.0 {
        return 54.0 / 255.0;
    } else if x < 20049.0 / 82979.0 {
        return (829.79 * x + 54.51) / 255.0;
    } else {
        return 1.0;
    }
}

fn warp_colormap_green(x: f32) -> f32 {
    if x < 20049.0 / 82979.0 {
        return 0.0;
    } else if x < 327013.0 / 810990.0 {
        return (8546482679670.0 / 10875673217.0 * x - 2064961390770.0 / 10875673217.0) / 255.0;
    } else if x <= 1.0 {
        return (103806720.0 / 483977.0 * x + 19607415.0 / 483977.0) / 255.0;
    } else {
        return 1.0;
    }
}

fn warp_colormap_blue(x: f32) -> f32 {
    if x < 0.0 {
        return 54.0 / 255.0;
    } else if x < 7249.0 / 82979.0 {
        return (829.79 * x + 54.51) / 255.0;
    } else if x < 20049.0 / 82979.0 {
        return 127.0 / 255.0;
    } else if x < 327013.0 / 810990.0 {
        return (792.02249341361393720147485376583 * x - 64.364790735602331034989206222672) / 255.0;
    } else {
        return 1.0;
    }
}

fn warp_colormap(x: f32) -> vec3<f32> {
    return vec3<f32>(warp_colormap_red(x), warp_colormap_green(x), warp_colormap_blue(x));
}

// Main color mode dispatcher
fn apply_color_mode(base_value: f32, gradient: f32, mode: u32) -> vec3<f32> {
    var color: vec3<f32>;
    
    if mode == 0u {
        // Rainbow
        let red = sin(base_value * 3.14159 + uniforms.color_shift) * 0.5 + 0.5;
        let green = sin(base_value * 3.14159 + uniforms.color_shift + 2.094) * 0.5 + 0.5;
        let blue = sin(base_value * 3.14159 + uniforms.color_shift + 4.189) * 0.5 + 0.5;
        color = vec3<f32>(red, green, blue);
    } else if mode == 1u {
        // Monochrome
        let gray = base_value * 0.5 + 0.5;
        color = vec3<f32>(gray);
    } else if mode == 2u {
        // Duotone
        let t = base_value * 0.5 + 0.5;
        color = mix(vec3<f32>(0.1, 0.1, 0.2), vec3<f32>(0.9, 0.7, 0.5), t);
    } else if mode == 3u {
        // Warm
        let t = base_value * 0.5 + 0.5;
        color = vec3<f32>(
            0.8 + t * 0.2,
            0.4 + t * 0.3,
            0.2 + t * 0.1
        );
    } else if mode == 4u {
        // Cool
        let t = base_value * 0.5 + 0.5;
        color = vec3<f32>(
            0.2 + t * 0.3,
            0.5 + t * 0.3,
            0.7 + t * 0.3
        );
    } else if mode == 5u {
        // Neon
        let t = base_value * 0.5 + 0.5;
        let r = sin(t * 6.28) * 0.5 + 0.5;
        let g = sin(t * 6.28 + 2.0) * 0.5 + 0.5;
        let b = sin(t * 6.28 + 4.0) * 0.5 + 0.5;
        color = vec3<f32>(r * 1.2, g * 1.2, b * 1.2);
    } else if mode == 6u {
        // Pastel
        let t = base_value * 0.5 + 0.5;
        color = vec3<f32>(
            0.7 + t * 0.2,
            0.6 + t * 0.2,
            0.7 + t * 0.2
        );
    } else if mode == 7u {
        // Cyberpunk
        let t = base_value * 0.5 + 0.5;
        let r = step(0.5, fract(t * 3.0)) * 0.9;
        let g = sin(t * 10.0) * 0.3 + 0.5;
        let b = cos(t * 15.0) * 0.3 + 0.7;
        color = vec3<f32>(r, g, b);
    } else if mode == 8u {
        // Warped (custom colormap)
        let t = clamp(base_value * 0.5 + 0.5, 0.0, 1.0);
        color = warp_colormap(t);
    } else {
        // Chromatic (default)
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
