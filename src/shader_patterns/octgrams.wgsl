// Octgrams
// 3D raymarched tunnel with animated rotating boxes

fn rot2d_octgrams(angle: f32) -> mat2x2<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return mat2x2<f32>(c, s, -s, c);
}

fn sd_box_octgrams(position: vec3<f32>, bounds: vec3<f32>) -> f32 {
    let q = abs(position) - bounds;
    return length(max(q, vec3<f32>(0.0))) + min(max(q.x, max(q.y, q.z)), 0.0);
}

fn box_shape_octgrams(position: vec3<f32>, scale: f32, rotation_angle: f32) -> f32 {
    var pos = position * scale;
    let base = sd_box_octgrams(pos, vec3<f32>(0.4, 0.4, 0.1)) / 1.5;
    pos = pos * vec3<f32>(5.0, 5.0, 1.0);
    pos.y = pos.y - 3.5;
    let rotated_xy = rot2d_octgrams(rotation_angle) * pos.xy;
    pos = vec3<f32>(rotated_xy.x, rotated_xy.y, pos.z);
    return -base;
}

fn box_set_octgrams(position: vec3<f32>, local_time: f32) -> f32 {
    let pos_origin = position;
    let wave = sin(local_time * 0.4);
    let scale_mod = 2.0 - abs(wave) * 1.5;
    
    var pos = pos_origin;
    pos.y = pos.y + wave * 2.5;
    let rot_xy1 = rot2d_octgrams(0.8) * pos.xy;
    pos = vec3<f32>(rot_xy1.x, rot_xy1.y, pos.z);
    let box1 = box_shape_octgrams(pos, scale_mod, 0.75);
    
    pos = pos_origin;
    pos.y = pos.y - wave * 2.5;
    let rot_xy2 = rot2d_octgrams(0.8) * pos.xy;
    pos = vec3<f32>(rot_xy2.x, rot_xy2.y, pos.z);
    let box2 = box_shape_octgrams(pos, scale_mod, 0.75);
    
    pos = pos_origin;
    pos.x = pos.x + wave * 2.5;
    let rot_xy3 = rot2d_octgrams(0.8) * pos.xy;
    pos = vec3<f32>(rot_xy3.x, rot_xy3.y, pos.z);
    let box3 = box_shape_octgrams(pos, scale_mod, 0.75);
    
    pos = pos_origin;
    pos.x = pos.x - wave * 2.5;
    let rot_xy4 = rot2d_octgrams(0.8) * pos.xy;
    pos = vec3<f32>(rot_xy4.x, rot_xy4.y, pos.z);
    let box4 = box_shape_octgrams(pos, scale_mod, 0.75);
    
    pos = pos_origin;
    let rot_xy5 = rot2d_octgrams(0.8) * pos.xy;
    pos = vec3<f32>(rot_xy5.x, rot_xy5.y, pos.z);
    let box5 = box_shape_octgrams(pos, 0.5, 0.75) * 6.0;
    
    let box6 = box_shape_octgrams(pos_origin, 0.5, 0.75) * 6.0;
    
    return max(max(max(max(max(box1, box2), box3), box4), box5), box6);
}

fn octgrams_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let center = vec2<f32>(0.5, 0.5);
    let screen_pos = (uv - center) * 2.0;
    let aspect = uniforms.resolution.x / uniforms.resolution.y;
    let p = vec2<f32>(screen_pos.x * aspect, screen_pos.y);
    
    let ray_origin = vec3<f32>(0.0, -0.2, time * uniforms.speed * 4.0);
    var ray_dir = normalize(vec3<f32>(p.x, p.y, 1.5));
    
    let rot_xy = rot2d_octgrams(sin(time * 0.03) * 5.0) * ray_dir.xy;
    ray_dir = vec3<f32>(rot_xy.x, rot_xy.y, ray_dir.z);
    let rot_yz = rot2d_octgrams(sin(time * 0.05) * 0.2) * ray_dir.yz;
    ray_dir = vec3<f32>(ray_dir.x, rot_yz.x, rot_yz.y);
    
    var t = 0.1;
    var accumulator = 0.0;
    
    for (var i = 0; i < 32; i = i + 1) {
        var pos = ray_origin + ray_dir * t;
        pos = (pos - 2.0) - floor((pos - 2.0) / 4.0) * 4.0 - 2.0;
        
        let local_time = time - f32(i) * 0.01;
        let distance = box_set_octgrams(pos, local_time);
        
        let d = max(abs(distance), 0.01);
        accumulator = accumulator + exp(-d * 23.0);
        
        t = t + d * 0.55;
    }
    
    let base_value = accumulator * 0.02 * uniforms.amplitude;
    let blue_wave = 0.5 + sin(time) * 0.2;
    let combined = base_value + blue_wave * 0.3;
    
    let gradient = base_value - blue_wave * 0.2;
    
    return vec2<f32>(combined * 2.0 - 1.0, gradient);
}
