// Pattern: Spiral
// Spiral arms pattern

fn spiral_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let center = vec2<f32>(0.5, 0.5);
    let diff = uv - center;
    let radius = length(diff);
    let angle = atan2(diff.y, diff.x);
    
    // Spiral arms - spacious, not filled
    let num_arms = 5.0;
    let spiral_angle = angle * num_arms + radius * uniforms.frequency * 3.0 - time;
    let spiral_val = sin(spiral_angle);
    
    // Only show the arms, not fill the space
    let arm_thickness = 0.3;
    let arm_strength = smoothstep(arm_thickness, 0.0, abs(spiral_val));
    
    return vec2<f32>(arm_strength * 2.0 - 1.0, spiral_val);
}
