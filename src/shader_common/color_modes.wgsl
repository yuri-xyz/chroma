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
        // Warm - Enhanced with more dynamic range
        let t = base_value * 0.5 + 0.5;
        let intensity = pow(t, 0.8); // Slight compression for better contrast
        color = vec3<f32>(
            0.6 + intensity * 0.4,  // Reds from 0.6 to 1.0
            0.2 + intensity * 0.5,  // Oranges/yellows
            0.05 + intensity * 0.25 // Subtle blues for depth
        );
    } else if mode == 4u {
        // Cool - Enhanced with more dynamic range
        let t = base_value * 0.5 + 0.5;
        let intensity = pow(t, 0.8);
        color = vec3<f32>(
            0.1 + intensity * 0.4,  // Subtle reds
            0.3 + intensity * 0.5,  // Cyans/teals
            0.5 + intensity * 0.5   // Blues from 0.5 to 1.0
        );
    } else if mode == 5u {
        // Neon
        let t = base_value * 0.5 + 0.5;
        let r = sin(t * 6.28) * 0.5 + 0.5;
        let g = sin(t * 6.28 + 2.0) * 0.5 + 0.5;
        let b = sin(t * 6.28 + 4.0) * 0.5 + 0.5;
        color = vec3<f32>(r * 1.2, g * 1.2, b * 1.2);
    } else if mode == 6u {
        // Pastel - Enhanced with more color variation
        let t = base_value * 0.5 + 0.5;
        let hue_var = sin(t * 3.14159) * 0.15; // Add hue variation
        color = vec3<f32>(
            0.65 + t * 0.3 + hue_var,
            0.55 + t * 0.35,
            0.70 + t * 0.25 - hue_var
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
    } else if mode == 9u {
        // Fire - Hot colors from black through red, orange, yellow to white
        let t = clamp(base_value * 0.5 + 0.5, 0.0, 1.0);
        let intensity = pow(t, 0.7); // Compress darks, expand highlights
        
        if intensity < 0.33 {
            // Black to deep red
            let local_t = intensity / 0.33;
            color = vec3<f32>(local_t * 0.8, 0.0, 0.0);
        } else if intensity < 0.66 {
            // Deep red to orange/yellow
            let local_t = (intensity - 0.33) / 0.33;
            color = vec3<f32>(
                0.8 + local_t * 0.2,
                local_t * 0.6,
                0.0
            );
        } else {
            // Orange/yellow to white (hottest)
            let local_t = (intensity - 0.66) / 0.34;
            color = vec3<f32>(
                1.0,
                0.6 + local_t * 0.4,
                local_t * 0.8
            );
        }
        
        // Add flicker effect using gradient
        let flicker = abs(gradient) * 0.15;
        color = color * (1.0 + flicker);
        
    } else if mode == 10u {
        // Ocean - Deep blues to cyan to seafoam
        let t = clamp(base_value * 0.5 + 0.5, 0.0, 1.0);
        let depth = pow(t, 0.9);
        
        if depth < 0.4 {
            // Deep dark blue (abyss)
            let local_t = depth / 0.4;
            color = vec3<f32>(
                0.0,
                local_t * 0.2,
                0.2 + local_t * 0.3
            );
        } else if depth < 0.7 {
            // Mid ocean blue to cyan
            let local_t = (depth - 0.4) / 0.3;
            color = vec3<f32>(
                local_t * 0.3,
                0.2 + local_t * 0.5,
                0.5 + local_t * 0.4
            );
        } else {
            // Shallow water - cyan to seafoam/white
            let local_t = (depth - 0.7) / 0.3;
            color = vec3<f32>(
                0.3 + local_t * 0.5,
                0.7 + local_t * 0.3,
                0.9 + local_t * 0.1
            );
        }
        
        // Add wave shimmer using gradient
        let shimmer = abs(gradient) * 0.2;
        color = color + vec3<f32>(shimmer * 0.3, shimmer * 0.5, shimmer * 0.4);
        
    } else if mode == 11u {
        // Aurora - Ethereal greens, blues, purples with flowing gradients
        let t = clamp(base_value * 0.5 + 0.5, 0.0, 1.0);
        
        // Create flowing aurora bands
        let flow = sin(t * 6.28 + uniforms.time * 0.3) * 0.5 + 0.5;
        let secondary_flow = cos(t * 4.0 - uniforms.time * 0.2) * 0.5 + 0.5;
        
        // Base colors: green and blue dominant with purple accents
        let green_component = sin(t * 3.14159) * 0.7 + 0.3;
        let blue_component = cos(t * 3.14159 + 1.0) * 0.6 + 0.4;
        let purple_component = sin(t * 6.28 + 2.0) * 0.4 + 0.3;
        
        color = vec3<f32>(
            purple_component * flow * 0.7,
            green_component * 0.9,
            blue_component
        );
        
        // Add streaks and wisps using gradient
        let wisp_strength = pow(abs(gradient), 0.5) * 0.4;
        color = color * (0.7 + wisp_strength) + vec3<f32>(0.1, 0.2, 0.15) * wisp_strength;
        
        // Ensure aurora has ethereal glow (avoid pure blacks)
        color = max(color, vec3<f32>(0.05, 0.08, 0.1));
        
    } else if mode == 12u {
        // Galaxy - Deep space purples, blues, magentas with star clusters
        let t = clamp(base_value * 0.5 + 0.5, 0.0, 1.0);
        
        // Create dust lanes and star regions
        let dust_density = pow(t, 1.5);
        let star_regions = smoothstep(0.7, 0.95, t);
        
        // Base galaxy colors - purple and blue nebulae
        let base_purple = vec3<f32>(0.4, 0.1, 0.6);
        let base_blue = vec3<f32>(0.1, 0.2, 0.8);
        let base_magenta = vec3<f32>(0.7, 0.1, 0.5);
        
        // Mix nebula colors based on position
        let nebula_mix1 = sin(t * 3.14159 + uniforms.time * 0.1) * 0.5 + 0.5;
        let nebula_mix2 = cos(t * 6.28 - uniforms.time * 0.15) * 0.5 + 0.5;
        
        color = mix(
            mix(base_purple, base_blue, nebula_mix1),
            base_magenta,
            nebula_mix2 * 0.4
        ) * dust_density;
        
        // Add bright star clusters in high-value regions
        let star_brightness = star_regions * 3.0;
        color = color + vec3<f32>(star_brightness * 0.9, star_brightness * 0.85, star_brightness);
        
        // Add sparkle variation using gradient
        let sparkle = pow(abs(gradient), 2.0) * star_regions * 0.5;
        color = color + vec3<f32>(sparkle);
        
        // Keep some darkness for deep space
        color = mix(vec3<f32>(0.01, 0.01, 0.02), color, smoothstep(0.0, 0.3, t));
        
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
