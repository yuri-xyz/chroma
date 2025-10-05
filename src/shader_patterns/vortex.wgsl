// Pattern: Vortex
// Spiral pattern combining angle and radius

fn vortex_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let angle = atan2(uv.y - 0.5, uv.x - 0.5);
    let radius = distance(uv, vec2<f32>(0.5));
    
    let value = sin(angle * uniforms.frequency + radius * 10.0 - time);
    let gradient = cos(angle * uniforms.frequency);
    
    return vec2<f32>(value, gradient);
}
