// Pattern: World
// 3D globe/earth slowly rotating - creates a spinning planet effect

fn world_hash(value: f32) -> f32 {
    return fract(sin(value * 127.1 + 311.7) * 43758.5453);
}

fn world_ring_seed() -> vec4<f32> {
    let seed = uniforms.color_shift * 13.17 +
        uniforms.frequency * 2.31 +
        uniforms.amplitude * 5.73 +
        uniforms.noise_scale * 913.0 +
        uniforms.z_rate * 271.0;

    return vec4<f32>(
        world_hash(seed + 1.0),
        world_hash(seed + 2.0),
        world_hash(seed + 3.0),
        world_hash(seed + 4.0)
    );
}

fn world_ring(diff: vec2<f32>, time: f32, globe_radius: f32, seed: vec4<f32>) -> vec3<f32> {
    if seed.x < 0.5 {
        return vec3<f32>(-1.0, 0.0, 0.0);
    }

    let spin_direction = select(-1.0, 1.0, seed.y > 0.5);
    let ring_angle = time * spin_direction * (0.06 + seed.z * 0.11) + seed.w * 6.2831853;
    let wobble = sin(time * (0.10 + seed.x * 0.05) + seed.y * 6.2831853) * (0.10 + seed.z * 0.18);
    let c = cos(ring_angle + wobble);
    let s = sin(ring_angle + wobble);
    let q = vec2<f32>(diff.x * c - diff.y * s, diff.x * s + diff.y * c);
    let major = globe_radius * (1.46 + seed.y * 0.38);
    let minor = major * (0.20 + seed.z * 0.22);
    let thickness = globe_radius * (0.035 + seed.w * 0.060);
    let band_offset = (seed.x - 0.5) * thickness * 1.4;
    let ellipse_distance = abs(length(vec2<f32>(q.x / major, (q.y + band_offset) / minor)) - 1.0);
    let ring = smoothstep(thickness, 0.0, ellipse_distance);
    let arc = atan2(q.y / max(minor, 0.001), q.x / max(major, 0.001));
    let grain = 0.76 + 0.24 * sin(arc * (7.0 + seed.z * 8.0) + time * spin_direction * (0.38 + seed.w * 0.35));
    let front = smoothstep(-0.04, 0.06, q.y + seed.x * 0.025);
    let value = ring * grain * (0.52 + front * 0.38);
    let gradient = ring * (front * 2.0 - 1.0 + seed.z * 0.35);

    return vec3<f32>(value, gradient, front);
}

fn world_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let center = vec2<f32>(0.5, 0.5);
    let diff = uv - center;
    let radius = length(diff);

    // Globe radius
    let globe_radius = 0.4;
    let ring_seed = world_ring_seed();
    let ring_result = world_ring(diff, time, globe_radius, ring_seed);

    // Outside the globe - empty dark space
    if radius > globe_radius {
        if ring_result.x > 0.01 {
            return vec2<f32>(ring_result.x, ring_result.y);
        }

        return vec2<f32>(-1.0, 0.0);
    }

    // Calculate 3D sphere coordinates
    // Map 2D position to 3D sphere surface
    let sphere_x = diff.x / globe_radius;
    let sphere_y = diff.y / globe_radius;
    let r2 = sphere_x * sphere_x + sphere_y * sphere_y;

    // Z coordinate on sphere surface
    let sphere_z = sqrt(max(1.0 - r2, 0.0));

    // Rotate around Y axis (longitude rotation)
    let rot_speed = time * 0.3;
    let rotated_x = sphere_x * cos(rot_speed) + sphere_z * sin(rot_speed);
    let rotated_z = -sphere_x * sin(rot_speed) + sphere_z * cos(rot_speed);

    // Convert to spherical coordinates for texture mapping
    let longitude = atan2(rotated_x, rotated_z);
    let latitude = asin(clamp(sphere_y, -1.0, 1.0));

    // Normalize to 0-1 range for pattern sampling
    let tex_u = (longitude / 3.14159265 + 1.0) * 0.5;
    let tex_v = (latitude / 1.5707963 + 1.0) * 0.5;

    let freq = uniforms.frequency * 0.5;

    // Create continent-like patterns
    // Layer 1: Large continental shapes
    let continent1 = sin(tex_u * 6.0 + 0.5) * cos(tex_v * 4.0 - 0.3);
    let continent2 = cos(tex_u * 8.0 - 1.0) * sin(tex_v * 5.0 + 0.7);
    let continents = smoothstep(-0.2, 0.3, continent1 + continent2 * 0.5);

    // Layer 2: Mountain ranges / terrain detail
    let terrain = sin(tex_u * freq * 15.0 + time * 0.1) *
                  cos(tex_v * freq * 12.0) * 0.3;

    // Layer 3: Ocean waves
    let ocean = sin(tex_u * 20.0 + time * 0.5) *
                sin(tex_v * 15.0 - time * 0.3) * 0.1;

    // Layer 4: Atmospheric glow at edges (limb darkening inverse)
    let limb = 1.0 - sphere_z;
    let atmosphere = pow(limb, 2.0) * 0.5;

    // Layer 5: Cloud layer (moves independently)
    let cloud_u = tex_u + time * 0.05;
    let clouds = sin(cloud_u * 12.0) * cos(tex_v * 8.0 + time * 0.1);
    let cloud_mask = smoothstep(0.3, 0.7, clouds) * 0.3;

    // Combine: land vs ocean
    let surface = mix(ocean, terrain + 0.3, continents);

    // Add clouds and atmosphere
    let combined = surface + cloud_mask + atmosphere;

    // Lighting: simple directional light from upper left
    let light_dir = normalize(vec3<f32>(-0.5, 0.5, 0.8));
    let normal = vec3<f32>(rotated_x, sphere_y, rotated_z);
    let lighting = max(dot(normal, light_dir), 0.0);

    // Apply lighting
    var lit_surface = combined * (0.3 + lighting * 0.7);

    // Gradient for color variation (land vs water)
    var gradient = continents - ocean + atmosphere;

    if ring_result.x > 0.01 && ring_result.z > 0.45 {
        let front_ring = ring_result.x * smoothstep(0.0, 0.45, ring_result.z);
        lit_surface = max(lit_surface, front_ring);
        gradient = mix(gradient, ring_result.y, clamp(front_ring, 0.0, 1.0));
    }

    return vec2<f32>(lit_surface, gradient);
}
