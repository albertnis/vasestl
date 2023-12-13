use std::f32::consts::PI;
use std::{fs::File, io::Write};

type Xyz = (f32, f32, f32);
type Xy = (f32, f32);
type Triangle = (Xyz, Xyz, Xyz, Xyz);

fn main() {
    // Rotational resolution as denominator of Pi
    // e.g. t_res=8 means a point every pi/8 radians, or 22.5 degrees
    const T_RES: i32 = 600;

    // Distance between each height ring, in mm
    const H_STEP: f32 = 0.2;
    const H_MAX: f32 = 100.0;

    // Location to save output
    const OUTPUT_LOCATION: &str = "out/output.stl";

    let points: Vec<Vec<Xyz>> = build_point_cloud(T_RES, H_MAX, H_STEP);
    let n_points = points.len();
    println!("Built {n_points} points");

    let triangles: Vec<Triangle> = triangles_from_points(points);
    let n_triangles = triangles.len();
    println!("Built {n_triangles} triangles");

    let stl_content = build_stl_file(triangles);
    let filesize_megabytes = (stl_content.len() as f32) / 1024.0 / 1024.0;
    println!("Generated {filesize_megabytes} MB STL file");

    let path = std::path::Path::new(OUTPUT_LOCATION);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(&stl_content).unwrap();
    println!("Wrote file to {OUTPUT_LOCATION}");
}

fn build_point_cloud(t_res: i32, h_max: f32, h_step: f32) -> Vec<Vec<Xyz>> {
    let n_layers = f32::floor(h_max / h_step) as i32 + 1;
    println!("Building {n_layers} layers");
    (0..n_layers)
        .map(|i| {
            let z = (i as f32) * h_step;
            let z_f = z / h_max; // Normalise z to 0..1
            let points = build_point_ring(t_res, z_f)
                .iter()
                .map(|(x, y)| ((*x), (*y), z))
                .collect();

            points
        })
        .collect()
}

