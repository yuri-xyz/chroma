// Pattern: Pyramid
// Rotating 3D pyramid with a drifting planetary ring on a transparent black field

struct PyramidFaceHit {
    inside: bool,
    value: f32,
    gradient: f32,
    depth: f32,
};

fn pyramid_drift(time: f32) -> vec4<f32> {
    return vec4<f32>(
        0.5 + 0.5 * sin(time * 0.17 + 1.3),
        0.5 + 0.5 * sin(time * 0.11 + 3.7),
        0.5 + 0.5 * sin(time * 0.13 + 5.1),
        0.5 + 0.5 * sin(time * 0.07 + 8.4)
    );
}

fn pyramid_rotation_angles(time: f32, drift: vec4<f32>) -> vec3<f32> {
    let spin = time * (0.28 + drift.w * 0.14);
    let wander = vec3<f32>(
        sin(time * 0.23 + drift.x * 6.2831853),
        sin(time * 0.19 + drift.y * 6.2831853),
        sin(time * 0.29 + drift.z * 6.2831853)
    ) * 0.62;

    return vec3<f32>(
        0.58 + spin * 0.47 + wander.x,
        0.20 + spin * 0.73 + wander.y,
        -0.22 + spin * 0.31 + wander.z
    );
}

fn pyramid_center_for_rotation() -> vec3<f32> {
    return vec3<f32>(0.0, 0.11, 0.0);
}

fn pyramid_rotate_x(point: vec3<f32>, angle: f32) -> vec3<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return vec3<f32>(point.x, point.y * c - point.z * s, point.y * s + point.z * c);
}

fn pyramid_rotate_y(point: vec3<f32>, angle: f32) -> vec3<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return vec3<f32>(point.x * c + point.z * s, point.y, -point.x * s + point.z * c);
}

fn pyramid_rotate_z(point: vec3<f32>, angle: f32) -> vec3<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return vec3<f32>(point.x * c - point.y * s, point.x * s + point.y * c, point.z);
}

fn pyramid_rotate(point: vec3<f32>, angles: vec3<f32>) -> vec3<f32> {
    var rotated = pyramid_rotate_x(point, angles.x);
    rotated = pyramid_rotate_y(rotated, angles.y);
    rotated = pyramid_rotate_z(rotated, angles.z);
    return rotated;
}

fn pyramid_project(point: vec3<f32>) -> vec2<f32> {
    let perspective = 1.0 / (1.9 - point.z * 0.45);
    return point.xy * perspective;
}

fn pyramid_edge(a: vec2<f32>, b: vec2<f32>, p: vec2<f32>) -> f32 {
    return (p.x - a.x) * (b.y - a.y) - (p.y - a.y) * (b.x - a.x);
}

fn pyramid_face(
    p: vec2<f32>,
    a3: vec3<f32>,
    b3: vec3<f32>,
    c3: vec3<f32>,
    face_tint: f32
) -> PyramidFaceHit {
    let a = pyramid_project(a3);
    let b = pyramid_project(b3);
    let c = pyramid_project(c3);
    let e0 = pyramid_edge(a, b, p);
    let e1 = pyramid_edge(b, c, p);
    let e2 = pyramid_edge(c, a, p);
    let has_neg = e0 < 0.0 || e1 < 0.0 || e2 < 0.0;
    let has_pos = e0 > 0.0 || e1 > 0.0 || e2 > 0.0;
    let inside = !(has_neg && has_pos);
    let normal = normalize(cross(b3 - a3, c3 - a3));
    let light_dir = normalize(vec3<f32>(-0.35, 0.55, 0.9));
    let rim_dir = normalize(vec3<f32>(0.5, -0.25, 0.6));
    let light = max(dot(normal, light_dir), 0.0) * 0.7 + max(dot(normal, rim_dir), 0.0) * 0.25;
    let wire = max(
        smoothstep(0.018, 0.0, abs(e0)),
        max(smoothstep(0.018, 0.0, abs(e1)), smoothstep(0.018, 0.0, abs(e2)))
    );
    let depth = (a3.z + b3.z + c3.z) / 3.0;
    let value = clamp(light + wire * 0.55 + face_tint, 0.0, 1.0);

    return PyramidFaceHit(inside, value * 2.0 - 1.0, wire + depth * 0.4, depth);
}

