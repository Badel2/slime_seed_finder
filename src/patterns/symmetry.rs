//! There are 24 rotational symmetries in 3D space, 48 if counting reflections.

use ndarray::Array2;

// Rotate 90 degrees counter-clockwise
fn rotate_2d_cc(c: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let size_x = c[0].len();
    let mut o = vec![vec![]; size_x];

    for (_j, d1) in c.into_iter().enumerate() {
        for (i, d2) in d1.into_iter().enumerate() {
            o[size_x - 1 - i].push(d2);
        }
    }

    o
}

// Rotate 90 degrees perpendicular to the y axis (counter-clockwise if looking from above)
fn rotate_3d_y(c: Vec<Vec<Vec<char>>>) -> Vec<Vec<Vec<char>>> {
    let mut o = vec![];

    // Each y-slice can be mapped independently
    for xz in c {
        let xz_rot = rotate_2d_cc(xz);
        o.push(xz_rot);
    }

    o
}

// Rotate 90 degrees perpendicular to the z axis (left side down and right side up, if looking from above)
fn rotate_3d_z(c: Vec<Vec<Vec<char>>>) -> Vec<Vec<Vec<char>>> {
    let (size_x, _size_y, size_z) = (c[0][0].len(), c.len(), c[0].len());
    let mut o = vec![vec![vec![]; size_z]; size_x];

    for (_k, d1) in c.into_iter().enumerate().rev() {
        for (j, d2) in d1.into_iter().enumerate() {
            for (i, d3) in d2.into_iter().enumerate() {
                o[i][j].push(d3);
            }
        }
    }

    o
}

// Rotate 90 degrees perpendicular to the x axis (top side up and bottom side down, if looking from above)
fn rotate_3d_x(c: Vec<Vec<Vec<char>>>) -> Vec<Vec<Vec<char>>> {
    let (_size_x, _size_y, size_z) = (c[0][0].len(), c.len(), c[0].len());
    let mut o = vec![vec![]; size_z];

    for (_k, d1) in c.into_iter().enumerate() {
        for (j, d2) in d1.into_iter().enumerate() {
            o[size_z - 1 - j].push(d2);
        }
    }

    o
}

// Reflect perpendicular to the y axis (xz plane)
fn reflect_3d_y(c: Vec<Vec<Vec<char>>>) -> Vec<Vec<Vec<char>>> {
    let mut o = c;

    o.reverse();

    o
}

fn identity_matrix() -> Array2<i8> {
    Array2::eye(3)
}

fn rotate_3d_x_matrix() -> Array2<i8> {
    Array2::from_shape_vec((3, 3), vec![1, 0, 0, 0, 0, -1, 0, 1, 0]).unwrap()
}

fn rotate_3d_y_matrix() -> Array2<i8> {
    Array2::from_shape_vec((3, 3), vec![0, 0, 1, 0, 1, 0, -1, 0, 0]).unwrap()
}

fn rotate_3d_z_matrix() -> Array2<i8> {
    Array2::from_shape_vec((3, 3), vec![0, -1, 0, 1, 0, 0, 0, 0, 1]).unwrap()
}

fn reflect_3d_y_matrix() -> Array2<i8> {
    Array2::from_shape_vec((3, 3), vec![1, 0, 0, 0, -1, 0, 0, 0, 1]).unwrap()
}

fn multiply_rotation_matrix_by(rotation: &Array2<i8>, x: &Array2<i8>) -> Array2<i8> {
    rotation.dot(x)
}

/// (nx, ny, nz, reflect_y)
fn decompose_rotation_matrix_into_xyz_rots(m: &Array2<i8>) -> (u8, u8, u8, bool) {
    let nx;
    let ny;
    let nz;
    let reflect_y = if determinant(&m) == -1 { -1 } else { 1 };

    let singular = m[(0, 0)] == 0 && m[(1, 0)] == 0;

    /// Return number of 90 degree rotations to get to the angle returned by atan2 called with the
    /// same arguments. Degrees => number of rotations: 0 => 0, 90 => 1, 180 => 2, -90 => 3.
    fn atan2nr(a: i8, b: i8) -> u8 {
        match (a, b) {
            (1, 0) => 1,
            (-1, 0) => 3,
            (0, 1) => 0,
            (0, -1) => 2,
            (0, 0) => 0,
            _ => unreachable!(),
        }
    }

    if singular {
        nx = atan2nr(m[(1, 2)] * reflect_y * -1, m[(1, 1)] * reflect_y);
        ny = atan2nr(m[(2, 0)] * -1, 0);
        nz = 0;
    } else {
        nx = atan2nr(m[(2, 1)], m[(2, 2)]);
        ny = 0;
        nz = atan2nr(m[(1, 0)] * reflect_y, m[(0, 0)]);
    }

    (nx, ny, nz, reflect_y == -1)
}

