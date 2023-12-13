use std::f32::consts::PI;
use std::{fs::File, io::Write};

pub mod point_cloud;
pub mod stl;
pub mod triangles;

fn main() -> Result<(), std::io::Error> {
    // Rotational resolution as denominator of Pi
    // e.g. t_res=8 means a point every pi/8 radians, or 22.5 degrees
    const T_RES: i32 = 600;

    // Distance between each height ring, in mm
    const H_STEP: f32 = 0.2;
    const H_MAX: f32 = 100.0;

    // Location to save output
    const OUTPUT_LOCATION: &str = "out/output.stl";

    // Generate points from configuration and function
    let points = point_cloud::build(r, T_RES, H_MAX, H_STEP);
    println!("Built {} points", points.len());

    // Generate surface from points
    let triangles = triangles::build(points);
    println!("Built {} triangles", triangles.len());

    // Generate STL file from surface
    let stl_content = stl::build(triangles);
    println!(
        "Generated {} MB STL file",
        (stl_content.len() as f32) / 1024.0 / 1024.0
    );

    // Write STL to disk
    let path = std::path::Path::new(OUTPUT_LOCATION);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix)?;
    let mut file = File::create(path)?;
    file.write_all(&stl_content)?;
    println!("Wrote file to {OUTPUT_LOCATION}");

    Ok(())
}

/// The fun part! The mathematical function r(θ, z) defining the object, using polar co-ordinates
///
/// `t` is the angle θ, in radians (from 0 to 2π)
///
/// `z` is the normalised z-height (from 0 to 1)
///
/// Return `r`, the distance from the origin for this point
fn r(t: f32, z: f32) -> f32 {
    // "Base shape": a circle sweeping to a square
    
    const SQUARE_SIZE: f32 = 51.0; // Half the width (minimum "radius") of square
    let square = if (PI * 0.25..PI * 0.75).contains(&t) || (PI * 1.25..PI * 1.75).contains(&t) {
        f32::abs(SQUARE_SIZE / f32::sin(t))
    } else {
        f32::abs(SQUARE_SIZE / f32::cos(t))
    };

    const CIRCLE: f32 = 42.0; // Radius of circle

    let z_ease = -(f32::cos(PI * z) - 1.0) / 2.0;
    let base = CIRCLE * (1.0 - z_ease) + square * z_ease;

    // "Decoration": A sine wave to add a rippled texture
    let wave_1 = f32::sin((t + 0.5 * z) * 80.0) * 1.0;

    // Combine and return
    base + wave_1
}
