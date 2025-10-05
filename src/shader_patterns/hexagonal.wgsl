// Pattern: Hexagonal
// Hexagonal grid pattern

fn hexagonal_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let scale = uniforms.frequency;
    let hex_uv = uv * scale * vec2<f32>(1.0, 0.866);
    
    let q = vec2<f32>(hex_uv.x * 1.1547, hex_uv.y - hex_uv.x * 0.5773);
    let p = vec2<f32>(floor(q.x), floor(q.y));
    let r = fract(q);
    
    var hex_center: vec2<f32>;
    if r.x + r.y > 1.0 {
        hex_center = vec2<f32>(1.0 - r.x, 1.0 - r.y);
    } else {
        hex_center = r;
    }
    
    let cell_id = p + vec2<f32>(time * 0.1, 0.0);
    let hash_val = simple_hash(cell_id);
    let hex_dist = length(hex_center - vec2<f32>(0.5)) * 2.0;
    
    return vec2<f32>(sin(hex_dist * 6.28 + hash_val * 6.28) * (1.0 - hex_dist), hex_dist);
}