fn compose_rotation_matrix(nx: u8, ny: u8, nz: u8, reflect_y: bool) -> Array2<i8> {
    let mut m = identity_matrix();
    let rotx = rotate_3d_x_matrix();
    let roty = rotate_3d_y_matrix();
    let rotz = rotate_3d_z_matrix();
    let refly = reflect_3d_y_matrix();

    for _ in 0..nx {
        m = multiply_rotation_matrix_by(&rotx, &m);
    }
    for _ in 0..ny {
        m = multiply_rotation_matrix_by(&roty, &m);
    }
    for _ in 0..nz {
        m = multiply_rotation_matrix_by(&rotz, &m);
    }
    if reflect_y {
        m = multiply_rotation_matrix_by(&refly, &m);
    }

    m
}

fn compose_rotation_map(
    nx: u8,
    ny: u8,
    nz: u8,
    reflect_y: bool,
    mut map: Vec<Vec<Vec<char>>>,
) -> Vec<Vec<Vec<char>>> {
    for _ in 0..nx {
        map = rotate_3d_x(map);
    }
    for _ in 0..ny {
        map = rotate_3d_y(map);
    }
    for _ in 0..nz {
        map = rotate_3d_z(map);
    }
    if reflect_y {
        map = reflect_3d_y(map);
    }

    map
}

fn determinant(m: &Array2<i8>) -> i8 {
    let a = m[(0, 0)];
    let b = m[(0, 1)];
    let c = m[(0, 2)];
    let d = m[(1, 0)];
    let e = m[(1, 1)];
    let f = m[(1, 2)];
    let g = m[(2, 0)];
    let h = m[(2, 1)];
    let i = m[(2, 2)];

    fn det2(a: i8, b: i8, c: i8, d: i8) -> i8 {
        a * d - b * c
    }

    a * det2(e, f, h, i) - b * det2(d, f, g, i) + c * det2(d, e, g, h)
}

/// Given an orthogonal 3D rotation matrix, return the index into the list of 48 possible
/// rotations/reflections.
fn mat2idx(m: &Vec<i8>) -> u8 {
    let mut idx = 0;
    let b1;
    let b2;
    let mut c1 = 0;
    match &m[0..3] {
        &[1, 0, 0] => {
            idx += 0;
            b1 = 1;
            b2 = 2;
        }
        &[-1, 0, 0] => {
            idx += 40;
            b1 = 1;
            b2 = 2;
        }
        &[0, 1, 0] => {
            idx += 8;
            b1 = 0;
            b2 = 2;
        }
        &[0, -1, 0] => {
            idx += 32;
            b1 = 0;
            b2 = 2;
        }
        &[0, 0, 1] => {
            idx += 16;
            b1 = 0;
            b2 = 1;
        }
        &[0, 0, -1] => {
            idx += 24;
            b1 = 0;
            b2 = 1;
        }
        _ => unreachable!(),
    }

    match (m[3 + b1], m[3 + b2]) {
        (1, 0) => {
            c1 = b2;
            idx += 0;
        }
        (-1, 0) => {
            c1 = b2;
            idx += 6;
        }
        (0, 1) => {
            c1 = b1;
            idx += 2;
        }
        (0, -1) => {
            c1 = b1;
            idx += 4;
        }
        x => println!("{:?}", x),
    }

    match m[6 + c1] {
        1 => {}
        -1 => {
            idx += 1;
        }
        _ => unreachable!(),
    }

    idx
}

