// Pattern: Kaleidoscope
// Symmetrical mirrored pattern that rotates - great for music visualization

fn kaleidoscope_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let center = vec2<f32>(0.5, 0.5);
    let diff = uv - center;

    // Number of mirror segments (6 or 8 looks great)
    let segments = 6.0;
    let segment_angle = 3.14159265 * 2.0 / segments;

    // Get polar coordinates
    var angle = atan2(diff.y, diff.x);
    let radius = length(diff);

    // Rotate the whole pattern slowly
    angle = angle + time * 0.2;

    // Fold angle into one segment (creates mirror effect)
    angle = abs(((angle % segment_angle) - segment_angle * 0.5));

    // Convert back to cartesian for pattern sampling
    let folded_uv = vec2<f32>(
        cos(angle) * radius,
        sin(angle) * radius
    ) + center;

    // Create interesting pattern within the kaleidoscope
    let freq = uniforms.frequency * 0.5;

    // Layer 1: Flowing waves
    let wave1 = sin(folded_uv.x * freq * 3.0 + time * 0.8) *
                cos(folded_uv.y * freq * 2.0 - time * 0.6);

    // Layer 2: Radial pulse from center
    let pulse = sin(radius * freq * 8.0 - time * 2.0);

    // Layer 3: Spiral element
    let spiral = sin(angle * 4.0 + radius * freq * 5.0 - time * 1.5);

    // Layer 4: Diamond pattern
    let diamond = sin((folded_uv.x + folded_uv.y) * freq * 4.0 + time) *
                  sin((folded_uv.x - folded_uv.y) * freq * 4.0 - time * 0.7);

    // Combine layers
    let combined = wave1 * 0.3 + pulse * 0.3 + spiral * 0.25 + diamond * 0.15;

    // Gradient for color variation
    let gradient = wave1 - pulse + spiral * 0.5;

    return vec2<f32>(combined, gradient);
}
