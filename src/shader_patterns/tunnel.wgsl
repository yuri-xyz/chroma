// Pattern: Tunnel
// Infinite tunnel/wormhole effect - classic music visualizer

fn tunnel_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let center = vec2<f32>(0.5, 0.5);
    let diff = uv - center;

    // Polar coordinates
    let angle = atan2(diff.y, diff.x);
    var radius = length(diff);

    // Prevent division by zero at center
    radius = max(radius, 0.001);

    // Tunnel depth (inverse radius creates infinite tunnel effect)
    let depth = 1.0 / radius;

    // Tunnel coordinates
    let tunnel_x = angle / 3.14159265; // -1 to 1 around the tunnel
    let tunnel_y = depth * 0.5 - time * uniforms.speed * 2.0; // Moving through tunnel

    let freq = uniforms.frequency * 0.3;

    // Layer 1: Tunnel rings
    let rings = sin(depth * freq * 10.0 - time * 3.0);

    // Layer 2: Spiral stripes on tunnel walls
    let stripes = sin(angle * 8.0 + depth * freq * 5.0 - time * 2.0);

    // Layer 3: Pulsing glow from center
    let glow = exp(-radius * 3.0) * sin(time * 4.0) * 0.5 + 0.5;

    // Layer 4: Hexagonal pattern on walls
    let hex_angle = angle * 3.0;
    let hex = sin(hex_angle) * sin(depth * freq * 8.0 - time * 1.5);

    // Layer 5: Depth fog/atmosphere
    let fog = 1.0 - exp(-depth * 0.1);

    // Combine with depth-based weighting
    let wall_pattern = rings * 0.4 + stripes * 0.3 + hex * 0.3;
    let combined = mix(glow, wall_pattern, clamp(radius * 2.0, 0.0, 1.0));

    // Add subtle warping based on audio-reactive distortion
    let warp = sin(angle * 4.0 + time) * uniforms.distort_amplitude * 0.3;

    let gradient = stripes - rings + warp;

    return vec2<f32>(combined + warp, gradient);
}
