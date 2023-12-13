type Xyz = (f32, f32, f32);
type Triangle = (Xyz, Xyz, Xyz, Xyz);

/// Build binary-mode STL file from triangles
pub fn build(triangles: Vec<Triangle>) -> Vec<u8> {
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
    let t_0_0_bytes = t.0 .0.to_le_bytes();
    let t_0_1_bytes = t.0 .1.to_le_bytes();
    let t_0_2_bytes = t.0 .2.to_le_bytes();
    let t_1_0_bytes = t.1 .0.to_le_bytes();
    let t_1_1_bytes = t.1 .1.to_le_bytes();
    let t_1_2_bytes = t.1 .2.to_le_bytes();
    let t_2_0_bytes = t.2 .0.to_le_bytes();
    let t_2_1_bytes = t.2 .1.to_le_bytes();
    let t_2_2_bytes = t.2 .2.to_le_bytes();
    let t_3_0_bytes = t.3 .0.to_le_bytes();
    let t_3_1_bytes = t.3 .1.to_le_bytes();
    let t_3_2_bytes = t.3 .2.to_le_bytes();
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
