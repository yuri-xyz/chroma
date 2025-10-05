// Pattern: Voronoi
// Cell-based patterns using Voronoi diagram

fn voronoi_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let scale = uniforms.frequency * 2.0;
    let cell = floor(uv * scale);
    let fract_uv = fract(uv * scale);
    
    var min_dist = 10.0;
    var second_min = 10.0;
    
    for (var y = -1; y <= 1; y = y + 1) {
        for (var x = -1; x <= 1; x = x + 1) {
            let neighbor = vec2<f32>(f32(x), f32(y));
            let cell_pos = cell + neighbor;
            
            let point_offset = vec2<f32>(
                simple_hash(cell_pos + vec2<f32>(time * 0.1, 0.0)),
                simple_hash(cell_pos + vec2<f32>(0.0, time * 0.15))
            );
            
            let point = neighbor + point_offset - fract_uv;
            let dist = length(point);
            
            if dist < min_dist {
                second_min = min_dist;
                min_dist = dist;
            } else if dist < second_min {
                second_min = dist;
            }
        }
    }
    
    let edge = second_min - min_dist;
    return vec2<f32>(min_dist * 2.0 - 1.0, edge * 3.0);
}
