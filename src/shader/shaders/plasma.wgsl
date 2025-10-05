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
    effect_time: f32,
    effect_type: u32,
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

fn voronoi_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let scale = uniforms.frequency * 2.0;
    let cell = floor(uv * scale);
    let fract_uv = fract(uv * scale);
    
    var min_dist = 10.0;
    var second_min = 10.0;
    
    for (var y = -1; y <= 1; y = y + 1) {
        for (var x = -1; x <= 1; x = x + 1) {
            let neighbor = vec2<f32>(f32(x), f32(y));
            let cell_pos = cell + neighbor;
            
            let point_offset = vec2<f32>(
                simple_hash(cell_pos + vec2<f32>(time * 0.1, 0.0)),
                simple_hash(cell_pos + vec2<f32>(0.0, time * 0.15))
            );
            
            let point = neighbor + point_offset - fract_uv;
            let dist = length(point);
            
            if dist < min_dist {
                second_min = min_dist;
                min_dist = dist;
            } else if dist < second_min {
                second_min = dist;
            }
        }
    }
    
    let edge = second_min - min_dist;
    return vec2<f32>(min_dist * 2.0 - 1.0, edge * 3.0);
}

fn truchet_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let scale = uniforms.frequency;
    let cell = floor(uv * scale);
    let local = fract(uv * scale);
    
    let hash_val = simple_hash(cell + vec2<f32>(time * 0.05, 0.0));
    let rotation = floor(hash_val * 4.0);
    
    var rotated: vec2<f32>;
    if rotation < 1.0 {
        rotated = local;
    } else if rotation < 2.0 {
        rotated = vec2<f32>(1.0 - local.y, local.x);
    } else if rotation < 3.0 {
        rotated = vec2<f32>(1.0 - local.x, 1.0 - local.y);
    } else {
        rotated = vec2<f32>(local.y, 1.0 - local.x);
    }
    
    let arc1 = length(rotated - vec2<f32>(0.0, 0.0));
    let arc2 = length(rotated - vec2<f32>(1.0, 1.0));
    let pattern = min(abs(arc1 - 0.5), abs(arc2 - 0.5));
    
    return vec2<f32>(pattern * 4.0 - 1.0, pattern * 2.0);
}

fn hexagonal_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let scale = uniforms.frequency;
    let hex_uv = uv * scale * vec2<f32>(1.0, 0.866);
    
    let q = vec2<f32>(hex_uv.x * 1.1547, hex_uv.y - hex_uv.x * 0.5773);
    let p = vec2<f32>(floor(q.x), floor(q.y));
    let r = fract(q);
    
    var hex_center: vec2<f32>;
    if r.x + r.y > 1.0 {
        hex_center = vec2<f32>(1.0 - r.x, 1.0 - r.y);
    } else {
        hex_center = r;
    }
    
    let cell_id = p + vec2<f32>(time * 0.1, 0.0);
    let hash_val = simple_hash(cell_id);
    let hex_dist = length(hex_center - vec2<f32>(0.5)) * 2.0;
    
    return vec2<f32>(sin(hex_dist * 6.28 + hash_val * 6.28) * (1.0 - hex_dist), hex_dist);
}

fn interference_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let p1 = vec2<f32>(0.3 + sin(time * 0.5) * 0.2, 0.5 + cos(time * 0.3) * 0.2);
    let p2 = vec2<f32>(0.7 + cos(time * 0.4) * 0.2, 0.5 + sin(time * 0.6) * 0.2);
    let p3 = vec2<f32>(0.5 + sin(time * 0.7) * 0.2, 0.3 + cos(time * 0.5) * 0.2);
    
    let d1 = distance(uv, p1);
    let d2 = distance(uv, p2);
    let d3 = distance(uv, p3);
    
    let wave1 = sin(d1 * uniforms.frequency * 15.0 - time * 3.0);
    let wave2 = sin(d2 * uniforms.frequency * 15.0 - time * 3.5);
    let wave3 = sin(d3 * uniforms.frequency * 15.0 - time * 4.0);
    
    let interference = (wave1 + wave2 + wave3) / 3.0;
    return vec2<f32>(interference, abs(wave1 - wave2));
}

fn fractal_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    var z = (uv - 0.5) * 3.0;
    var value = 0.0;
    var gradient = 0.0;
    
    for (var i = 0; i < 5; i = i + 1) {
        let fi = f32(i);
        let angle = time * 0.1 * (1.0 + fi * 0.1);
        let s = sin(angle);
        let c = cos(angle);
        z = vec2<f32>(z.x * c - z.y * s, z.x * s + z.y * c);
        
        z = abs(z);
        z = z * uniforms.frequency * 0.5 - vec2<f32>(1.0, 0.5);
        
        let d = length(z);
        value += exp(-d * 2.0) / (1.0 + fi);
        gradient += d / (1.0 + fi);
    }
    
    return vec2<f32>(value * 2.0 - 1.0, gradient * 0.5);
}

