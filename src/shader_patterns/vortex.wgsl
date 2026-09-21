// Pattern: Vortex
// Spiral pattern combining angle and radius, in two variants that differ only in
// where the eye of the vortex sits

fn vortex_around(uv: vec2<f32>, eye: vec2<f32>, time: f32) -> vec2<f32> {
    let angle = atan2(uv.y - eye.y, uv.x - eye.x);
    let radius = distance(uv, eye);
    
    let value = sin(angle * uniforms.frequency + radius * 10.0 - time);
    let gradient = cos(angle * uniforms.frequency);
    
    return vec2<f32>(value, gradient);
}

// Eye in the middle of the screen at every scale.
fn vortex_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    return vortex_around(uv, PATTERN_CENTER, time);
}

// Eye at screen position 0.5 / scale, which is where scaling from the top-left
// corner puts it: it slides into that corner as scale grows past 1, so the screen
// shows the sweeping outer arms instead of a centred eye. Pattern space maps that
// screen position to `PATTERN_CENTER * (2.0 - scale)`.
fn vortex_corner_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    return vortex_around(uv, PATTERN_CENTER * (2.0 - uniforms.scale), time);
}
