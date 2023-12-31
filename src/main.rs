use std::{fs::File, io::Write};

pub mod point_cloud;
pub mod stl;
pub mod triangles;

// Rotational resolution as denominator of Pi
// e.g. t_res=8 means a point every pi/8 radians, or 22.5 degrees
const T_RES: i32 = 600;

// Distance between each height ring, in mm
const H_STEP: f32 = 0.2;
const H_MAX: f32 = 100.0;

/// The fun part! The mathematical function r(θ, z) defining the object, using polar co-ordinates
///
/// `t` is the angle θ, in radians (from 0 to 2π)
///
/// `z` is the normalised z-height (from 0 to 1)
///
/// Return `r`, the distance from the origin for this point
fn r(t: f32, z: f32) -> f32 {
    // Cylinder of radius 100 (see the examples directory for more advanced designs)
    100.0
}


fn main() -> Result<(), std::io::Error> {
    // Location to save output
    const OUTPUT_LOCATION: &str = "out/output.stl";

    // Generate points from configuration and function
    let points = point_cloud::build(r, T_RES, H_MAX, H_STEP);
    println!(
        "Built {} points in {} layers",
        points.len() * points[0].len(),
        points.len()
    );

    // Generate surface from points
    let triangles = triangles::build(points, true);
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