fn glitch_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let glitch_time = floor(time * 5.0) * 0.2;
    let row = floor(uv.y * uniforms.frequency * 3.0);
    let glitch_amount = simple_hash(vec2<f32>(row, glitch_time));
    
    var offset_uv = uv;
    if glitch_amount > 0.7 {
        offset_uv.x += (simple_hash(vec2<f32>(row * 2.0, glitch_time)) - 0.5) * 0.1;
    }
    
    let scan_line = sin(uv.y * uniforms.frequency * 50.0 + time * 10.0) * 0.5 + 0.5;
    let col_shift = abs(simple_hash(vec2<f32>(floor(offset_uv.x * uniforms.frequency * 2.0), glitch_time)) - 0.5);
    
    let blocks = step(0.5, simple_hash(floor(offset_uv * uniforms.frequency) + vec2<f32>(glitch_time, 0.0)));
    
    return vec2<f32>((scan_line * col_shift + blocks) * 2.0 - 1.0, scan_line);
}

fn spiral_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let center = vec2<f32>(0.5, 0.5);
    let diff = uv - center;
    let radius = length(diff);
    let angle = atan2(diff.y, diff.x);
    
    // Spiral arms - spacious, not filled
    let num_arms = 5.0;
    let spiral_angle = angle * num_arms + radius * uniforms.frequency * 3.0 - time;
    let spiral_val = sin(spiral_angle);
    
    // Only show the arms, not fill the space
    let arm_thickness = 0.3;
    let arm_strength = smoothstep(arm_thickness, 0.0, abs(spiral_val));
    
    return vec2<f32>(arm_strength * 2.0 - 1.0, spiral_val);
}

fn rings_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let center = vec2<f32>(0.5, 0.5);
    let dist = distance(uv, center);
    
    // Concentric rings - spacious
    let ring_value = sin(dist * uniforms.frequency * 15.0 - time * 2.0);
    
    // Only show the rings themselves, not fill
    let ring_thickness = 0.2;
    let ring_strength = smoothstep(ring_thickness, 0.0, abs(ring_value));
    
    return vec2<f32>(ring_strength * 2.0 - 1.0, ring_value);
}

fn grid_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    // Flowing grid lines - spacious
    let flow_speed = time * 0.3;
    let grid_x = sin((uv.x + flow_speed * 0.5) * uniforms.frequency * 8.0);
    let grid_y = sin((uv.y + flow_speed * 0.7) * uniforms.frequency * 8.0);
    
    // Only show grid lines, not fill squares
    let line_thickness = 0.15;
    let x_line = smoothstep(line_thickness, 0.0, abs(grid_x));
    let y_line = smoothstep(line_thickness, 0.0, abs(grid_y));
    
    let grid_strength = max(x_line, y_line);
    
    return vec2<f32>(grid_strength * 2.0 - 1.0, grid_x - grid_y);
}

fn diamonds_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    // Diamond lattice - spacious
    let offset = vec2<f32>(sin(time * 0.3) * 0.2, cos(time * 0.4) * 0.2);
    let rotated = (uv + offset) * uniforms.frequency * 5.0;
    
    // Create diamond shape using Manhattan distance
    let diamond_x = abs(fract(rotated.x + rotated.y) - 0.5);
    let diamond_y = abs(fract(rotated.x - rotated.y) - 0.5);
    let diamond_dist = diamond_x + diamond_y;
    
    // Only show diamond edges, not fill
    let edge_thickness = 0.15;
    let diamond_strength = smoothstep(edge_thickness, 0.0, abs(diamond_dist - 0.5));
    
    return vec2<f32>(diamond_strength * 2.0 - 1.0, diamond_x - diamond_y);
}

