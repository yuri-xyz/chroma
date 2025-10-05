// Pattern: Rings
// Concentric rings pattern

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
