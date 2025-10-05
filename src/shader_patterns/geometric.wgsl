// Pattern Geometric
// Grid-based geometric patterns

fn geometric_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let grid_x = floor(uv.x * uniforms.frequency) + sin(time * 0.5);
    let grid_y = floor(uv.y * uniforms.frequency) + cos(time * 0.3);
    
    let value = simple_hash(vec2<f32>(grid_x, grid_y)) * 2.0 - 1.0;
    let gradient = fract(uv.x * uniforms.frequency) - 0.5;
    
    return vec2<f32>(value, gradient);
}
