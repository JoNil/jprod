use math::Vec4;
use math;
use pool::PoolAllocator;
use utils;

#[inline]
pub fn quad<'a>(pool: &'a PoolAllocator<'a>) -> (&'a [[f32; 3]], &'a [[f32; 3]]) {

    let vertices = 4;

    let mut quad = pool.allocate_slice(vertices);
    let mut normals = pool.allocate_slice(vertices);

    let verts: [(i8, i8); 4] = [
        ( 1, 1),
        (-1, 1),
        ( 1,-1),
        (-1,-1),
    ];

    for (i, (vert, normal)) in quad.iter_mut().zip(normals.iter_mut()).enumerate() {

        let v = unsafe { verts.get_unchecked(i) };

        *vert = [v.0 as f32, v.1 as f32, 0.0];
        *normal = [0.0, 0.0, 1.0];
    }

    (quad, normals)
}

#[inline]
pub fn tetrahedron<'a>(pool: &'a PoolAllocator<'a>) -> (&'a [[f32; 3]], &'a [[f32; 3]]) {

    let vertices = 3 * 4;

    let mut tetrahedron = pool.allocate_slice(vertices);
    let mut normals = pool.allocate_slice(vertices);

    let verts: [(i8, i8, i8); 4] = [
        ( 1, 1, 1),
        ( 1,-1,-1),
        (-1, 1,-1),
        (-1,-1, 1),
    ];

    let index: [u8; 12] = [
        0, 1, 2,
        0, 2, 3,
        0, 3, 1,
        2, 1, 3,
    ];

    for (i, vert) in tetrahedron.iter_mut().enumerate() {

        let v = unsafe { verts.get_unchecked(*index.get_unchecked(i) as usize) };

        *vert = [v.0 as f32, v.1 as f32, v.2 as f32];
    }

    for i in 0..(vertices/3) {

        let ii = 3*i;
        let i1 = ii + 0;
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

    (tetrahedron, normals)
}

#[inline]
pub fn sphere<'a>(pool: &'a PoolAllocator<'a>, vertical_slices: i32, radial_slices: i32) -> (&'a [[f32; 3]], &'a [[f32; 3]]) {

    let vertices = (vertical_slices*(2 + 2*(radial_slices + 1))) as usize;

    let mut sphere = pool.allocate_slice(vertices);

    let vertical_slices_f32 = vertical_slices as f32;
    let radial_slices_f32 = radial_slices as f32;

    let mut index = 0;

    for i in 0..vertical_slices {

        let elevation = i as f32 * math::PI / vertical_slices_f32;
        let elevation_next = (i + 1) as f32 * math::PI / vertical_slices_f32;

        let (sin_elevation, cos_elevation) = math::sin_cos(elevation);
        let (sin_elevation_next, cos_elevation_next) = math::sin_cos(elevation_next);

        unsafe { *sphere.get_unchecked_mut(index) = [
            sin_elevation,
            cos_elevation,
            0.0 ] };
        index += 1;

        for j in 0..(radial_slices + 1) {

            let azimuth = j as f32 * 2.0 * math::PI / radial_slices_f32;

            let (sin_azimuth, cos_azimuth) = math::sin_cos(azimuth);

            unsafe { *sphere.get_unchecked_mut(index) = [
                sin_elevation * cos_azimuth,
                cos_elevation,
                sin_elevation * sin_azimuth ] };
            index += 1;

            unsafe { *sphere.get_unchecked_mut(index) = [
                sin_elevation_next * cos_azimuth,
                cos_elevation_next,
                sin_elevation_next * sin_azimuth ] };
            index += 1;
        }

        unsafe { *sphere.get_unchecked_mut(index) =
            [ sin_elevation_next, cos_elevation_next, 0.0 ] };
        index += 1;
    }

    utils::assert(sphere.len() != index);

    (sphere, sphere)
}