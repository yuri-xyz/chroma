// Pattern Fractal
// Iterative fractal pattern with rotations

fn fractal_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    var z = (uv - 0.5) * 3.0;
    var value = 0.0;
    var gradient = 0.0;
    
    for (var i = 0; i < 5; i = i + 1) {
        let fi = f32(i);
        let angle = time * 0.1 * (1.0 + fi * 0.1);
        let s = sin(angle);
        let c = cos(angle);
        z = vec2<f32>(z.x * c - z.y * s, z.x * s + z.y * c);
        
        z = abs(z);
        z = z * uniforms.frequency * 0.5 - vec2<f32>(1.0, 0.5);
        
        let d = length(z);
        value += exp(-d * 2.0) / (1.0 + fi);
        gradient += d / (1.0 + fi);
    }
    
    return vec2<f32>(value * 2.0 - 1.0, gradient * 0.5);
}
