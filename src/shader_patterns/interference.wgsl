// Pattern: Interference
// Wave interference from multiple sources

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
