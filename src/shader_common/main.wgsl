// Main shader entry point and pattern dispatcher

fn compute_pattern(uv: vec2<f32>, time: f32, pattern_type: u32) -> vec2<f32> {
    if pattern_type == 0u {
        return plasma_pattern(uv, time);
    } else if pattern_type == 1u {
        return waves_pattern(uv, time);
    } else if pattern_type == 2u {
        return ripples_pattern(uv, time);
    } else if pattern_type == 3u {
        return vortex_pattern(uv, time);
    } else if pattern_type == 4u {
        return noise_pattern(uv, time);
    } else if pattern_type == 5u {
        return geometric_pattern(uv, time);
    } else if pattern_type == 6u {
        return voronoi_pattern(uv, time);
    } else if pattern_type == 7u {
        return truchet_pattern(uv, time);
    } else if pattern_type == 8u {
        return hexagonal_pattern(uv, time);
    } else if pattern_type == 9u {
        return interference_pattern(uv, time);
    } else if pattern_type == 10u {
        return fractal_pattern(uv, time);
    } else if pattern_type == 11u {
        return glitch_pattern(uv, time);
    } else if pattern_type == 12u {
        return spiral_pattern(uv, time);
    } else if pattern_type == 13u {
        return rings_pattern(uv, time);
    } else if pattern_type == 14u {
        return grid_pattern(uv, time);
    } else if pattern_type == 15u {
        return diamonds_pattern(uv, time);
    } else if pattern_type == 16u {
        return sphere_pattern(uv, time);
    } else if pattern_type == 17u {
        return octgrams_pattern(uv, time);
    } else {
        return warped_fbm_pattern(uv, time);
    }
}

fn plasma_effect(position: vec2<f32>, time: f32) -> vec3<f32> {
    // Apply beat zoom first
    var processed_position = apply_beat_zoom(position, time);
    
    // Then apply beat-reactive distortion to position for visual pop effect
    processed_position = apply_beat_distortion(processed_position, time);
    
    let uv = processed_position * uniforms.scale;
    
    let pattern_result = compute_pattern(uv, time, uniforms.pattern_type);
    let combined = pattern_result.x;
    let gradient = pattern_result.y;
    
    var color = apply_color_mode(combined, gradient, uniforms.color_mode);
    
    color = apply_color_adjustments(color);
    
    color = apply_effect(position, uv, color, time);
    
    // Apply beat flash for additional pop emphasis
    color = apply_beat_flash(color, position, time);
    
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
