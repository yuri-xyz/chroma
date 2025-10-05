// Diamonds
// Diamond lattice pattern

fn diamonds_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    // Diamond lattice - spacious
    let offset = vec2<f32>(sin(time * 0.3) * 0.2, cos(time * 0.4) * 0.2);
    let rotated = (uv + offset) * uniforms.frequency * 5.0;
    
    // Create diamond shape using Manhattan distance
    let diamond_x = abs(fract(rotated.x + rotated.y) - 0.5);
    let diamond_y = abs(fract(rotated.x - rotated.y) - 0.5);
    let diamond_dist = diamond_x + diamond_y;
    
    // Only show diamond edges, not fill
    let edge_thickness = 0.15;
    let diamond_strength = smoothstep(edge_thickness, 0.0, abs(diamond_dist - 0.5));
    
    return vec2<f32>(diamond_strength * 2.0 - 1.0, diamond_x - diamond_y);
}
