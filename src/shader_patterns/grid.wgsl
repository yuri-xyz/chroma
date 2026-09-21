// Pattern: Grid
// Flowing grid lines pattern

fn grid_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    // Flowing grid lines - spacious. The flow is a phase added after the frequency
    // product: inside it, the point a frequency change zooms about drifts away with
    // time, and every audio-driven change then jumps the grid along its flow.
    let centered = centered_uv(uv);
    let grid_x = sin(centered.x * uniforms.frequency * 8.0 + time * 9.6);
    let grid_y = sin(centered.y * uniforms.frequency * 8.0 + time * 13.44);
    
    // Only show grid lines, not fill squares
    let line_thickness = 0.15;
    let x_line = smoothstep(line_thickness, 0.0, abs(grid_x));
    let y_line = smoothstep(line_thickness, 0.0, abs(grid_y));
    
    let grid_strength = max(x_line, y_line);
    
    return vec2<f32>(grid_strength * 2.0 - 1.0, grid_x - grid_y);
}
