// Pattern: Fluid
// Advanced water/fluid simulation with caustics, layered waves, and organic flow

// Simplex-like gradient noise for fluid flow
fn fluid_grad_noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);

    // Quintic interpolation for smoother gradients
    let u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);

    let a = simple_hash(i);
    let b = simple_hash(i + vec2<f32>(1.0, 0.0));
    let c = simple_hash(i + vec2<f32>(0.0, 1.0));
    let d = simple_hash(i + vec2<f32>(1.0, 1.0));

    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y) * 2.0 - 1.0;
}

// Multi-octave fluid noise with flow distortion
fn fluid_fbm(p: vec2<f32>, time: f32) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var frequency = 1.0;
    var flow = p;

    for (var i = 0; i < 5; i++) {
        // Add time-based flow distortion at each octave
        let flow_offset = vec2<f32>(
            sin(time * 0.3 + f32(i) * 0.7) * 0.2,
            cos(time * 0.4 + f32(i) * 0.5) * 0.2
        );
        value += amplitude * fluid_grad_noise(flow * frequency + flow_offset);
        frequency *= 2.0;
        amplitude *= 0.5;
        // Rotate each octave slightly for more organic look
        let angle = 0.5 + time * 0.02;
        flow = vec2<f32>(
            flow.x * cos(angle) - flow.y * sin(angle),
            flow.x * sin(angle) + flow.y * cos(angle)
        );
    }
    return value;
}

// Caustics effect - light patterns from water surface refraction
fn caustics(p: vec2<f32>, time: f32) -> f32 {
    var c = 0.0;
    let scale = uniforms.frequency * 3.0;

    // Layer 1: Primary caustic pattern
    let p1 = p * scale;
    let t1 = time * uniforms.speed * 0.5;
    c += pow(abs(sin(p1.x + sin(p1.y + t1) * 2.0) *
               sin(p1.y + sin(p1.x + t1 * 1.3) * 2.0)), 0.5);

    // Layer 2: Secondary caustic at different scale and speed
    let p2 = p * scale * 1.7 + vec2<f32>(3.14, 1.57);
    let t2 = time * uniforms.speed * 0.7;
    c += pow(abs(sin(p2.x + cos(p2.y * 0.7 + t2) * 1.5) *
               sin(p2.y + cos(p2.x * 0.9 + t2 * 0.8) * 1.5)), 0.6) * 0.5;

    // Layer 3: Fine detail caustics
    let p3 = p * scale * 3.2;
    let t3 = time * uniforms.speed * 0.3;
    c += pow(abs(sin(p3.x * 0.8 + sin(p3.y * 1.1 + t3) * 1.2) *
               sin(p3.y * 0.9 + sin(p3.x * 0.8 + t3 * 1.1) * 1.3)), 0.7) * 0.25;

    return c / 1.75;
}

// Voronoi-based foam/bubble pattern
fn foam_cells(p: vec2<f32>, time: f32) -> f32 {
    let scale = uniforms.frequency * 4.0;
    let cell = floor(p * scale);
    let fract_p = fract(p * scale);

    var min_dist = 1.0;

    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let neighbor = vec2<f32>(f32(x), f32(y));
            let cell_pos = cell + neighbor;

            // Animated cell points
            let point_offset = vec2<f32>(
                simple_hash(cell_pos) + sin(time * 0.5 + simple_hash(cell_pos + vec2<f32>(10.0, 0.0)) * 6.28) * 0.15,
                simple_hash(cell_pos + vec2<f32>(0.0, 10.0)) + cos(time * 0.6 + simple_hash(cell_pos + vec2<f32>(0.0, 10.0)) * 6.28) * 0.15
            );

            let point = neighbor + point_offset - fract_p;
            min_dist = min(min_dist, length(point));
        }
    }

    // Create foam-like edges
    return smoothstep(0.0, 0.4, min_dist);
}

// Layered wave interference pattern
fn water_waves(p: vec2<f32>, time: f32) -> f32 {
    let freq = uniforms.frequency;
    let spd = uniforms.speed;

    // Multiple wave sources with different directions
    let wave1 = sin(p.x * freq * 2.0 + p.y * freq * 0.5 + time * spd);
    let wave2 = sin(p.x * freq * 1.5 - p.y * freq * 1.2 + time * spd * 0.8 + 1.0);
    let wave3 = sin((p.x + p.y) * freq * 1.8 + time * spd * 0.6 + 2.0);
    let wave4 = cos((p.x - p.y) * freq * 2.2 - time * spd * 0.9);

    // Circular ripples from animated points
    let ripple_center1 = vec2<f32>(0.3 + sin(time * 0.2) * 0.2, 0.5 + cos(time * 0.3) * 0.2);
    let ripple_center2 = vec2<f32>(0.7 + cos(time * 0.25) * 0.15, 0.4 + sin(time * 0.35) * 0.15);

    let ripple1 = sin(length(p - ripple_center1) * freq * 5.0 - time * spd * 1.5) * 0.3;
    let ripple2 = sin(length(p - ripple_center2) * freq * 6.0 - time * spd * 1.3) * 0.25;

    return (wave1 * 0.3 + wave2 * 0.25 + wave3 * 0.2 + wave4 * 0.15 + ripple1 + ripple2) * 0.5 + 0.5;
}

// Curl noise for fluid flow visualization
fn curl_flow(p: vec2<f32>, time: f32) -> f32 {
    let eps = 0.01;

    // Compute curl from noise gradient
    let n1 = fluid_fbm(p + vec2<f32>(eps, 0.0), time);
    let n2 = fluid_fbm(p - vec2<f32>(eps, 0.0), time);
    let n3 = fluid_fbm(p + vec2<f32>(0.0, eps), time);
    let n4 = fluid_fbm(p - vec2<f32>(0.0, eps), time);

    let curl = ((n1 - n2) - (n3 - n4)) / (4.0 * eps);

    return curl * 0.5 + 0.5;
}

fn fluid_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let amp = uniforms.amplitude;

    // Flow-distorted coordinates for organic motion
    let flow_time = time * uniforms.speed * 0.3;
    let flow_distort = vec2<f32>(
        fluid_fbm(uv * 2.0, flow_time) * 0.08,
        fluid_fbm(uv * 2.0 + vec2<f32>(5.0, 3.0), flow_time) * 0.08
    ) * amp;
    let distorted_uv = uv + flow_distort;

    // Compute caustics - the star of the show
    let caustic_value = caustics(distorted_uv, time);

    // Add layered waves
    let wave_value = water_waves(distorted_uv, time);

    // Subtle foam/bubble structure
    let foam_value = foam_cells(distorted_uv, time);

    // Curl flow for swirling patterns
    let curl_value = curl_flow(uv * uniforms.frequency, time);

    // Combine layers with artistic weighting
    // Caustics are primary, waves add motion, foam adds texture
    let combined = caustic_value * 0.5 +
                   wave_value * 0.25 +
                   foam_value * 0.15 +
                   curl_value * 0.1;

    // Depth simulation - center is "deeper" (darker base)
    let depth = 1.0 - length(uv - vec2<f32>(0.5, 0.5)) * 0.5;

    // Final value with depth influence
    let value = combined * depth;

    // Gradient based on caustic intensity and wave direction
    let gradient = caustic_value * 0.6 + wave_value * 0.4 +
                   sin(time * 0.5) * 0.1;

    return vec2<f32>(value * 2.0 - 1.0, gradient);
}
