// Pattern: World
// 3D globe/earth slowly rotating - creates a spinning planet effect

fn world_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let center = vec2<f32>(0.5, 0.5);
    let diff = uv - center;
    let radius = length(diff);

    // Globe radius
    let globe_radius = 0.4;

    // Outside the globe - empty dark space
    if radius > globe_radius {
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
    let lit_surface = combined * (0.3 + lighting * 0.7);

    // Gradient for color variation (land vs water)
    let gradient = continents - ocean + atmosphere;

    return vec2<f32>(lit_surface, gradient);
}
