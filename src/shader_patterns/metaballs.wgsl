// Pattern: Metaballs
// Organic blob-like shapes that merge and separate - great for bass-reactive visuals

fn metaballs_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let freq = uniforms.frequency * 0.1;

    // Define metaball centers - they move organically
    let ball1 = vec2<f32>(
        0.5 + sin(time * 0.7) * 0.25,
        0.5 + cos(time * 0.9) * 0.25
    );
    let ball2 = vec2<f32>(
        0.5 + cos(time * 0.8) * 0.3,
        0.5 + sin(time * 0.6) * 0.2
    );
    let ball3 = vec2<f32>(
        0.5 + sin(time * 0.5 + 2.0) * 0.2,
        0.5 + cos(time * 0.7 + 1.0) * 0.3
    );
    let ball4 = vec2<f32>(
        0.5 + cos(time * 0.9 + 3.0) * 0.25,
        0.5 + sin(time * 0.4 + 2.0) * 0.25
    );
    let ball5 = vec2<f32>(
        0.5 + sin(time * 0.3) * 0.35,
        0.5 + cos(time * 0.5) * 0.15
    );

    // Ball sizes (pulsing with different phases)
    let size1 = 0.08 + sin(time * 2.0) * 0.02;
    let size2 = 0.1 + cos(time * 1.8) * 0.025;
    let size3 = 0.07 + sin(time * 2.2 + 1.0) * 0.015;
    let size4 = 0.09 + cos(time * 1.6 + 2.0) * 0.02;
    let size5 = 0.06 + sin(time * 2.5) * 0.015;

    // Calculate metaball field (sum of inverse distances)
    let d1 = length(uv - ball1);
    let d2 = length(uv - ball2);
    let d3 = length(uv - ball3);
    let d4 = length(uv - ball4);
    let d5 = length(uv - ball5);

    // Metaball potential field
    let field = size1 / (d1 * d1 + 0.001) +
                size2 / (d2 * d2 + 0.001) +
                size3 / (d3 * d3 + 0.001) +
                size4 / (d4 * d4 + 0.001) +
                size5 / (d5 * d5 + 0.001);

    // Threshold for blob edges
    let threshold = 15.0 + sin(time * 1.5) * 3.0;

    // Smooth blob shape
    let blob = smoothstep(threshold * 0.7, threshold * 1.3, field);

    // Inner glow/gradient based on field strength
    let inner_glow = clamp(field / threshold, 0.0, 1.0);

    // Edge detection for outline effect
    let edge = smoothstep(threshold * 0.9, threshold * 1.0, field) -
               smoothstep(threshold * 1.0, threshold * 1.1, field);

    // Combine blob body with edge highlight
    let combined = blob * 0.7 + inner_glow * 0.2 + edge * 0.5;

    // Gradient based on nearest ball influence
    let min_dist = min(min(min(d1, d2), min(d3, d4)), d5);
    let gradient = sin(min_dist * freq * 20.0 + time) + inner_glow;

    return vec2<f32>(combined * 2.0 - 1.0, gradient);
}
