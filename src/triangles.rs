type Xyz = (f32, f32, f32);
type Triangle = (Xyz, Xyz, Xyz, Xyz);

/// Generate the triangles defining a surface on which all points lie
pub fn build(points: Vec<Vec<Xyz>>) -> Vec<Triangle> {
    let mut output: Vec<Triangle> =
        Vec::with_capacity((points.len() - 1) * (points[0].len() - 1) * 2);

    for i in 0..(points.len() - 1) {
        for j in 0..points[i].len() {
            let j_next = if j == (points[i].len() - 1) { 0 } else { j + 1 };

            let p1: Xyz = points[i][j];
            let p2: Xyz = points[i][j_next];
            let p3: Xyz = points[i + 1][j_next];
            let p4: Xyz = points[i + 1][j];

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
