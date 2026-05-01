// Pattern: Infinity
// Projected 3D infinity-loop tube with slow drift and stretch variation

fn infinity_rotate_x(point: vec3<f32>, angle: f32) -> vec3<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return vec3<f32>(point.x, point.y * c - point.z * s, point.y * s + point.z * c);
}

fn infinity_rotate_y(point: vec3<f32>, angle: f32) -> vec3<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return vec3<f32>(point.x * c + point.z * s, point.y, -point.x * s + point.z * c);
}

fn infinity_rotate_z(point: vec3<f32>, angle: f32) -> vec3<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return vec3<f32>(point.x * c - point.y * s, point.x * s + point.y * c, point.z);
}

fn infinity_rotate(point: vec3<f32>, angles: vec3<f32>) -> vec3<f32> {
    var rotated = infinity_rotate_x(point, angles.x);
    rotated = infinity_rotate_y(rotated, angles.y);
    rotated = infinity_rotate_z(rotated, angles.z);
    return rotated;
}

fn infinity_project(point: vec3<f32>) -> vec2<f32> {
    let perspective = 1.0 / (2.05 - point.z * 0.52);
    return point.xy * perspective;
}

fn infinity_hash(value: f32) -> f32 {
    return fract(sin(value * 127.1 + 311.7) * 43758.5453);
}

fn infinity_size_variation() -> f32 {
    let seed = uniforms.noise_scale * 997.0 +
        uniforms.z_rate * 313.0 +
        uniforms.gamma * 29.37 +
        uniforms.vignette_softness * 41.91 +
        uniforms.glyph_sharpness * 53.23;

    return 0.68 + infinity_hash(seed) * 0.74;
}

fn infinity_drift(time: f32) -> vec4<f32> {
    return vec4<f32>(
        0.5 + 0.5 * sin(time * 0.09 + 0.7),
        0.5 + 0.5 * sin(time * 0.13 + 2.9),
        0.5 + 0.5 * sin(time * 0.07 + 4.6),
        0.5 + 0.5 * sin(time * 0.11 + 6.1)
    );
}

fn infinity_beat_glow(time: f32) -> f32 {
    let elapsed = time - uniforms.beat_distortion_time;

    if elapsed < 0.0 || elapsed > 1.15 {
        return 0.0;
    }

    let beat_strength = max(uniforms.beat_distortion_strength, uniforms.beat_zoom_strength);
    let attack = smoothstep(0.0, 0.08, elapsed);
    let decay = exp(-elapsed * 2.35);
    let pulse = 0.5 + 0.5 * sin(elapsed * 18.0);

    return clamp(beat_strength * attack * decay * (0.72 + pulse * 0.28), 0.0, 1.0);
}

fn infinity_motion_time(time: f32) -> f32 {
    return time;
}

fn infinity_curve(t: f32, drift: vec4<f32>) -> vec3<f32> {
    let stretch = 0.92 + drift.x * 0.48;
    let height = 0.36 + drift.y * 0.28;
    let depth = 0.24 + drift.z * 0.42;
    let wave = sin(t * 3.0 + drift.w * 6.2831853) * 0.06;

    return vec3<f32>(
        sin(t) * stretch,
        sin(t * 2.0) * height,
        cos(t) * depth + wave
    );
}

fn infinity_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let center = vec2<f32>(0.5, 0.5);
    var p = (uv - center) * 2.15;
    p.x *= uniforms.resolution.x / uniforms.resolution.y;

    let beat_glow = infinity_beat_glow(time);
    let motion_time = infinity_motion_time(time);
    let drift = infinity_drift(motion_time);
    let size = infinity_size_variation() * (0.88 + drift.w * 0.20);
    let velocity = vec2<f32>(
        sin(motion_time * 0.16 + drift.x * 6.2831853),
        cos(motion_time * 0.12 + drift.y * 6.2831853)
    ) * 0.055;
    let sample_p = p - velocity;
    let angles = vec3<f32>(
        0.55 + motion_time * (0.11 + drift.x * 0.05),
        motion_time * (0.23 + drift.y * 0.08),
        0.22 * sin(motion_time * 0.18 + drift.z * 6.2831853)
    );
    let tube_radius = 0.055 + drift.z * 0.035;
    let idle_glow = 0.12 + 0.08 * sin(motion_time * 1.6 + drift.x * 6.2831853);
    let glow_activity = clamp(idle_glow + beat_glow, 0.0, 1.0);
    let glow_radius = tube_radius * (2.8 + glow_activity * 3.4);
    let shell_radius = tube_radius * (1.55 + beat_glow * 1.25);

    var best_core = 0.0;
    var best_glow = 0.0;
    var best_shell = 0.0;
    var best_depth = -2.0;
    var best_phase = 0.0;

    for (var i = 0u; i < 72u; i = i + 1u) {
        let fi = f32(i);
        let t = fi / 72.0 * 6.2831853 + motion_time * (0.08 + drift.y * 0.04);
        let curve_point = infinity_rotate(infinity_curve(t, drift) * size, angles);
        let projected = infinity_project(curve_point);
        let distance_to_tube = distance(sample_p, projected);
        let core = smoothstep(tube_radius, 0.0, distance_to_tube);
        let shell = smoothstep(shell_radius, tube_radius, distance_to_tube) * beat_glow;
        let glow = smoothstep(glow_radius, 0.0, distance_to_tube) * (0.24 + glow_activity * 0.72);
        let depth_weight = 0.62 + curve_point.z * 0.38;
        let lit_core = core * clamp(depth_weight, 0.24, 1.0);

        best_glow = max(best_glow, glow);
        best_shell = max(best_shell, shell);

        if lit_core > best_core {
            best_core = lit_core;
            best_depth = curve_point.z;
            best_phase = sin(t * 5.0 + motion_time * 0.75);
        }
    }

    let body = max(best_core, max(best_glow, best_shell));

    if body <= 0.01 {
        return vec2<f32>(-1.0, -999.0);
    }

    let highlight = best_core * (0.55 + best_phase * 0.18) +
        best_glow * (0.25 + beat_glow * 0.7) +
        best_shell * (0.35 + beat_glow * 0.9);
    let value = clamp(body + highlight, 0.0, 1.0);
    let gradient = best_depth + best_phase * 0.35 + beat_glow * 0.45;

    return vec2<f32>(value * 2.0 - 1.0, gradient);
}