fn pyramid_ring(p: vec2<f32>, time: f32, randoms: vec4<f32>) -> vec2<f32> {
    let ring_angle = time * (0.08 + randoms.z * 0.08) + randoms.x * 6.2831853;
    let tilt = 0.28 + randoms.y * 0.42;
    let major = 0.66 + randoms.x * 0.2;
    let minor = major * (0.22 + randoms.y * 0.2);
    let thickness = 0.035 + randoms.z * 0.055;
    let twist = sin(time * 0.17 + randoms.w * 6.2831853) * 0.28;
    let c = cos(ring_angle + twist);
    let s = sin(ring_angle + twist);
    let q = vec2<f32>(p.x * c - p.y * s, p.x * s + p.y * c);
    let ellipse_distance = abs(length(vec2<f32>(q.x / major, q.y / minor)) - 1.0);
    let ring = smoothstep(thickness, 0.0, ellipse_distance);
    let dash = 0.72 + 0.28 * sin(atan2(q.y / max(minor, 0.001), q.x / max(major, 0.001)) * (8.0 + randoms.w * 5.0) + time * 0.5);
    let depth_cut = smoothstep(-0.02, 0.08, q.y + tilt * 0.1);
    let value = ring * dash * (0.58 + depth_cut * 0.34);

    return vec2<f32>(value * 2.0 - 1.0, ring * (depth_cut * 2.0 - 1.0));
}

fn pyramid_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let center = vec2<f32>(0.5, 0.5);
    var p = (uv - center) * 2.15;
    p.x *= uniforms.resolution.x / uniforms.resolution.y;

    let randoms = pyramid_drift(time);
    let base_angles = pyramid_rotation_angles(time, randoms);

    let rotation_center = pyramid_center_for_rotation();
    let apex = pyramid_rotate(vec3<f32>(0.0, 0.58, 0.0) - rotation_center, base_angles);
    let b0 = pyramid_rotate(vec3<f32>(-0.52, -0.36, -0.52) - rotation_center, base_angles);
    let b1 = pyramid_rotate(vec3<f32>(0.52, -0.36, -0.52) - rotation_center, base_angles);
    let b2 = pyramid_rotate(vec3<f32>(0.52, -0.36, 0.52) - rotation_center, base_angles);
    let b3 = pyramid_rotate(vec3<f32>(-0.52, -0.36, 0.52) - rotation_center, base_angles);

    let ring = pyramid_ring(p, time, randoms);
    var best_value = ring.x;
    var best_gradient = ring.y;
    var best_depth = -2.0;

    let f0 = pyramid_face(p, apex, b0, b1, 0.06);
    if f0.inside && f0.depth > best_depth {
        best_value = f0.value;
        best_gradient = f0.gradient;
        best_depth = f0.depth;
    }

    let f1 = pyramid_face(p, apex, b1, b2, 0.0);
    if f1.inside && f1.depth > best_depth {
        best_value = f1.value;
        best_gradient = f1.gradient;
        best_depth = f1.depth;
    }

    let f2 = pyramid_face(p, apex, b2, b3, -0.04);
    if f2.inside && f2.depth > best_depth {
        best_value = f2.value;
        best_gradient = f2.gradient;
        best_depth = f2.depth;
    }

    let f3 = pyramid_face(p, apex, b3, b0, 0.02);
    if f3.inside && f3.depth > best_depth {
        best_value = f3.value;
        best_gradient = f3.gradient;
        best_depth = f3.depth;
    }

    if best_value <= -0.98 {
        return vec2<f32>(-1.0, -999.0);
    }

    return vec2<f32>(best_value, best_gradient);
}
