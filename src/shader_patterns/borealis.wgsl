// Pattern: Borealis
// Aurora curtains: folded ribbons of light with vertical rays rising from a
// bright hem, hung over a black, starlit sky. Bass (amplitude) stretches the
// rays upward and mids (frequency) pack them tighter.

fn borealis_noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);

    let a = simple_hash(i);
    let b = simple_hash(i + vec2<f32>(1.0, 0.0));
    let c = simple_hash(i + vec2<f32>(0.0, 1.0));
    let d = simple_hash(i + vec2<f32>(1.0, 1.0));

    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

fn borealis_fbm(p: vec2<f32>) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var point = p;

    for (var i = 0; i < 3; i++) {
        value += amplitude * borealis_noise(point);
        point = point * 2.03 + vec2<f32>(17.1, 9.7);
        amplitude *= 0.5;
    }

    // Normalise the three octaves back to 0..1.
    return value / 0.875;
}

// One curtain at `p` (centred and aspect-corrected; y grows downward).
// Returns (brightness, ray strength).
fn borealis_curtain(p: vec2<f32>, time: f32, layer: f32) -> vec2<f32> {
    let drift = time * (0.05 + layer * 0.02);

    // The curtain folds and sways, so its hem wanders up and down along x.
    let fold = borealis_fbm(vec2<f32>(p.x * 0.8 + layer * 3.7 - drift, time * 0.06 + layer * 1.3)) - 0.5;
    let sway = sin(p.x * 1.7 + time * 0.21 + layer * 2.1) * 0.05;
    let hem = 0.14 - layer * 0.13 + fold * 0.38 + sway;
    let above_hem = hem - p.y;

    // Rays follow the folds; frequency scales the centred x before the offsets.
    let ray_x = (p.x + fold * 0.45) * uniforms.frequency * 1.1 + layer * 11.0;
    let coarse_rays = borealis_noise(vec2<f32>(ray_x, time * 0.35 + layer * 5.0));
    let fine_rays = borealis_noise(vec2<f32>(ray_x * 2.7 + 3.1, time * 0.6 - layer * 2.0));
    let rays = smoothstep(0.25, 0.9, coarse_rays * 0.65 + fine_rays * 0.35);

    // Light ends sharply below the hem and fades slowly above it; louder bass
    // raises `amplitude` and with it how far the rays reach.
    let ray_height = 0.09 + 0.1 * uniforms.amplitude;
    let below = exp(-max(-above_hem, 0.0) * 30.0);
    let above = exp(-max(above_hem, 0.0) / ray_height);
    let hem_glow = exp(-abs(above_hem) * 30.0);

    // Some stretches of a curtain burn brighter than others.
    let segment = smoothstep(0.05, 0.6, borealis_noise(vec2<f32>(p.x * 1.3 - drift * 2.0, layer * 7.0 + time * 0.05)));

    let brightness = (below * above * (0.12 + 0.88 * pow(rays, 1.5)) + hem_glow * (0.3 + 0.5 * rays)) * segment * 1.35;

    return vec2<f32>(brightness, rays);
}

fn borealis_stars(uv: vec2<f32>, time: f32) -> f32 {
    let cell = floor(uv * uniforms.resolution);
    let seed = simple_hash(cell * 0.731 + vec2<f32>(13.7, 4.1));
    let twinkle = 0.55 + 0.45 * sin(time * (1.5 + seed * 3.0) + seed * 60.0);

    return step(0.982, seed) * twinkle;
}

fn borealis_pattern(uv: vec2<f32>, time: f32) -> vec2<f32> {
    var p = centered_uv(uv);
    p.x *= uniforms.resolution.x / uniforms.resolution.y;

    var total = 0.0;
    var layer_weighted = 0.0;
    var ray_weighted = 0.0;

    for (var i = 0; i < 3; i++) {
        let layer = f32(i);
        // The front curtain (lowest hem) is brightest.
        let curtain = borealis_curtain(p, time, layer) * vec2<f32>(1.0 - layer * 0.3, 1.0);

        total += curtain.x;
        layer_weighted += curtain.x * (layer - 1.0);
        ray_weighted += curtain.x * curtain.y;
    }

    // Stars only show through where the curtains are faint.
    let stars = borealis_stars(uv, time) * 0.5 * (1.0 - clamp(total * 3.0, 0.0, 1.0));
    let intensity = total + stars;

    // Empty sky: ask main.wgsl for a pure black backdrop.
    if intensity < 0.03 {
        return vec2<f32>(-1.0, -999.0);
    }

    pattern_coverage = smoothstep(0.03, 0.3, intensity);

    let weight = max(total, 1e-4);
    let gradient = (layer_weighted / weight) * 0.7 + (ray_weighted / weight) * 0.6 - 0.3;

    return vec2<f32>(clamp(intensity, 0.0, 1.0) * 2.0 - 1.0, gradient);
}
