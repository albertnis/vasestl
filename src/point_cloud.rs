use std::f32::consts::PI;

type Xyz = (f32, f32, f32);
type Xy = (f32, f32);

/// Build a cloud of points by iterating over angles and z-heights,
/// running a mathematical function `r` at each coordinate
///
/// Returns a vector of vectors. Each vector is a 2-dimensional "ring" of points
/// calculated for that z-height iteration.
pub fn build(r: fn(f32, f32) -> f32, t_res: i32, h_max: f32, h_step: f32) -> Vec<Vec<Xyz>> {
    let n_layers = f32::floor(h_max / h_step) as i32 + 1;
    println!("Building {n_layers} layers");
    (0..n_layers)
        .map(|i| {
            let z = (i as f32) * h_step;
            let z_f = z / h_max; // Normalise z to 0..1
            let points = build_point_ring(r, t_res, z_f)
                .iter()
                .map(|(x, y)| ((*x), (*y), z))
                .collect();

            points
        })
        .collect()
}

fn build_point_ring(r: fn(f32, f32) -> f32, t_res: i32, z: f32) -> Vec<Xy> {
    // `t_res` is denominator of pi
    // one revolution is 2*pi, so there will be 2*t_res points
    let n_points = t_res * 2;

    (0..n_points)
        .map(|i| {
            let t = (i as f32) * PI / (t_res as f32);

            let r = r(t, z);

            let x = r * f32::cos(t);
            let y = r * f32::sin(t);

            (x, y)
        })
        .collect()
}
