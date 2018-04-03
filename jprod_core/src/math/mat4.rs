use core::mem;
use math::Vec4;
use math;
use random::Rng;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mat4 {
    pub m: (
        Vec4,
        Vec4,
        Vec4,
        Vec4,
    ),
}

impl Mat4 {

    #[inline]
    pub extern "vectorcall" fn identity() -> Mat4 {
        Mat4 {
            m: (
                Vec4::xyzw(1.0, 0.0, 0.0, 0.0),
                Vec4::xyzw(0.0, 1.0, 0.0, 0.0),
                Vec4::xyzw(0.0, 0.0, 1.0, 0.0),
                Vec4::xyzw(0.0, 0.0, 0.0, 1.0),
            ),
        }
    }

    #[inline]
    pub extern "vectorcall" fn axis(x: Vec4, y: Vec4, z: Vec4) -> Mat4 {
        Mat4 {
            m: (
                x.with_w(0.0),
                y.with_w(0.0),
                z.with_w(0.0),
                Vec4::xyzw(0.0, 0.0, 0.0, 1.0),
            ),
        }
    }

    #[inline]
    pub extern "vectorcall" fn translate(pos: Vec4) -> Mat4 {
        Mat4 {
            m: (
                Vec4::xyzw(1.0, 0.0, 0.0, 0.0),
                Vec4::xyzw(0.0, 1.0, 0.0, 0.0),
                Vec4::xyzw(0.0, 0.0, 1.0, 0.0),
                Vec4::xyzw(pos.x(), pos.y(), pos.z(), 1.0),
            ),
        }   
    }

    #[inline]
    pub extern "vectorcall" fn scale_xyz(x: f32, y: f32, z: f32) -> Mat4 {
        Mat4 {
            m: (
                Vec4::xyzw(x,  0.0, 0.0, 0.0),
                Vec4::xyzw(0.0,  y, 0.0, 0.0),
                Vec4::xyzw(0.0, 0.0,  z, 0.0),
                Vec4::xyzw(0.0, 0.0, 0.0, 1.0),
            ),
        }   
    }

    #[inline]
    pub extern "vectorcall" fn scale(s: f32) -> Mat4 {
        Mat4 {
            m: (
                Vec4::xyzw(s,  0.0, 0.0, 0.0),
                Vec4::xyzw(0.0,  s, 0.0, 0.0),
                Vec4::xyzw(0.0, 0.0,  s, 0.0),
                Vec4::xyzw(0.0, 0.0, 0.0, 1.0),
            ),
        }   
    }

    #[inline]
    pub extern "vectorcall" fn rotate_deg(angle: f32, axis: Vec4) -> Mat4 {
        Mat4::rotate(angle * math::PI / 180.0, axis)
    }

    #[inline]
    pub extern "vectorcall" fn rotate(angle: f32, axis: Vec4) -> Mat4 {
        
        let mut temp = Mat4::identity();

        let (s, c) = math::sin_cos(angle);
        let t = 1.0 - c;
        let a = axis.normalized();

        temp.m.0 = temp.m.0.with_x(c + a.x()*a.x()*t);
        temp.m.1 = temp.m.1.with_y(c + a.y()*a.y()*t);
        temp.m.2 = temp.m.2.with_z(c + a.z()*a.z()*t);

        {
            let tmp1 = a.x()*a.y()*t;
            let tmp2 = a.z()*s;
            temp.m.0 = temp.m.0.with_y(tmp1 + tmp2);
            temp.m.1 = temp.m.1.with_x(tmp1 - tmp2);
        }

        {
            let tmp1 = a.x()*a.z()*t;
            let tmp2 = a.y()*s;
            temp.m.0 = temp.m.0.with_z(tmp1 - tmp2);
            temp.m.2 = temp.m.2.with_x(tmp1 + tmp2);
        }

        {
            let tmp1 = a.y()*a.z()*t;
            let tmp2 = a.x()*s;
            temp.m.1 = temp.m.1.with_z(tmp1 + tmp2);
            temp.m.2 = temp.m.2.with_y(tmp1 - tmp2);
        }

        temp
    }

    #[inline]
    pub extern "vectorcall" fn random_rotation(rng: &mut Rng) -> Mat4 {

        let a = Vec4::xyz(rng.next_f32() - 0.5, rng.next_f32() - 0.5, rng.next_f32() - 0.5).normalized();
        let b = Vec4::xyz(rng.next_f32() - 0.5, rng.next_f32() - 0.5, rng.next_f32() - 0.5).normalized();

        let c = a.cross(b).normalized();
        let d = a.cross(c).normalized();
        
        Mat4::axis(a, c, d)
    }

