// Pattern: Plasma
// Enhanced plasma effect with multiple layers, distortions, and color variation

fn plasma_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let freq = uniforms.frequency;
    let distort = uniforms.distort_amplitude;
    
    // Base moving coordinates with circular motion
    let cx = uv.x + 0.5 * sin(time * 0.2) * distort;
    let cy = uv.y + 0.5 * cos(time * 0.15) * distort;
    
    // Layer 1: Classic plasma wave
    let v1 = sin(uv.x * freq + time);
    
    // Layer 2: Warped coordinate plasma with rotation
    let angle = time * 0.1;
    let rotated_x = cx * cos(angle) - cy * sin(angle);
    let rotated_y = cx * sin(angle) + cy * cos(angle);
    let v2 = sin(freq * (rotated_x * sin(time * 0.5) + rotated_y * cos(time * 0.33)) + time);
    
    // Layer 3: Diagonal plasma flow
    let v3 = sin((cx + cy) * freq * 0.5 + time * 0.7);
    
    // Layer 4: Radial plasma from center
    let center = vec2<f32>(0.5, 0.5);
    let dist = length(uv - center);
    let angle_from_center = atan2(uv.y - center.y, uv.x - center.x);
    let v4 = sin(dist * freq * 2.0 - time * 0.8 + angle_from_center * 3.0);
    
    // Layer 5: Swirling vortex pattern
    let vortex_angle = angle_from_center + dist * 3.0 + time * 0.3;
    let v5 = cos(vortex_angle * 2.0) * sin(dist * freq * 1.5 - time);
    
    // Layer 6: Interference pattern between moving points
    let focal1 = vec2<f32>(0.5 + sin(time * 0.37) * 0.25, 0.5 + cos(time * 0.43) * 0.25);
    let focal2 = vec2<f32>(0.5 + cos(time * 0.31) * 0.25, 0.5 + sin(time * 0.29) * 0.25);
    let d1 = length(uv - focal1);
    let d2 = length(uv - focal2);
    let v6 = sin(d1 * freq * 3.0 - time * 1.2) + cos(d2 * freq * 3.0 - time * 1.1);
    
    // Combine layers with weights (some layers contribute more than others)
    let value = (
        v1 * 0.25 +
        v2 * 0.25 +
        v3 * 0.20 +
        v4 * 0.15 +
        v5 * 0.10 +
        v6 * 0.05
    );
    
    // Enhanced gradient calculation for better edge definition
    // Combine multiple gradient sources for richer color variation
    let gradient_base = v1 - v2;
    let gradient_radial = v4 * 0.5;
    let gradient_vortex = v5 * 0.3;
    let gradient = gradient_base + gradient_radial + gradient_vortex;
    
    return vec2<f32>(value, gradient);
}