fn idx2mat(idx: u8) -> Array2<i8> {
    let mut v = vec![0; 9];
    let b1;
    let b2;
    let c1;

    match idx / 8 {
        0 => {
            v[0] = 1;
            b1 = 1;
            b2 = 2;
        }
        5 => {
            v[0] = -1;
            b1 = 1;
            b2 = 2;
        }
        1 => {
            v[1] = 1;
            b1 = 0;
            b2 = 2;
        }
        4 => {
            v[1] = -1;
            b1 = 0;
            b2 = 2;
        }
        2 => {
            v[2] = 1;
            b1 = 0;
            b2 = 1;
        }
        3 => {
            v[2] = -1;
            b1 = 0;
            b2 = 1;
        }
        _ => unreachable!(),
    }

    match (idx % 8) / 2 {
        0 => {
            v[3 + b1] = 1;
            c1 = b2;
        }
        3 => {
            v[3 + b1] = -1;
            c1 = b2;
        }
        1 => {
            v[3 + b2] = 1;
            c1 = b1;
        }
        2 => {
            v[3 + b2] = -1;
            c1 = b1;
        }
        _ => unreachable!(),
    }

    match idx % 2 {
        0 => {
            v[6 + c1] = 1;
        }
        1 => {
            v[6 + c1] = -1;
        }
        _ => unreachable!(),
    }

    Array2::from_shape_vec((3, 3), v).unwrap()
}

/// Return the maximum vertical size of a pattern, taking into account rotations.
pub fn max_y_size(dims: (u32, u32, u32), rot_idx: &[u8]) -> u32 {
    let mut max_y = dims.1;

    for idx in rot_idx {
        let mat = idx2mat(*idx);
        let vec = Array2::from_shape_vec((1, 3), vec![0, 1, 0]).unwrap();
        let rot_vec = multiply_rotation_matrix_by(&mat, &vec);

        if rot_vec[(0, 0)] != 0 {
            max_y = std::cmp::max(max_y, dims.0);
        }
        if rot_vec[(0, 2)] != 0 {
            max_y = std::cmp::max(max_y, dims.2);
        }
    }

    max_y
}

