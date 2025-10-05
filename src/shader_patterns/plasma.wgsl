// Pattern: Plasma
// Classic plasma effect using combined sine waves

fn plasma_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let v1 = sin(uv.x * uniforms.frequency + time);
    let v2 = sin(uniforms.frequency * (uv.x * sin(time / 2.0) + uv.y * cos(time / 3.0)) + time);
    let cx = uv.x + 0.5 * sin(time / 5.0) * uniforms.distort_amplitude;
    let cy = uv.y + 0.5 * cos(time / 3.0) * uniforms.distort_amplitude;
    let v3 = sin((cx + cy) * uniforms.frequency * 0.5 + time);
    
    let value = (v1 + v2 + v3) / 3.0;
    let gradient = v1 - v2;
    
    return vec2<f32>(value, gradient);
}
