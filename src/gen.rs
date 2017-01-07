use pool::PoolAllocator;

pub fn quad<'a>(pool: &'a PoolAllocator<'a>) -> &'a [[f32; 3]] {

    let mut quad = pool.allocate_slice(3 * 2);

    let verts: [(i8, i8); 4] = [
        ( 1, 1),
        ( 1,-1),
        (-1, 1),
        (-1,-1),
    ];

    let index: [u8; 6] = [
        3, 1, 0,
        3, 2, 0,
    ];

    for (i, vert) in quad.iter_mut().enumerate() {

        let v = unsafe { verts.get_unchecked(*index.get_unchecked(i) as usize) };

       *vert = [v.0 as f32 * 0.5, v.1 as f32 * 0.5, 0.0];
    }

    quad
}

pub fn tetrahedron<'a>(pool: &'a PoolAllocator<'a>) -> &'a [[f32; 3]] {

    let mut tetrahedron = pool.allocate_slice(3 * 4);

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
        2, 3, 1,
    ];

    for (i, vert) in tetrahedron.iter_mut().enumerate() {

        let v = unsafe { verts.get_unchecked(*index.get_unchecked(i) as usize) };

       *vert = [v.0 as f32, v.1 as f32, v.2 as f32];
    }

    tetrahedron
}