pub fn get_rotated_maps(map: &Vec<Vec<Vec<char>>>, rot_idx: &[u8]) -> Vec<Vec<Vec<Vec<char>>>> {
    let mut maps = vec![];

    for idx in rot_idx {
        let m = idx2mat(*idx);
        let (nx, ny, nz, reflect_y) = decompose_rotation_matrix_into_xyz_rots(&m);
        let fm = compose_rotation_map(nx, ny, nz, reflect_y, map.clone());
        maps.push(fm);
    }

    maps
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::patterns::parse_block_pattern_map;
    use rand::{thread_rng, Rng as _};
    use std::collections::HashMap;
    use std::collections::HashSet;

    fn call_n<T, F: FnMut(T) -> T>(n: usize, mut f: F, mut arg: T) -> T {
        for _ in 0..n {
            arg = f(arg);
        }

        arg
    }

    #[test]
    fn test_rotate_3d_y() {
        // Rotate 90 degrees perpendicular to the y axis (counter-clockwise if looking from above)
        let x1 = parse_block_pattern_map("ABCD,EFGH,IJKL;abcd,efgh,ijkl").unwrap();
        let x2 = parse_block_pattern_map("DHL,CGK,BFJ,AEI;dhl,cgk,bfj,aei").unwrap();

        assert_eq!(rotate_3d_y(x1.clone()), x2);
        assert_eq!(call_n(3, rotate_3d_y, x2), x1);
    }

    #[test]
    fn test_rotate_3d_z() {
        // Rotate 90 degrees perpendicular to the z axis (left side down and right side up, if looking from above)
        let x1 = parse_block_pattern_map("ABCD,EFGH,IJKL;abcd,efgh,ijkl").unwrap();
        let x2 = parse_block_pattern_map("Dd,Hh,Ll;Cc,Gg,Kk;Bb,Ff,Jj;Aa,Ee,Ii").unwrap();

        assert_eq!(rotate_3d_z(x1.clone()), x2);
        assert_eq!(call_n(3, rotate_3d_z, x2), x1);
    }

    #[test]
    fn test_rotate_3d_x() {
        // Rotate 90 degrees perpendicular to the x axis (top side up and bottom side down, if looking from above)
        let x1 = parse_block_pattern_map("ABCD,EFGH,IJKL;abcd,efgh,ijkl").unwrap();
        let x2 = parse_block_pattern_map("abcd,ABCD;efgh,EFGH;ijkl,IJKL").unwrap();

        assert_eq!(rotate_3d_x(x1.clone()), x2);
        assert_eq!(call_n(3, rotate_3d_x, x2), x1);
    }

    #[test]
    fn test_reflect_3d_y() {
        // Reflect perpendicular to the y axis (xz plane)
        let x1 = parse_block_pattern_map("ABCD,EFGH,IJKL;abcd,efgh,ijkl").unwrap();
        let x2 = parse_block_pattern_map("abcd,efgh,ijkl;ABCD,EFGH,IJKL").unwrap();

        assert_eq!(reflect_3d_y(x1.clone()), x2);
        assert_eq!(reflect_3d_y(x2), x1);
    }

    #[test]
    fn the_24_rotations() {
        let mut hs = HashSet::new();
        let x1 = parse_block_pattern_map("ABCD,EFGH,IJKL;abcd,efgh,ijkl").unwrap();
        let mut x = x1.clone();

        while hs.len() < 24 {
            // Rotate randomly
            match thread_rng().gen_range(0..=1) {
                0 => {
                    x = rotate_3d_x(x);
                }
                _ => {
                    x = rotate_3d_y(x);
                }
            }
            hs.insert(x.clone());
        }
    }

    #[test]
    fn the_48_rotations_and_reflections() {
        let mut hs = HashSet::new();
        let x1 = parse_block_pattern_map("ABCD,EFGH,IJKL;abcd,efgh,ijkl").unwrap();
        let mut x = x1.clone();

        while hs.len() < 48 {
            // Rotate randomly
            match thread_rng().gen_range(0..=2) {
                0 => {
                    x = rotate_3d_x(x);
                }
                1 => {
                    x = rotate_3d_y(x);
                }
                _ => {
                    x = reflect_3d_y(x);
                }
            }
            hs.insert(x.clone());
        }
    }

    #[test]
    fn the_48_rotations_and_reflections_matrix() {
        let mut h = HashMap::new();
        let x1 = parse_block_pattern_map("ABCD,EFGH,IJKL;abcd,efgh,ijkl").unwrap();
        let mut x = x1.clone();
        let mut m = identity_matrix();
        let rotx = rotate_3d_x_matrix();
        let roty = rotate_3d_y_matrix();
        let refly = reflect_3d_y_matrix();

        while h.len() < 48 {
            // Rotate randomly
            match thread_rng().gen_range(0..=2) {
                0 => {
                    x = rotate_3d_x(x);
                    m = multiply_rotation_matrix_by(&rotx, &m);
                }
                1 => {
                    x = rotate_3d_y(x);
                    m = multiply_rotation_matrix_by(&roty, &m);
                }
                _ => {
                    x = reflect_3d_y(x);
                    m = multiply_rotation_matrix_by(&refly, &m);
                }
            }
            let old_m = h.insert(x.clone(), m.clone());
            if old_m.is_some() && old_m.as_ref().unwrap() != m {
                panic!("Bug in matrix code: {:?} != {:?}", old_m, m);
            }
        }

        let mut possible_matrix_list: Vec<_> = h
            .values()
            .map(|m| {
                let x: Vec<_> = m.iter().cloned().collect();
                x
            })
            .collect();
        // Reverse sort
        possible_matrix_list.sort_by(|a, b| b.cmp(a));
        //panic!("Done: {:?}", possible_matrix_list);
        // Test mat2idx and idx2mat functions
        for (i, m) in possible_matrix_list.iter().enumerate() {
            assert_eq!(mat2idx(&m) as usize, i);
            assert_eq!(
                idx2mat(i as u8),
                Array2::from_shape_vec((3, 3), m.clone()).unwrap()
            );
        }
    }

    #[test]
    fn rotation_matrix_decomposition() {
        for idx in 0..48 {
            let m = idx2mat(idx);
            let (nx, ny, nz, reflect_y) = decompose_rotation_matrix_into_xyz_rots(&m);
            let fm = compose_rotation_matrix(nx, ny, nz, reflect_y);
            assert_eq!(fm, m, "Fail at {}: {:?} != {:?}", idx, fm, m);
        }
    }

    #[test]
    fn test_det() {
        let m1 = Array2::from_shape_vec((3, 3), vec![1, 0, 0, 0, 1, 0, 0, 0, 1]).unwrap();
        assert_eq!(determinant(&m1), 1);
        let m2 = Array2::from_shape_vec((3, 3), vec![1, 0, 0, 0, -1, 0, 0, 0, 1]).unwrap();
        assert_eq!(determinant(&m2), -1);
    }
}
