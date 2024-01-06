use std::f32::consts::PI;

const T_RES: i32 = 600;
const H_STEP: f32 = 0.2;
const H_MAX: f32 = 104.0;

fn r(t: f32, z: f32) -> f32 {
    // Pot is circle ø44 at bottom; square 104×104 at top

    const SQUARE_SIZE: f32 = 52.0; // Half the width (minimum "radius") of square
    let square = if (PI * 0.25..PI * 0.75).contains(&t) || (PI * 1.25..PI * 1.75).contains(&t) {
        f32::abs(SQUARE_SIZE / f32::sin(t))
    } else {
        f32::abs(SQUARE_SIZE / f32::cos(t))
    };

    const CIRCLE: f32 = 44.0; // Radius of circle

    let z_ease = -(f32::cos(PI * z) - 1.0) / 2.0;
    let base = CIRCLE * (1.0 - z_ease) + square * z_ease;

    // "Decoration": A sine wave to add a rippled texture
    let wave_1 = f32::sin((t + 0.5 * z) * 80.0) * 1.0;

    // Combine and return
    base + wave_1
}
