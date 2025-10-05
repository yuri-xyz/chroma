// Pattern: Ripples
// Concentric circular waves from moving center

fn ripples_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let center = vec2<f32>(0.5 + sin(time * 0.3) * 0.2, 0.5 + cos(time * 0.4) * 0.2);
    let dist = distance(uv, center);
    
    let value = sin(dist * uniforms.frequency * 10.0 - time * 2.0);
    let gradient = cos(dist * uniforms.frequency * 10.0);
    
    return vec2<f32>(value, gradient);
}
