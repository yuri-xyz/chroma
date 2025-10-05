// Pattern: Grid
// Flowing grid lines pattern

fn grid_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    // Flowing grid lines - spacious
    let flow_speed = time * 0.3;
    let grid_x = sin((uv.x + flow_speed * 0.5) * uniforms.frequency * 8.0);
    let grid_y = sin((uv.y + flow_speed * 0.7) * uniforms.frequency * 8.0);
    
    // Only show grid lines, not fill squares
    let line_thickness = 0.15;
    let x_line = smoothstep(line_thickness, 0.0, abs(grid_x));
    let y_line = smoothstep(line_thickness, 0.0, abs(grid_y));
    
    let grid_strength = max(x_line, y_line);
    
    return vec2<f32>(grid_strength * 2.0 - 1.0, grid_x - grid_y);
}
