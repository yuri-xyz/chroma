// Pattern: Waves
// Horizontal and vertical wave interference

fn waves_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let value = sin(uv.x * uniforms.frequency + time) * cos(uv.y * uniforms.frequency * 0.7 + time * 0.8);
    let gradient = cos(uv.x * uniforms.frequency + time);
    
    return vec2<f32>(value, gradient);
}
