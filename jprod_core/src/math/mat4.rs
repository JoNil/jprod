use crate::{
    math::{self, Vec4},
    random::Rng,
};
use core::{arch::x86_64::*, mem};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mat4 {
    pub m_0: Vec4,
    pub m_1: Vec4,
    pub m_2: Vec4,
    pub m_3: Vec4,
}

impl Mat4 {
    #[inline]
    pub fn identity() -> Mat4 {
        Mat4 {
            m_0: Vec4::xyzw(1.0, 0.0, 0.0, 0.0),
            m_1: Vec4::xyzw(0.0, 1.0, 0.0, 0.0),
            m_2: Vec4::xyzw(0.0, 0.0, 1.0, 0.0),
            m_3: Vec4::xyzw(0.0, 0.0, 0.0, 1.0),
        }
    }

    #[inline]
    pub fn axis(x: Vec4, y: Vec4, z: Vec4) -> Mat4 {
        Mat4 {
            m_0: x.with_w(0.0),
            m_1: y.with_w(0.0),
            m_2: z.with_w(0.0),
            m_3: Vec4::xyzw(0.0, 0.0, 0.0, 1.0),
        }
    }

    #[inline]
    pub fn translate(pos: Vec4) -> Mat4 {
        Mat4 {
            m_0: Vec4::xyzw(1.0, 0.0, 0.0, 0.0),
            m_1: Vec4::xyzw(0.0, 1.0, 0.0, 0.0),
            m_2: Vec4::xyzw(0.0, 0.0, 1.0, 0.0),
            m_3: Vec4::xyzw(pos.x(), pos.y(), pos.z(), 1.0),
        }
    }

    #[inline]
    pub fn scale_xyz(x: f32, y: f32, z: f32) -> Mat4 {
        Mat4 {
            m_0: Vec4::xyzw(x, 0.0, 0.0, 0.0),
            m_1: Vec4::xyzw(0.0, y, 0.0, 0.0),
            m_2: Vec4::xyzw(0.0, 0.0, z, 0.0),
            m_3: Vec4::xyzw(0.0, 0.0, 0.0, 1.0),
        }
    }

    #[inline]
    pub fn scale(s: f32) -> Mat4 {
        Mat4 {
            m_0: Vec4::xyzw(s, 0.0, 0.0, 0.0),
            m_1: Vec4::xyzw(0.0, s, 0.0, 0.0),
            m_2: Vec4::xyzw(0.0, 0.0, s, 0.0),
            m_3: Vec4::xyzw(0.0, 0.0, 0.0, 1.0),
        }
    }

    #[inline]
    pub fn rotate_deg(angle: f32, axis: Vec4) -> Mat4 {
        Mat4::rotate(angle * math::PI / 180.0, axis)
    }

    #[inline]
    pub fn rotate(angle: f32, axis: Vec4) -> Mat4 {
        let mut temp = Mat4::identity();

        let (s, c) = math::sin_cos(angle);
        let t = 1.0 - c;
        let a = axis.normalized();

        temp.m_0 = temp.m_0.with_x(c + a.x() * a.x() * t);
        temp.m_1 = temp.m_1.with_y(c + a.y() * a.y() * t);
        temp.m_2 = temp.m_2.with_z(c + a.z() * a.z() * t);

        {
            let tmp1 = a.x() * a.y() * t;
            let tmp2 = a.z() * s;
            temp.m_0 = temp.m_0.with_y(tmp1 + tmp2);
            temp.m_1 = temp.m_1.with_x(tmp1 - tmp2);
        }

        {
            let tmp1 = a.x() * a.z() * t;
            let tmp2 = a.y() * s;
            temp.m_0 = temp.m_0.with_z(tmp1 - tmp2);
            temp.m_2 = temp.m_2.with_x(tmp1 + tmp2);
        }

        {
            let tmp1 = a.y() * a.z() * t;
            let tmp2 = a.x() * s;
            temp.m_1 = temp.m_1.with_z(tmp1 + tmp2);
            temp.m_2 = temp.m_2.with_y(tmp1 - tmp2);
        }

        temp
    }

    #[inline]
    pub fn random_rotation(rng: &mut Rng) -> Mat4 {
        let a = Vec4::xyz(
            rng.next_f32() - 0.5,
            rng.next_f32() - 0.5,
            rng.next_f32() - 0.5,
        )
        .normalized();
        let b = Vec4::xyz(
            rng.next_f32() - 0.5,
            rng.next_f32() - 0.5,
            rng.next_f32() - 0.5,
        )
        .normalized();

        let c = a.cross(b).normalized();
        let d = a.cross(c).normalized();

        Mat4::axis(a, c, d)
    }

