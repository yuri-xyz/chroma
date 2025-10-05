// Pattern: Glitch
// Digital glitch effect with scan lines

fn glitch_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let glitch_time = floor(time * 5.0) * 0.2;
    let row = floor(uv.y * uniforms.frequency * 3.0);
    let glitch_amount = simple_hash(vec2<f32>(row, glitch_time));
    
    var offset_uv = uv;
    if glitch_amount > 0.7 {
        offset_uv.x += (simple_hash(vec2<f32>(row * 2.0, glitch_time)) - 0.5) * 0.1;
    }
    
    let scan_line = sin(uv.y * uniforms.frequency * 50.0 + time * 10.0) * 0.5 + 0.5;
    let col_shift = abs(simple_hash(vec2<f32>(floor(offset_uv.x * uniforms.frequency * 2.0), glitch_time)) - 0.5);
    
    let blocks = step(0.5, simple_hash(floor(offset_uv * uniforms.frequency) + vec2<f32>(glitch_time, 0.0)));
    
    return vec2<f32>((scan_line * col_shift + blocks) * 2.0 - 1.0, scan_line);
}
