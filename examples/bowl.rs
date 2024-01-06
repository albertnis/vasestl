use std::f32::consts::PI;

const T_RES: i32 = 300;
const H_STEP: f32 = 0.2;
const H_MAX: f32 = 30.0;

fn r(t: f32, z: f32) -> f32 {
    const R_SPHERE: f32 = 30.;
    let z_abs = z * R_SPHERE;

    let sphere = f32::sqrt(R_SPHERE.powi(2) - (z_abs - R_SPHERE).powi(2));

    let ease_in = -(f32::cos(PI * z) - 1.0) / 2.0;

    // // "Decoration": A sine wave to add a rippled texture
    let w_1 = f32::sin((t + z) * 20.0) * 0.6;

    let decoration = ease_in * (w_1);

    // Combine and return
    sphere + decoration
}
