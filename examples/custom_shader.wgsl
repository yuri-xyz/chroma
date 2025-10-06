// Audio-Reactive Rainbow Tiles - A beginner-friendly custom shader example
// Creates colorful tiles that pulse and change with your music!
// - Bass (low sounds) makes tiles pulse brighter
// - Mids (vocals) shift the colors faster  
// - Treble (high sounds) increases tile detail

// IMPORTANT: Must match src/shader_common/uniforms.wgsl exactly (WGSL handles alignment automatically)
struct Uniforms {
    time: f32,
    resolution: vec2<f32>,
    frequency: f32,
    amplitude: f32,
    speed: f32,
    color_shift: f32,
    scale: f32,
    octaves: u32,
    noise_strength: f32,
    distort_amplitude: f32,
    noise_scale: f32,
    z_rate: f32,
    brightness: f32,
    contrast: f32,
    hue: f32,
    saturation: f32,
    gamma: f32,
    vignette: f32,
    vignette_softness: f32,
    glyph_sharpness: f32,
    color_mode: u32,
    pattern_type: u32,
    effect_time: f32,
    effect_type: u32,
    beat_distortion_time: f32,
    beat_distortion_strength: f32,
    beat_zoom_strength: f32,
    background_tint: vec3<f32>,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read_write> output_buffer: array<vec4<f32>>;

// Beginner-friendly custom pattern: Audio-Reactive Rainbow Tiles
// Creates a grid of tiles that pulse and change with music
fn custom_pattern(uv: vec2<f32>, time: f32) -> vec3<f32> {
    // Audio reactivity: frequency changes tile count (treble = more detail)
    // amplitude creates a pulse effect (bass = bigger pulses)
    let tile_size = uniforms.frequency * 0.5;
    let tile_x = floor(uv.x * tile_size);
    let tile_y = floor(uv.y * tile_size);
    
    // Create position within each tile (0.0 to 1.0)
    let tile_uv_x = fract(uv.x * tile_size);
    let tile_uv_y = fract(uv.y * tile_size);
    let center_dist = distance(vec2<f32>(tile_uv_x, tile_uv_y), vec2<f32>(0.5, 0.5));
    
    // Audio pulse effect: tiles pulse from center based on amplitude (bass)
    let pulse = 1.0 - (center_dist * 2.0);
    let audio_pulse = pulse * uniforms.amplitude;  // Bass makes this bigger!
    
    // Create a unique color for each tile
    let tile_id = tile_x + tile_y * 100.0;
    
    // Animate colors over time, with audio color shift (mids change colors faster)
    let t = time * uniforms.speed;
    let color_offset = tile_id * 0.3 + t + uniforms.color_shift * 3.0;
    
    // Create rainbow colors with audio pulse intensity
    let r = (sin(color_offset) * 0.5 + 0.5) * (0.5 + audio_pulse);
    let g = (sin(color_offset + 2.094) * 0.5 + 0.5) * (0.5 + audio_pulse);
    let b = (sin(color_offset + 4.189) * 0.5 + 0.5) * (0.5 + audio_pulse);
    
    // Apply brightness
    let brightness = uniforms.brightness;

    return vec3<f32>(r * brightness, g * brightness, b * brightness);
}

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let dimensions = vec2<u32>(u32(uniforms.resolution.x), u32(uniforms.resolution.y));

    if global_id.x >= dimensions.x || global_id.y >= dimensions.y {
        return;
    }

    let index = global_id.y * dimensions.x + global_id.x;

    let uv = vec2<f32>(
        f32(global_id.x) / uniforms.resolution.x,
        f32(global_id.y) / uniforms.resolution.y
    );

    let color = custom_pattern(uv * uniforms.scale, uniforms.time);

    output_buffer[index] = vec4<f32>(color, 1.0);
}

