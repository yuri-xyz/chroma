// Beat-reactive distortion effects for visual "pop"

/// Apply beat-reactive zoom pulse
/// Creates smooth zoom in/out effect synchronized with beats
fn apply_beat_zoom(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let elapsed = time - uniforms.beat_distortion_time;
    
    // Only apply zoom for 0.6 seconds after beat
    if elapsed < 0.0 || elapsed > 0.6 || uniforms.beat_zoom_strength < 0.01 {
        return uv;
    }
    
    let strength = uniforms.beat_zoom_strength;
    
    // Fast zoom in, smooth zoom out (like a punch)
    let envelope = exp(-elapsed * 4.0);
    
    // Create zoom pulse: starts at 1.0, dips down (zoom in), returns to 1.0
    // Using sin wave for smooth in-out motion
    let zoom_curve = 1.0 - (sin(elapsed * 8.0) * envelope * strength * 0.15);
    
    // Zoom from center
    let center = vec2<f32>(0.5, 0.5);
    let offset = uv - center;
    
    return center + offset * zoom_curve;
}

/// Apply beat-reactive distortion to UV coordinates
/// Creates expanding ripple waves or diagonal waves that distort the underlying pattern
fn apply_beat_distortion(uv: vec2<f32>, time: f32) -> vec2<f32> {
    let elapsed = time - uniforms.beat_distortion_time;
    
    // Only apply distortion for 0.8 seconds after beat
    if elapsed < 0.0 || elapsed > 0.8 {
        return uv;
    }
    
    let strength = uniforms.beat_distortion_strength;
    
    // Fast attack, slow decay envelope
    let envelope = exp(-elapsed * 3.0); // Exponential decay
    
    let center = vec2<f32>(0.5, 0.5);
    
    // Ripple wave expanding from center (like dropping stone in water)
    let dist_from_center = distance(uv, center);
    
    // Multiple expanding ripples with different frequencies
    let ripple_frequency = 25.0;
    let expansion_speed = 1.5;
    let wave_radius = elapsed * expansion_speed;
    
    // Create multiple concentric waves
    let wave1 = sin((dist_from_center - wave_radius) * ripple_frequency);
    let wave2 = sin((dist_from_center - wave_radius) * ripple_frequency * 1.5 + 1.0);
    let wave_combined = (wave1 + wave2 * 0.5) / 1.5;
    
    // Falloff based on distance from wave front
    let wave_dist = abs(dist_from_center - wave_radius);
    let wave_falloff = smoothstep(0.4, 0.0, wave_dist);
    
    // Calculate distortion direction (radial from center)
    let direction = normalize(uv - center);
    
    // Apply radial distortion
    let distortion_amount = wave_combined * envelope * strength * wave_falloff * 0.08;
    var distorted_uv = uv + direction * distortion_amount;
    
    // Add secondary diagonal wave for variety
    let diagonal_phase = (uv.x + uv.y) * 15.0 - elapsed * 20.0;
    let diagonal_wave = sin(diagonal_phase) * 0.5 + 0.5;
    let diagonal_distortion = diagonal_wave * envelope * strength * 0.04;
    
    distorted_uv.x += sin(uv.y * 20.0 - elapsed * 15.0) * diagonal_distortion;
    distorted_uv.y += cos(uv.x * 20.0 - elapsed * 15.0) * diagonal_distortion;
    
    return distorted_uv;
}

/// Apply beat-reactive color flash/boost
/// Adds visual emphasis to beats through brightness/color changes
fn apply_beat_flash(color: vec3<f32>, position: vec2<f32>, time: f32) -> vec3<f32> {
    let elapsed = time - uniforms.beat_distortion_time;
    
    if elapsed < 0.0 || elapsed > 0.5 {
        return color;
    }
    
    let strength = uniforms.beat_distortion_strength;
    
    // Quick flash that fades fast
    let flash_envelope = exp(-elapsed * 8.0);
    
    // Radial gradient from center for more impact at center
    let center = vec2<f32>(0.5, 0.5);
    let dist_from_center = distance(position, center);
    let radial_falloff = 1.0 - smoothstep(0.0, 0.7, dist_from_center);
    
    // Brightness boost
    let brightness_boost = flash_envelope * strength * radial_falloff * 0.3;
    
    // Slight color shift pulse
    let hue_shift = sin(elapsed * 10.0) * flash_envelope * strength * 0.1;
    
    var boosted_color = color * (1.0 + brightness_boost);
    
    // Add slight color variation
    boosted_color.r += hue_shift;
    boosted_color.b += hue_shift * 0.5;
    
    return boosted_color;
}

