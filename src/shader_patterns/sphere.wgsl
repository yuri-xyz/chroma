// Pattern: Sphere
// Rotating 3D sphere with latitude/longitude grid

fn sphere_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    // Rotating 3D sphere (like Earth) - perspective projection
    let center = vec2<f32>(0.5, 0.5);
    let pos = (uv - center) * 2.0; // -1 to 1
    
    // Calculate if point is on sphere
    let dist_sq = pos.x * pos.x + pos.y * pos.y;
    
    if dist_sq > 1.0 {
        // Outside sphere - empty space
        return vec2<f32>(-1.0, 0.0);
    }
    
    // Calculate z coordinate (depth on sphere)
    let z = sqrt(1.0 - dist_sq);
    
    // 3D point on sphere surface
    let sphere_point = vec3<f32>(pos.x, pos.y, z);
    
    // Rotate sphere
    let rotation_speed = time * 0.5;
    let cos_t = cos(rotation_speed);
    let sin_t = sin(rotation_speed);
    
    // Rotate around Y axis
    let rotated_x = sphere_point.x * cos_t + sphere_point.z * sin_t;
    let rotated_z = -sphere_point.x * sin_t + sphere_point.z * cos_t;
    let rotated = vec3<f32>(rotated_x, sphere_point.y, rotated_z);
    
    // Create latitude/longitude grid (like Earth)
    let lat = asin(rotated.y);
    let lon = atan2(rotated.z, rotated.x);
    
    // Grid lines for latitude and longitude
    let lat_lines = sin(lat * uniforms.frequency * 3.0);
    let lon_lines = sin(lon * uniforms.frequency * 3.0);
    
    // Combine grid lines
    let grid_strength = max(
        smoothstep(0.15, 0.0, abs(lat_lines)),
        smoothstep(0.15, 0.0, abs(lon_lines))
    );
    
    // Add shading based on light direction (lighting from front-right)
    let light_dir = normalize(vec3<f32>(0.5, 0.3, 1.0));
    let normal = normalize(rotated);
    let diffuse = max(dot(normal, light_dir), 0.0);
    
    // Combine grid with shading
    let sphere_value = grid_strength * 0.7 + diffuse * 0.3;
    
    return vec2<f32>(sphere_value * 2.0 - 1.0, diffuse);
}