    #[inline]
    pub extern "vectorcall" fn frustum(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
        Mat4 {
            m: (
                Vec4::xyzw(2.0*near/(right-left), 0.0, 0.0, 0.0),
                Vec4::xyzw(0.0, 2.0*near/(top-bottom), 0.0, 0.0),
                Vec4::xyzw((right+left)/(right-left), (top+bottom)/(top-bottom), -(far+near)/(far-near), -1.0),
                Vec4::xyzw(0.0, 0.0, -2.0*far*near/(far-near), 0.0),
            ),
        }
    }

    #[inline]
    pub extern "vectorcall" fn perspective(horizontal_fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4 {

        let height = near*math::tan(horizontal_fov*math::PI/360.0);
        let width = height*aspect_ratio;
        Mat4::frustum(-width, width, -height, height, near, far)
    }

    #[inline]
    pub extern "vectorcall" fn transposed(&self) -> Mat4 {
        Mat4 {
            m: (
                Vec4::xyzw(self.m.0.x(), self.m.1.x(), self.m.2.x(), self.m.3.x()),
                Vec4::xyzw(self.m.0.y(), self.m.1.y(), self.m.2.y(), self.m.3.y()),
                Vec4::xyzw(self.m.0.z(), self.m.1.z(), self.m.2.z(), self.m.3.z()),
                Vec4::xyzw(self.m.0.w(), self.m.1.w(), self.m.2.w(), self.m.3.w()),
            )
        }
    }

    #[inline]
    pub extern "vectorcall" fn inverted(&self) -> Mat4 {
        let mut res = Mat4::identity();

        {
            let row0;
            let mut row1;
            let mut row2;
            let mut row3;

            let mut det;
            let mut tmp1;

            /* Load matrix: */

            let mut col0 = self.m.0;
            let mut col1 = self.m.1;
            let mut col2 = self.m.2;
            let mut col3 = self.m.3;

            /* Transpose: */

            tmp1 = vec4_shuffle!(col0, col2, 0, 4, 1, 5);
            row1 = vec4_shuffle!(col1, col3, 0, 4, 1, 5);

            row0 = vec4_shuffle!(tmp1, row1, 0, 4, 1, 5);
            row1 = vec4_shuffle!(tmp1, row1, 2, 6, 3, 7);

            tmp1 = vec4_shuffle!(col0, col2, 2, 6, 3, 7);
            row3 = vec4_shuffle!(col1, col3, 2, 6, 3, 7);

            row2 = vec4_shuffle!(tmp1, row3, 0, 4, 1, 5);
            row3 = vec4_shuffle!(tmp1, row3, 2, 6, 3, 7);

            /* Compute adjoint: */

            row1 = vec4_shuffle!(row1, row1, 2, 3, 0, 1);
            row3 = vec4_shuffle!(row3, row3, 2, 3, 0, 1);

            tmp1 = row2.pairwise_mul(row3);
            tmp1 = vec4_shuffle!(tmp1, tmp1, 1, 0, 7, 6);

            col0 = row1.pairwise_mul(tmp1);
            col1 = row0.pairwise_mul(tmp1);

            tmp1 = vec4_shuffle!(tmp1, tmp1, 2, 3, 4, 5);

            col0 = row1.pairwise_mul(tmp1).sub(col0);
            col1 = row0.pairwise_mul(tmp1).sub(col1);
            col1 = vec4_shuffle!(col1, col1, 2, 3, 4, 5);

            tmp1 = row1.pairwise_mul(row2);
            tmp1 = vec4_shuffle!(tmp1, tmp1, 1, 0, 7, 6);

            col0 = row3.pairwise_mul(tmp1).add(col0);
            col3 = row0.pairwise_mul(tmp1);

            tmp1 = vec4_shuffle!(tmp1, tmp1, 2, 3, 4, 5);

            col0 = col0.sub(row3.pairwise_mul(tmp1));
            col3 = row0.pairwise_mul(tmp1).sub(col3);
            col3 = vec4_shuffle!(col3, col3, 2, 3, 4, 5);

            tmp1 = vec4_shuffle!(row1, row1, 2, 3, 4, 5).pairwise_mul(row3);
            tmp1 = vec4_shuffle!(tmp1, tmp1, 1, 0, 7, 6);
            row2 = vec4_shuffle!(row2, row2, 2, 3, 4, 5);

            col0 = row2.pairwise_mul(tmp1).add(col0);
            col2 = row0.pairwise_mul(tmp1);

            tmp1 = vec4_shuffle!(tmp1, tmp1, 2, 3, 4, 5);

            col0 = col0.sub(row2.pairwise_mul(tmp1));
            col2 = row0.pairwise_mul(tmp1).sub(col2);
            col2 = vec4_shuffle!(col2, col2, 2, 3, 4, 5);

            tmp1 = row0.pairwise_mul(row1);
            tmp1 = vec4_shuffle!(tmp1, tmp1, 1, 0, 7, 6);

            col2 = row3.pairwise_mul(tmp1).add(col2);
            col3 = row2.pairwise_mul(tmp1).sub(col3);

            tmp1 = vec4_shuffle!(tmp1, tmp1, 2, 3, 4, 5);

            col2 = row3.pairwise_mul(tmp1).sub(col2);
            col3 = col3.sub(row2.pairwise_mul(tmp1));

            tmp1 = row0.pairwise_mul(row3);
            tmp1 = vec4_shuffle!(tmp1, tmp1, 1, 0, 7, 6);

            col1 = col1.sub(row2.pairwise_mul(tmp1));
            col2 = row1.pairwise_mul(tmp1).add(col2);

            tmp1 = vec4_shuffle!(tmp1, tmp1, 2, 3, 4, 5);

            col1 = row2.pairwise_mul(tmp1).add(col1);
            col2 = col2.sub(row1.pairwise_mul(tmp1));

            tmp1 = row0.pairwise_mul(row2);
            tmp1 = vec4_shuffle!(tmp1, tmp1, 1, 0, 7, 6);

            col1 = row3.pairwise_mul(tmp1).add(col1);
            col3 = col3.sub(row1.pairwise_mul(tmp1));

            tmp1 = vec4_shuffle!(tmp1, tmp1, 2, 3, 4, 5);

            col1 = col1.sub(row3.pairwise_mul(tmp1));
            col3 = row1.pairwise_mul(tmp1).add(col3);

            /* Compute determinant: */

            det = row0.pairwise_mul(col0);
            det = vec4_shuffle!(det, det, 2, 3, 4, 5).add(det);
            det = vec4_shuffle!(det, det, 1, 0, 7, 6).add(det);

            /* Compute reciprocal of determinant: */

            det = Vec4::splat(1.0).pairwise_div(det);

            /* Multiply matrix of cofactors with reciprocal of determinant: */

            col0 = col0.pairwise_mul(det);
            col1 = col1.pairwise_mul(det);
            col2 = col2.pairwise_mul(det);
            col3 = col3.pairwise_mul(det);

            /* Store inverted matrix: */

            res.m.0 = col0;
            res.m.1 = col1;
            res.m.2 = col2;
            res.m.3 = col3;
        }

        res
    }

    #[inline]
    pub extern "vectorcall" fn as_array(&self) -> &[f32; 16] {
        unsafe { mem::transmute(&self.m) }
    }

    #[inline]
    pub extern "vectorcall" fn as_array_mut(&mut self) -> &mut [f32; 16] {
        unsafe { mem::transmute(&mut self.m) }
    }

    #[inline]
    pub extern "vectorcall" fn as_vec4_array(&self) -> &[Vec4; 4] {
        unsafe { mem::transmute(&self.m) }
    }

    #[inline]
    pub extern "vectorcall" fn as_vec4_array_mut(&mut self) -> &mut [Vec4; 4] {
        unsafe { mem::transmute(&mut self.m) }
    }

    #[inline]
    pub extern "vectorcall" fn as_flat_tuple(&self) -> &(
        f32, f32, f32, f32,
        f32, f32, f32, f32,
        f32, f32, f32, f32,
        f32, f32, f32, f32)
    {
        unsafe { mem::transmute(&self.m) }
    }

    #[inline]
    pub extern "vectorcall" fn as_flat_tuple_mut(&mut self) -> &mut (
        f32, f32, f32, f32,
        f32, f32, f32, f32,
        f32, f32, f32, f32,
        f32, f32, f32, f32)
    {
        unsafe { mem::transmute(&mut self.m) }
    }

    #[inline]
    pub extern "vectorcall" fn transform(self, rhs: Vec4) -> Vec4 {

        let col1 = self.m.0;
        let col2 = self.m.1;
        let col3 = self.m.2;
        let col4 = self.m.3;

        let xxxx = vec4_swizzle!(rhs, 0, 0, 0, 0);
        let yyyy = vec4_swizzle!(rhs, 1, 1, 1, 1);
        let zzzz = vec4_swizzle!(rhs, 2, 2, 2, 2);
        let wwww = vec4_swizzle!(rhs, 3, 3, 3, 3);

        (col1.pairwise_mul(xxxx).add(col2.pairwise_mul(yyyy))).add(
            col3.pairwise_mul(zzzz).add(col4.pairwise_mul(wwww)))
    }

    #[inline]
    pub extern "vectorcall" fn mul(self, rhs: Mat4) -> Mat4 {

        let mut res: Mat4 = unsafe { mem::uninitialized() };

        {
            let b = rhs.as_array();
            let c = res.as_vec4_array_mut();

            for i in 0..4 {

                let x = unsafe { *b.get_unchecked(4*i + 0) };
                let y = unsafe { *b.get_unchecked(4*i + 1) };
                let z = unsafe { *b.get_unchecked(4*i + 2) };
                let w = unsafe { *b.get_unchecked(4*i + 3) };

                unsafe { *c.get_unchecked_mut(i) = self.transform(Vec4::xyzw(x, y, z, w)); }
            }
        }

        res
    }
}