fn build_point_ring(t_res: i32, z: f32) -> Vec<Xy> {
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

/// The fun part! The mathematical function defining the object, using polar co-ordinates
/// 
/// `t` is the angle, in radians
/// 
/// `z` is the normalised z-height (from 0 to 1)
/// 
/// Return `r`, the distance from the origin for this point
fn r(t: f32, z: f32) -> f32 {
    const CIRCLE: f32 = 42.0;
    const SQUARE_SIZE: f32 = 51.0;

    let square = if (PI * 0.25..PI * 0.75).contains(&t) {
        SQUARE_SIZE / f32::sin(t)
    } else if (PI * 0.75..PI * 1.25).contains(&t) {
        -SQUARE_SIZE / f32::cos(t)
    } else if (PI * 1.25..PI * 1.75).contains(&t) {
        -SQUARE_SIZE / f32::sin(t)
    } else {
        SQUARE_SIZE / f32::cos(t)
    };

    let z_ease = -(f32::cos(PI * z) - 1.0) / 2.0;

    let base = CIRCLE * (1.0 - z_ease) + square * z_ease;

    let wave_1 = f32::sin((t + 0.5*z) * 80.0) * 1.0;
    // let wave_2 = f32::sin((t - z) * 80.0) * 1.0;

    base + wave_1
}

/// Generate the triangles defining a surface on which all points lie
fn triangles_from_points(rings: Vec<Vec<Xyz>>) -> Vec<Triangle> {
    let mut output: Vec<Triangle> =
        Vec::with_capacity((rings.len() - 1) * (rings[0].len() - 1) * 2);

    for i in 0..(rings.len() - 1) {
        for j in 0..rings[i].len() {
            let j_next = if j == (rings[i].len() - 1) { 0 } else { j + 1 };

            let p1: Xyz = rings[i][j];
            let p2: Xyz = rings[i][j_next];
            let p3: Xyz = rings[i + 1][j_next];
            let p4: Xyz = rings[i + 1][j];

            let tri1: Triangle = triangle_from_points((p1, p2, p4));
            let tri2: Triangle = triangle_from_points((p2, p3, p4));

            output.push(tri1);
            output.push(tri2);
        }
    }

    output
}

/// For a triangle defined by three points, compute the cross-product normal
/// Return the same triangle but with normal data included as the first element
fn triangle_from_points((p1, p2, p3): (Xyz, Xyz, Xyz)) -> Triangle {
    // A = p2 - p1, B = p3 - p1
    // Nx = Ay * Bz - Az * By
    // Ny = Az * Bx - Ax * Bz
    // Nz = Ax * By - Ay * Bx
    let a = (p2.0 - p1.0, p2.1 - p1.1, p2.2 - p1.2);
    let b = (p3.0 - p1.0, p3.1 - p1.1, p3.2 - p1.2);

    let nx = a.1 * b.2 - a.2 * b.1;
    let ny = a.2 * b.0 - a.0 * b.2;
    let nz = a.0 * b.1 - a.1 * b.0;

    ((nx, ny, nz), p1, p2, p3)
}

/// Build binary-mode STL file from triangles
fn build_stl_file(triangles: Vec<Triangle>) -> Vec<u8> {
    let n_triangles = triangles.len() as u32;

    let file_size = 80 + 4 + 50 * n_triangles;
    let mut data = Vec::with_capacity(file_size as usize);

    let header_data: [u8; 80] = [0x00; 80];
    data.extend(header_data);

    let triangle_count_data = n_triangles.to_le_bytes();
    data.extend(triangle_count_data);

    let triangle_data = triangles.iter().flat_map(triangle_to_stl_data);
    data.extend(triangle_data);

    data
}

/// Dump and pad triangle data into the 50-byte sequence used by the STL file
fn triangle_to_stl_data(t: &Triangle) -> [u8; 50] {
    let t_0_0_bytes = t.0.0.to_le_bytes();
    let t_0_1_bytes = t.0.1.to_le_bytes();
    let t_0_2_bytes = t.0.2.to_le_bytes();
    let t_1_0_bytes = t.1.0.to_le_bytes();
    let t_1_1_bytes = t.1.1.to_le_bytes();
    let t_1_2_bytes = t.1.2.to_le_bytes();
    let t_2_0_bytes = t.2.0.to_le_bytes();
    let t_2_1_bytes = t.2.1.to_le_bytes();
    let t_2_2_bytes = t.2.2.to_le_bytes();
    let t_3_0_bytes = t.3.0.to_le_bytes();
    let t_3_1_bytes = t.3.1.to_le_bytes();
    let t_3_2_bytes = t.3.2.to_le_bytes();
    [
        t_0_0_bytes[0],
        t_0_0_bytes[1],
        t_0_0_bytes[2],
        t_0_0_bytes[3],
        t_0_1_bytes[0],
        t_0_1_bytes[1],
        t_0_1_bytes[2],
        t_0_1_bytes[3],
        t_0_2_bytes[0],
        t_0_2_bytes[1],
        t_0_2_bytes[2],
        t_0_2_bytes[3],
        t_1_0_bytes[0],
        t_1_0_bytes[1],
        t_1_0_bytes[2],
        t_1_0_bytes[3],
        t_1_1_bytes[0],
        t_1_1_bytes[1],
        t_1_1_bytes[2],
        t_1_1_bytes[3],
        t_1_2_bytes[0],
        t_1_2_bytes[1],
        t_1_2_bytes[2],
        t_1_2_bytes[3],
        t_2_0_bytes[0],
        t_2_0_bytes[1],
        t_2_0_bytes[2],
        t_2_0_bytes[3],
        t_2_1_bytes[0],
        t_2_1_bytes[1],
        t_2_1_bytes[2],
        t_2_1_bytes[3],
        t_2_2_bytes[0],
        t_2_2_bytes[1],
        t_2_2_bytes[2],
        t_2_2_bytes[3],
        t_3_0_bytes[0],
        t_3_0_bytes[1],
        t_3_0_bytes[2],
        t_3_0_bytes[3],
        t_3_1_bytes[0],
        t_3_1_bytes[1],
        t_3_1_bytes[2],
        t_3_1_bytes[3],
        t_3_2_bytes[0],
        t_3_2_bytes[1],
        t_3_2_bytes[2],
        t_3_2_bytes[3],
        0x00,
        0x00,
    ]
}
