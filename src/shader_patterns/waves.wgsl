// Pattern: Waves
// Multi-layered wave interference with directional flow and modulation

fn waves_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let freq = uniforms.frequency;
    
    // Primary horizontal waves with phase modulation
    let wave1 = sin(uv.x * freq * 1.0 + time * 1.0 + sin(uv.y * 2.0) * 0.5);
    
    // Vertical waves moving at different speed
    let wave2 = cos(uv.y * freq * 0.7 + time * 0.8 + cos(uv.x * 1.5) * 0.4);
    
    // Diagonal waves for complexity
    let diag1 = sin((uv.x + uv.y) * freq * 0.5 + time * 0.6);
    let diag2 = cos((uv.x - uv.y) * freq * 0.5 - time * 0.5);
    
    // Circular ripples emanating from center
    let center = vec2<f32>(0.5, 0.5);
    let dist_from_center = length(uv - center);
    let ripple = sin(dist_from_center * freq * 2.0 - time * 2.0) * 0.5;
    
    // Moving focal points that create dynamic ripples
    let focal1 = vec2<f32>(0.5 + sin(time * 0.3) * 0.3, 0.5 + cos(time * 0.4) * 0.3);
    let focal2 = vec2<f32>(0.5 + cos(time * 0.25) * 0.3, 0.5 + sin(time * 0.35) * 0.3);
    
    let ripple1 = sin(length(uv - focal1) * freq * 3.0 - time * 1.5) * 0.3;
    let ripple2 = cos(length(uv - focal2) * freq * 3.0 - time * 1.7) * 0.3;
    
    // Amplitude modulation - waves that grow and shrink
    let amp_mod = 0.7 + sin(time * 0.5) * 0.3;
    
    // Combine all wave layers with different weights
    let value = (
        wave1 * 0.35 +
        wave2 * 0.25 +
        diag1 * 0.15 +
        diag2 * 0.10 +
        ripple * 0.08 +
        ripple1 * 0.04 +
        ripple2 * 0.03
    ) * amp_mod;
    
    // Calculate gradient for edge detection and color variation
    // Sample directional derivative along the dominant wave direction
    let gradient_x = cos(uv.x * freq + time) * 0.6 + sin((uv.x + uv.y) * freq * 0.5 + time * 0.6) * 0.4;
    let gradient_y = sin(uv.y * freq * 0.7 + time * 0.8) * 0.5;
    let gradient = gradient_x + gradient_y + ripple * 0.5;
    
    return vec2<f32>(value, gradient);
}
