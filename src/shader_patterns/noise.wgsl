// Pattern: Noise
// Smooth interpolated Perlin-style noise with multiple octaves

// Smooth interpolation function (quintic for smoother results than smoothstep)
fn quintic(t: f32) -> f32 {
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
}

// 2D gradient noise (Perlin-style)
fn gradient_noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    
    // Smooth interpolation
    let u = vec2<f32>(quintic(f.x), quintic(f.y));
    
    // Get gradient vectors at grid corners using hash function
    let a = simple_hash(i + vec2<f32>(0.0, 0.0));
    let b = simple_hash(i + vec2<f32>(1.0, 0.0));
    let c = simple_hash(i + vec2<f32>(0.0, 1.0));
    let d = simple_hash(i + vec2<f32>(1.0, 1.0));
    
    // Convert hash to gradient direction
    let ga = vec2<f32>(cos(a * 6.28318), sin(a * 6.28318));
    let gb = vec2<f32>(cos(b * 6.28318), sin(b * 6.28318));
    let gc = vec2<f32>(cos(c * 6.28318), sin(c * 6.28318));
    let gd = vec2<f32>(cos(d * 6.28318), sin(d * 6.28318));
    
    // Calculate dot products with distance vectors
    let va = dot(ga, f - vec2<f32>(0.0, 0.0));
    let vb = dot(gb, f - vec2<f32>(1.0, 0.0));
    let vc = dot(gc, f - vec2<f32>(0.0, 1.0));
    let vd = dot(gd, f - vec2<f32>(1.0, 1.0));
    
    // Bilinear interpolation
    return mix(mix(va, vb, u.x), mix(vc, vd, u.x), u.y);
}

// Fractional Brownian Motion - layered noise with decreasing amplitude
fn fbm_noise(p: vec2<f32>, octaves: i32) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var frequency = 1.0;
    var max_value = 0.0;
    
    for (var i = 0; i < octaves; i = i + 1) {
        value += gradient_noise(p * frequency) * amplitude;
        max_value += amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }
    
    return value / max_value;
}

fn noise_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let scale = uniforms.frequency * 3.0;
    let animated_uv = uv * scale + vec2<f32>(time * 0.15, time * 0.1);
    
    // Main noise with multiple octaves for detail
    let octaves = max(3, i32(uniforms.octaves));
    let noise1 = fbm_noise(animated_uv, octaves);
    
    // Secondary layer with different movement for complexity
    let noise2 = fbm_noise(animated_uv * 0.7 + vec2<f32>(-time * 0.12, time * 0.08), octaves);
    
    // Turbulence layer - uses absolute values for sharper features
    let turb_scale = animated_uv * 1.5 + vec2<f32>(time * 0.2, -time * 0.15);
    let turbulence = abs(fbm_noise(turb_scale, octaves - 1));
    
    // Combine layers with different weights
    let value = noise1 * 0.6 + noise2 * 0.3 + turbulence * 0.1;
    
    // Calculate gradient for edge detection (sample nearby points)
    let offset = 0.01;
    let dx = fbm_noise((uv + vec2<f32>(offset, 0.0)) * scale, octaves - 1) - 
             fbm_noise((uv - vec2<f32>(offset, 0.0)) * scale, octaves - 1);
    let dy = fbm_noise((uv + vec2<f32>(0.0, offset)) * scale, octaves - 1) - 
             fbm_noise((uv - vec2<f32>(0.0, offset)) * scale, octaves - 1);
    let gradient = length(vec2<f32>(dx, dy));
    
    return vec2<f32>(value * 2.0 - 1.0, gradient * 2.0);
}
