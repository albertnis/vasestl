use std::f32::consts::PI;

const T_RES: i32 = 300;
const H_STEP: f32 = 0.2;
const H_MAX: f32 = 98.0;

fn r(t: f32, z: f32) -> f32 {
    // Pot is rectangle 70×60 at bottom; rectangle 97×87 at top

    const RECT_FROM: (f32, f32) = (70.0, 60.0);
    const RECT_TO: (f32, f32) = (97.0, 87.0);
    const BREATHING_ROOM: f32 = 3.0;

    let ease_in = -(f32::cos(PI * z) - 1.0) / 2.0;
    let ease_out = 1.0 - ease_in;

    // From rectangle
    let r_from_long = (RECT_FROM.0 + BREATHING_ROOM) / 2.0;
    let r_from_short = (RECT_FROM.1 + BREATHING_ROOM) / 2.0;
    let c1_from = f32::atan(r_from_short / r_from_long);

    // To rectangle
    let r_to_long = (RECT_TO.0 + BREATHING_ROOM) / 2.0;
    let r_to_short = (RECT_TO.1 + BREATHING_ROOM) / 2.0;
    let c1_to = f32::atan(r_to_short / r_to_long);

    let c1 = ease_out * c1_from + ease_in * c1_to;
    let (c2, c3, c4) = (PI - c1, PI + c1, 2. * PI - c1);
    let r_long = ease_out * r_from_long + ease_in * r_to_long;
    let r_short = ease_out * r_from_short + ease_in * r_to_short;

    let rectangle = if (c1..c2).contains(&t) || (c3..c4).contains(&t) {
        f32::abs(r_short / f32::sin(t))
    } else {
        f32::abs(r_long / f32::cos(t))
    };

    let bulbous = 1. + (0.5 * ease_out * f32::sin(z * PI));

    let base = rectangle * bulbous;

    // "Decoration": A sine wave to add a rippled texture
    let w_1 = 0.3 * f32::sin((t + z) * 80.0);
    let w_2 = f32::sin((t - z) * 80.0);

    let decoration = w_1 + w_2;

    // Combine and return
    base + decoration
}
