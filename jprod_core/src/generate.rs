use crate::{
    math,
    pool::{AllocationArrayToken, Pool},
    utils,
};
use math::Vec4;

#[inline]
pub fn quad(
    pool: &mut Pool,
) -> (
    AllocationArrayToken<[f32; 3]>,
    AllocationArrayToken<[f32; 3]>,
) {
    let vertices = 4;

    let mut quad_token = pool.allocate_array(vertices, [0.0, 0.0, 0.0]);
    let mut normals_token = pool.allocate_array(vertices, [0.0, 0.0, 0.0]);

    let quad = pool.borrow_slice_mut(&mut quad_token);
    let normals = pool.borrow_slice_mut(&mut normals_token);

    let verts: [(i8, i8); 4] = [(1, 1), (-1, 1), (1, -1), (-1, -1)];

    for (i, (vert, normal)) in quad.iter_mut().zip(normals.iter_mut()).enumerate() {
        let v = unsafe { verts.get_unchecked(i) };

        *vert = [v.0 as f32, v.1 as f32, 0.0];
        *normal = [0.0, 0.0, 1.0];
    }

    (quad_token, normals_token)
}

#[inline]
pub fn tetrahedron(
    pool: &mut Pool,
) -> (
    AllocationArrayToken<[f32; 3]>,
    AllocationArrayToken<[f32; 3]>,
) {
    let vertices = 3 * 4;

    let mut tetrahedron_token = pool.allocate_array(vertices, [0.0, 0.0, 0.0]);
    let mut normals_token = pool.allocate_array(vertices, [0.0, 0.0, 0.0]);

    let tetrahedron = pool.borrow_slice_mut(&mut tetrahedron_token);
    let normals = pool.borrow_slice_mut(&mut normals_token);

    let verts: [(i8, i8, i8); 4] = [(1, 1, 1), (1, -1, -1), (-1, 1, -1), (-1, -1, 1)];

    let index: [u8; 12] = [0, 1, 2, 0, 2, 3, 0, 3, 1, 2, 1, 3];

    for (i, vert) in tetrahedron.iter_mut().enumerate() {
        let v = unsafe { verts.get_unchecked(*index.get_unchecked(i) as usize) };

        *vert = [v.0 as f32, v.1 as f32, v.2 as f32];
    }

    for i in 0..(vertices / 3) {
        let ii = 3 * i;
        let i1 = ii;
        let i2 = ii + 1;
        let i3 = ii + 2;

        let a = Vec4::from_slice(unsafe { tetrahedron.get_unchecked(i1) });
        let b = Vec4::from_slice(unsafe { tetrahedron.get_unchecked(i2) });
        let c = Vec4::from_slice(unsafe { tetrahedron.get_unchecked(i3) });

        let side_ab = b.sub(a);
        let side_ac = c.sub(a);

        let normal = side_ab.cross(side_ac).normalized().to_slice();

        unsafe { *normals.get_unchecked_mut(i1) = normal };
        unsafe { *normals.get_unchecked_mut(i2) = normal };
        unsafe { *normals.get_unchecked_mut(i3) = normal };
    }

    (tetrahedron_token, normals_token)
}

#[inline]
pub fn sphere(
    pool: &mut Pool,
    vertical_slices: i32,
    radial_slices: i32,
) -> AllocationArrayToken<[f32; 3]> {
    let vertices = (vertical_slices * (2 + 2 * (radial_slices + 1))) as usize;

    let mut sphere_token = pool.allocate_array(vertices, [0.0, 0.0, 0.0]);
    let sphere = pool.borrow_slice_mut(&mut sphere_token);

    let vertical_slices_f32 = vertical_slices as f32;
    let radial_slices_f32 = radial_slices as f32;

    let mut index = 0;

    for i in 0..vertical_slices {
        let elevation = i as f32 * math::PI / vertical_slices_f32;
        let elevation_next = (i + 1) as f32 * math::PI / vertical_slices_f32;

        let (sin_elevation, cos_elevation) = math::sin_cos(elevation);
        let (sin_elevation_next, cos_elevation_next) = math::sin_cos(elevation_next);

        unsafe { *sphere.get_unchecked_mut(index) = [sin_elevation, cos_elevation, 0.0] };
        index += 1;

        for j in 0..(radial_slices + 1) {
            let azimuth = j as f32 * 2.0 * math::PI / radial_slices_f32;

            let (sin_azimuth, cos_azimuth) = math::sin_cos(azimuth);

            unsafe {
                *sphere.get_unchecked_mut(index) = [
                    sin_elevation * cos_azimuth,
                    cos_elevation,
                    sin_elevation * sin_azimuth,
                ]
            };
            index += 1;

            unsafe {
                *sphere.get_unchecked_mut(index) = [
                    sin_elevation_next * cos_azimuth,
                    cos_elevation_next,
                    sin_elevation_next * sin_azimuth,
                ]
            };
            index += 1;
        }

        unsafe { *sphere.get_unchecked_mut(index) = [sin_elevation_next, cos_elevation_next, 0.0] };
        index += 1;
    }

    utils::assert(sphere.len() != index);

    sphere_token
}
