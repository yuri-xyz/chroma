// Gravity scroll + mouse warp that fights the fall locally without canceling it.

fn apply_gravity_and_mouse(position: vec2<f32>) -> vec2<f32> {
    var warped = position + uniforms.gravity_offset;

    let influence = uniforms.mouse_influence;
    if influence > 0.001 && uniforms.gravity > 0.001 {
        let to_mouse = uniforms.mouse_position - position;
        let distance_sq = dot(to_mouse, to_mouse);
        // Local tug toward the cursor; kept subtle so global gravity still dominates.
        let pull = influence * 0.07 * exp(-distance_sq * 10.0);
        warped = warped + to_mouse * pull;
    }

    return warped;
}