fn sphere_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    // Rotating 3D sphere (like Earth) - perspective projection
    let center = vec2<f32>(0.5, 0.5);
    let pos = (uv - center) * 2.0; // -1 to 1
    
    // Calculate if point is on sphere
    let dist_sq = pos.x * pos.x + pos.y * pos.y;
    
    if dist_sq > 1.0 {
        // Outside sphere - empty space
        return vec2<f32>(-1.0, 0.0);
    }
    
    // Calculate z coordinate (depth on sphere)
    let z = sqrt(1.0 - dist_sq);
    
    // 3D point on sphere surface
    let sphere_point = vec3<f32>(pos.x, pos.y, z);
    
    // Rotate sphere
    let rotation_speed = time * 0.5;
    let cos_t = cos(rotation_speed);
    let sin_t = sin(rotation_speed);
    
    // Rotate around Y axis
    let rotated_x = sphere_point.x * cos_t + sphere_point.z * sin_t;
    let rotated_z = -sphere_point.x * sin_t + sphere_point.z * cos_t;
    let rotated = vec3<f32>(rotated_x, sphere_point.y, rotated_z);
    
    // Create latitude/longitude grid (like Earth)
    let lat = asin(rotated.y);
    let lon = atan2(rotated.z, rotated.x);
    
    // Grid lines for latitude and longitude
    let lat_lines = sin(lat * uniforms.frequency * 3.0);
    let lon_lines = sin(lon * uniforms.frequency * 3.0);
    
    // Combine grid lines
    let grid_strength = max(
        smoothstep(0.15, 0.0, abs(lat_lines)),
        smoothstep(0.15, 0.0, abs(lon_lines))
    );
    
    // Add shading based on light direction (lighting from front-right)
    let light_dir = normalize(vec3<f32>(0.5, 0.3, 1.0));
    let normal = normalize(rotated);
    let diffuse = max(dot(normal, light_dir), 0.0);
    
    // Combine grid with shading
    let sphere_value = grid_strength * 0.7 + diffuse * 0.3;
    
    return vec2<f32>(sphere_value * 2.0 - 1.0, diffuse);
}

fn compute_pattern(uv: vec2<f32>, time: f32, pattern_type: u32) -> vec2<f32> {
    var value: f32;
    var gradient: f32;
    
    if pattern_type == 0u {
        let v1 = sin(uv.x * uniforms.frequency + time);
        let v2 = sin(uniforms.frequency * (uv.x * sin(time / 2.0) + uv.y * cos(time / 3.0)) + time);
        let cx = uv.x + 0.5 * sin(time / 5.0) * uniforms.distort_amplitude;
        let cy = uv.y + 0.5 * cos(time / 3.0) * uniforms.distort_amplitude;
        // Removed radial wave (v3) to eliminate center-outward effect
        let v3 = sin((cx + cy) * uniforms.frequency * 0.5 + time);
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
    } else if pattern_type == 5u {
        let grid_x = floor(uv.x * uniforms.frequency) + sin(time * 0.5);
        let grid_y = floor(uv.y * uniforms.frequency) + cos(time * 0.3);
        value = simple_hash(vec2<f32>(grid_x, grid_y)) * 2.0 - 1.0;
        gradient = fract(uv.x * uniforms.frequency) - 0.5;
    } else if pattern_type == 6u {
        let result = voronoi_pattern(uv, time);
        value = result.x;
        gradient = result.y;
    } else if pattern_type == 7u {
        let result = truchet_pattern(uv, time);
        value = result.x;
        gradient = result.y;
    } else if pattern_type == 8u {
        let result = hexagonal_pattern(uv, time);
        value = result.x;
        gradient = result.y;
    } else if pattern_type == 9u {
        let result = interference_pattern(uv, time);
        value = result.x;
        gradient = result.y;
    } else if pattern_type == 10u {
        let result = fractal_pattern(uv, time);
        value = result.x;
        gradient = result.y;
    } else if pattern_type == 11u {
        let result = glitch_pattern(uv, time);
        value = result.x;
        gradient = result.y;
    } else if pattern_type == 12u {
        let result = spiral_pattern(uv, time);
        value = result.x;
        gradient = result.y;
    } else if pattern_type == 13u {
        let result = rings_pattern(uv, time);
        value = result.x;
        gradient = result.y;
    } else if pattern_type == 14u {
        let result = grid_pattern(uv, time);
        value = result.x;
        gradient = result.y;
    } else if pattern_type == 15u {
        let result = diamonds_pattern(uv, time);
        value = result.x;
        gradient = result.y;
    } else {
        let result = sphere_pattern(uv, time);
        value = result.x;
        gradient = result.y;
    }
    
    return vec2<f32>(value * uniforms.amplitude, gradient);
}

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
    } else {
        let wave_y = center.y + sin(position.x * 10.0 - elapsed * 5.0) * 0.1;
        let wave_dist = abs(position.y - wave_y);
        let wave_thickness = 0.05;
        effect_strength = smoothstep(wave_thickness, 0.0, wave_dist);
    }
    
    let fade = 1.0 - (elapsed / 3.0);
    effect_strength = effect_strength * fade;
    
    var effect_color: vec3<f32>;
    
    let effect_index = uniforms.effect_type % 6u;
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

fn plasma_effect(position: vec2<f32>, time: f32) -> vec3<f32> {
    let uv = position * uniforms.scale;
    
    let pattern_result = compute_pattern(uv, time, uniforms.pattern_type);
    let combined = pattern_result.x;
    let gradient = pattern_result.y;
    
    var color = apply_color_mode(combined, gradient, uniforms.color_mode);
    
    color = apply_color_adjustments(color);
    
    color = apply_effect(position, uv, color, time);
    
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