    #[inline]
    pub fn frustum(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
        Mat4 {
            m_0: Vec4::xyzw(2.0 * near / (right - left), 0.0, 0.0, 0.0),
            m_1: Vec4::xyzw(0.0, 2.0 * near / (top - bottom), 0.0, 0.0),
            m_2: Vec4::xyzw(
                (right + left) / (right - left),
                (top + bottom) / (top - bottom),
                -(far + near) / (far - near),
                -1.0,
            ),
            m_3: Vec4::xyzw(0.0, 0.0, -2.0 * far * near / (far - near), 0.0),
        }
    }

    #[inline]
    pub fn perspective(horizontal_fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4 {
        let height = near * math::tan(horizontal_fov * math::PI / 360.0);
        let width = height * aspect_ratio;
        Mat4::frustum(-width, width, -height, height, near, far)
    }

    #[inline]
    pub fn transposed(&self) -> Mat4 {
        Mat4 {
            m_0: Vec4::xyzw(self.m_0.x(), self.m_1.x(), self.m_2.x(), self.m_3.x()),
            m_1: Vec4::xyzw(self.m_0.y(), self.m_1.y(), self.m_2.y(), self.m_3.y()),
            m_2: Vec4::xyzw(self.m_0.z(), self.m_1.z(), self.m_2.z(), self.m_3.z()),
            m_3: Vec4::xyzw(self.m_0.w(), self.m_1.w(), self.m_2.w(), self.m_3.w()),
        }
    }

    #[inline]
    pub fn inverted(&self) -> Mat4 {
        // Based on: https://lxjk.github.io/2017/09/03/Fast-4x4-Matrix-Inverse-with-SSE-SIMD-Explained.html

        #[inline(always)]
        unsafe fn mat_2_mul(vec1: __m128, vec2: __m128) -> __m128 {
            _mm_add_ps(
                _mm_mul_ps(vec1, vec4_swizzle!(Vec4(vec2), 0, 3, 0, 3).0),
                _mm_mul_ps(
                    vec4_swizzle!(Vec4(vec1), 1, 0, 3, 2).0,
                    vec4_swizzle!(Vec4(vec2), 2, 1, 2, 1).0,
                ),
            )
        }

        #[inline(always)]
        unsafe fn mat_2_adj_mul(vec1: __m128, vec2: __m128) -> __m128 {
            _mm_sub_ps(
                _mm_mul_ps(vec4_swizzle!(Vec4(vec1), 3, 3, 0, 0).0, vec2),
                _mm_mul_ps(
                    vec4_swizzle!(Vec4(vec1), 1, 1, 2, 2).0,
                    vec4_swizzle!(Vec4(vec2), 2, 3, 0, 1).0,
                ),
            )
        }

        #[inline(always)]
        unsafe fn mat_2_mul_adj(vec1: __m128, vec2: __m128) -> __m128 {
            _mm_sub_ps(
                _mm_mul_ps(vec1, vec4_swizzle!(Vec4(vec2), 3, 0, 3, 0).0),
                _mm_mul_ps(
                    vec4_swizzle!(Vec4(vec1), 1, 0, 3, 2).0,
                    vec4_swizzle!(Vec4(vec2), 2, 1, 2, 1).0,
                ),
            )
        }

        unsafe {
            let a: __m128 = vec4_shuffle!(self.m_0, self.m_1, 0, 1, 0, 1).0;
            let b: __m128 = vec4_shuffle!(self.m_0, self.m_1, 2, 3, 2, 3).0;
            let c: __m128 = vec4_shuffle!(self.m_2, self.m_3, 0, 1, 0, 1).0;
            let d: __m128 = vec4_shuffle!(self.m_2, self.m_3, 2, 3, 2, 3).0;

            let mat = self.as_tuples();

            let det_a = _mm_set1_ps((mat.0).0 * (mat.1).1 - (mat.0).1 * (mat.1).0);
            let det_b = _mm_set1_ps((mat.0).2 * (mat.1).3 - (mat.0).3 * (mat.1).2);
            let det_c = _mm_set1_ps((mat.2).0 * (mat.3).1 - (mat.2).1 * (mat.3).0);
            let det_d = _mm_set1_ps((mat.2).2 * (mat.3).3 - (mat.2).3 * (mat.3).2);

            let d_c = mat_2_adj_mul(d, c);
            let a_b = mat_2_adj_mul(a, b);

            let x = _mm_sub_ps(_mm_mul_ps(det_d, a), mat_2_mul(b, d_c));
            let w = _mm_sub_ps(_mm_mul_ps(det_a, d), mat_2_mul(c, a_b));

            let det_m = _mm_mul_ps(det_a, det_d);

            let y = _mm_sub_ps(_mm_mul_ps(det_b, c), mat_2_mul_adj(d, a_b));
            let z = _mm_sub_ps(_mm_mul_ps(det_c, b), mat_2_mul_adj(a, d_c));

            let det_m = _mm_add_ps(det_m, _mm_mul_ps(det_b, det_c));

            let tr = _mm_mul_ps(a_b, vec4_swizzle!(Vec4(d_c), 0, 2, 1, 3).0);
            let tr = _mm_hadd_ps(tr, tr);
            let tr = _mm_hadd_ps(tr, tr);

            let det_m = _mm_sub_ps(det_m, tr);

            let adj_sign_mask = _mm_setr_ps(1.0, -1.0, -1.0, 1.0);
            let r_det_m = _mm_div_ps(adj_sign_mask, det_m);

            let x = _mm_mul_ps(x, r_det_m);
            let y = _mm_mul_ps(y, r_det_m);
            let z = _mm_mul_ps(z, r_det_m);
            let w = _mm_mul_ps(w, r_det_m);

            // apply adjugate and store, here we combine adjugate shuffle and store shuffle
            Mat4 {
                m_0: vec4_shuffle!(Vec4(x), Vec4(y), 3, 1, 3, 1),
                m_1: vec4_shuffle!(Vec4(x), Vec4(y), 2, 0, 2, 0),
                m_2: vec4_shuffle!(Vec4(z), Vec4(w), 3, 1, 3, 1),
                m_3: vec4_shuffle!(Vec4(z), Vec4(w), 2, 0, 2, 0),
            }
        }
    }

    #[inline]
    pub fn as_array(&self) -> &[f32; 16] {
        unsafe { mem::transmute(self) }
    }

    #[inline]
    pub fn as_array_mut(&mut self) -> &mut [f32; 16] {
        unsafe { mem::transmute(self) }
    }

    #[inline]
    pub fn as_vec4_array(&self) -> &[Vec4; 4] {
        unsafe { mem::transmute(self) }
    }

    #[inline]
    pub fn as_vec4_array_mut(&mut self) -> &mut [Vec4; 4] {
        unsafe { mem::transmute(self) }
    }

    #[inline]
    pub fn as_tuples(
        &self,
    ) -> &(
        (f32, f32, f32, f32),
        (f32, f32, f32, f32),
        (f32, f32, f32, f32),
        (f32, f32, f32, f32),
    ) {
        unsafe { mem::transmute(self) }
    }

    #[inline]
    pub fn transform(self, rhs: Vec4) -> Vec4 {
        let col1 = self.m_0;
        let col2 = self.m_1;
        let col3 = self.m_2;
        let col4 = self.m_3;

        let xxxx = vec4_swizzle!(rhs, 0, 0, 0, 0);
        let yyyy = vec4_swizzle!(rhs, 1, 1, 1, 1);
        let zzzz = vec4_swizzle!(rhs, 2, 2, 2, 2);
        let wwww = vec4_swizzle!(rhs, 3, 3, 3, 3);

        (col1.pairwise_mul(xxxx).add(col2.pairwise_mul(yyyy)))
            .add(col3.pairwise_mul(zzzz).add(col4.pairwise_mul(wwww)))
    }

    #[inline]
    pub fn mul(self, rhs: Mat4) -> Mat4 {
        let mut res: Mat4 = Mat4 {
            m_0: Vec4::zero(),
            m_1: Vec4::zero(),
            m_2: Vec4::zero(),
            m_3: Vec4::zero(),
        };

        {
            let b = rhs.as_array();
            let c = res.as_vec4_array_mut();

            for i in 0..4 {
                let x = unsafe { *b.get_unchecked(4 * i + 0) };
                let y = unsafe { *b.get_unchecked(4 * i + 1) };
                let z = unsafe { *b.get_unchecked(4 * i + 2) };
                let w = unsafe { *b.get_unchecked(4 * i + 3) };

                unsafe {
                    *c.get_unchecked_mut(i) = self.transform(Vec4::xyzw(x, y, z, w));
                }
            }
        }

        res
    }
}
