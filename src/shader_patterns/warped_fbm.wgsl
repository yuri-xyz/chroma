// Warped fBM
// Domain-warped fractional Brownian motion

fn warp_rand(n: vec2<f32>) -> f32 {
    return fract(sin(dot(n, vec2<f32>(12.9898, 4.1414))) * 43758.5453);
}

fn warp_noise(p: vec2<f32>) -> f32 {
    let ip = floor(p);
    let u = fract(p);
    let u_smooth = u * u * (3.0 - 2.0 * u);
    
    let res = mix(
        mix(warp_rand(ip), warp_rand(ip + vec2<f32>(1.0, 0.0)), u_smooth.x),
        mix(warp_rand(ip + vec2<f32>(0.0, 1.0)), warp_rand(ip + vec2<f32>(1.0, 1.0)), u_smooth.x),
        u_smooth.y
    );
    
    return res * res;
}

fn warp_fbm(p_input: vec2<f32>, time: f32) -> f32 {
    let rotation_matrix = mat2x2<f32>(0.80, 0.60, -0.60, 0.80);
    
    var p = p_input;
    var f = 0.0;
    
    f += 0.500000 * warp_noise(p + time);
    p = rotation_matrix * p * 2.02;
    
    f += 0.031250 * warp_noise(p);
    p = rotation_matrix * p * 2.01;
    
    f += 0.250000 * warp_noise(p);
    p = rotation_matrix * p * 2.03;
    
    f += 0.125000 * warp_noise(p);
    p = rotation_matrix * p * 2.01;
    
    f += 0.062500 * warp_noise(p);
    p = rotation_matrix * p * 2.04;
    
    f += 0.015625 * warp_noise(p + sin(time));
    
    return f / 0.96875;
}

fn warp_pattern_value(p_input: vec2<f32>, time: f32) -> f32 {
    let warped = warp_fbm(p_input + warp_fbm(p_input + warp_fbm(p_input, time), time), time);
    return warped;
}

fn warped_fbm_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let scaled_uv = uv * uniforms.frequency;
    let shade = warp_pattern_value(scaled_uv, time * uniforms.speed);
    
    return vec2<f32>(shade * 2.0 - 1.0, shade);
}
