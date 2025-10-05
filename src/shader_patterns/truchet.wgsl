// Pattern: Truchet
// Truchet tile patterns with rotations

fn truchet_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let scale = uniforms.frequency;
    let cell = floor(uv * scale);
    let local = fract(uv * scale);
    
    let hash_val = simple_hash(cell + vec2<f32>(time * 0.05, 0.0));
    let rotation = floor(hash_val * 4.0);
    
    var rotated: vec2<f32>;
    if rotation < 1.0 {
        rotated = local;
    } else if rotation < 2.0 {
        rotated = vec2<f32>(1.0 - local.y, local.x);
    } else if rotation < 3.0 {
        rotated = vec2<f32>(1.0 - local.x, 1.0 - local.y);
    } else {
        rotated = vec2<f32>(local.y, 1.0 - local.x);
    }
    
    let arc1 = length(rotated - vec2<f32>(0.0, 0.0));
    let arc2 = length(rotated - vec2<f32>(1.0, 1.0));
    let pattern = min(abs(arc1 - 0.5), abs(arc2 - 0.5));
    
    return vec2<f32>(pattern * 4.0 - 1.0, pattern * 2.0);
}
