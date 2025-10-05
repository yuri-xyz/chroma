// Visual effects system

fn apply_effect(position: vec2<f32>, uv: vec2<f32>, color: vec3<f32>, time: f32) -> vec3<f32> {
    let elapsed = time - uniforms.effect_time;
    
    if elapsed < 0.0 || elapsed > 3.0 {
        return color;
    }
    
    let center = vec2<f32>(0.5, 0.5);
    let dist_from_center = distance(position, center);
    
    let expansion_speed = 0.5;
    let expansion_radius = elapsed * expansion_speed;
    
    var effect_strength = 0.0;
    
    if uniforms.effect_type == 0u {
        let ring_thickness = 0.08;
        let ring_dist = abs(dist_from_center - expansion_radius);
        effect_strength = smoothstep(ring_thickness, 0.0, ring_dist);
    } else if uniforms.effect_type == 1u {
        let dx = abs(position.x - center.x);
        let dy = abs(position.y - center.y);
        let cross_dist = min(dx, dy);
        let cross_ring = abs(max(dx, dy) - expansion_radius);
        let cross_thickness = 0.06;
        
        if cross_dist < 0.02 {
            effect_strength = smoothstep(cross_thickness, 0.0, cross_ring);
        }
    } else if uniforms.effect_type == 2u {
        let dx = position.x - center.x;
        let dy = position.y - center.y;
        let diamond_dist = abs(dx) + abs(dy);
        let diamond_ring = abs(diamond_dist - expansion_radius);
        let diamond_thickness = 0.08;
        effect_strength = smoothstep(diamond_thickness, 0.0, diamond_ring);
    } else if uniforms.effect_type == 3u {
        let angle = atan2(position.y - center.y, position.x - center.x);
        let num_rays = 8.0;
        let ray_angle = fract(angle / (6.28318 / num_rays));
        let ray_proximity = abs(ray_angle - 0.5) * 2.0;
        let ray_width = 0.15;
        
        if ray_proximity < ray_width && dist_from_center < expansion_radius {
            let ray_fade = 1.0 - (dist_from_center / expansion_radius);
            effect_strength = (1.0 - ray_proximity / ray_width) * ray_fade * 0.7;
        }
    } else if uniforms.effect_type == 4u {
        let grid_size = 0.1;
        let grid_x = abs(fract(position.x / grid_size) - 0.5) * 2.0;
        let grid_y = abs(fract(position.y / grid_size) - 0.5) * 2.0;
        let grid_proximity = min(grid_x, grid_y);
        let grid_width = 0.3;
        
        if grid_proximity < grid_width && dist_from_center < expansion_radius {
            let grid_fade = 1.0 - (dist_from_center / expansion_radius);
            effect_strength = (1.0 - grid_proximity / grid_width) * grid_fade * 0.5;
        }
    } else if uniforms.effect_type == 5u {
        // Octgrams-style starburst with modded repetition and box sdf-like spokes
        let angle = atan2(position.y - center.y, position.x - center.x);
        let radius = dist_from_center + 1e-5;
        let num_arms = 8.0;
        let k = 3.14159265 / num_arms; // sector half-angle
        let m = abs(fract((angle + time * 0.2) / (2.0 * k)) - 0.5) * 2.0; // repeating wedge
        let spoke = smoothstep(0.25, 0.0, m);
        let ring = smoothstep(0.06, 0.0, abs(radius - expansion_radius));
        let core = smoothstep(0.15, 0.0, radius);
        effect_strength = (spoke * 0.8 + ring * 0.6 + core * 0.4) * 0.9;
    } else {
        let wave_y = center.y + sin(position.x * 10.0 - elapsed * 5.0) * 0.1;
        let wave_dist = abs(position.y - wave_y);
        let wave_thickness = 0.05;
        effect_strength = smoothstep(wave_thickness, 0.0, wave_dist);
    }
    
    let fade = 1.0 - (elapsed / 3.0);
    effect_strength = effect_strength * fade;
    
    var effect_color: vec3<f32>;
    
    let effect_index = uniforms.effect_type % 7u;
    if effect_index == 0u {
        effect_color = vec3<f32>(0.2, 0.5, 0.9);
    } else if effect_index == 1u {
        effect_color = vec3<f32>(0.9, 0.3, 0.5);
    } else if effect_index == 2u {
        effect_color = vec3<f32>(0.3, 0.9, 0.5);
    } else if effect_index == 3u {
        effect_color = vec3<f32>(0.9, 0.7, 0.2);
    } else if effect_index == 4u {
        effect_color = vec3<f32>(0.7, 0.2, 0.9);
    } else {
        effect_color = vec3<f32>(0.2, 0.9, 0.9);
    }
    
    return mix(color, effect_color, effect_strength * 0.8);
}
