// Pattern: Noise
// Random hash-based noise pattern

fn noise_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let n1 = simple_hash(uv * uniforms.frequency + vec2<f32>(time * 0.5, 0.0));
    let n2 = simple_hash(uv * uniforms.frequency * 0.7 + vec2<f32>(0.0, time * 0.3));
    
    let value = (n1 + n2) * 2.0 - 1.0;
    let gradient = n1 - 0.5;
    
    return vec2<f32>(value, gradient);
}
