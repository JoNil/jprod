#![allow(dead_code)]

use core::mem;
use core::ops::Mul;
use core::ops::MulAssign;
use f32;
use intrinsics::*;
use simdty::*;
use vec4::Vec4;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mat4 {
    pub m: (
        (f32, f32, f32, f32),
        (f32, f32, f32, f32),
        (f32, f32, f32, f32),
        (f32, f32, f32, f32),
    ),
}

impl Mat4 {

    pub fn identity() -> Mat4 {
        Mat4 {
            m: (
                (1.0, 0.0, 0.0, 0.0),
                (0.0, 1.0, 0.0, 0.0),
                (0.0, 0.0, 1.0, 0.0),
                (0.0, 0.0, 0.0, 1.0),
            ),
        }
    }

    pub fn axis(x: Vec4, y: Vec4, z: Vec4) -> Mat4 {
        Mat4 {
            m: (
                (x.x, x.y, x.z, 0.0),
                (y.x, y.y, y.z, 0.0),
                (z.x, z.y, z.z, 0.0),
                (0.0, 0.0, 0.0, 1.0),
            ),
        }
    }


    pub fn translate(pos: Vec4) -> Mat4 {
        Mat4 {
            m: (
                (1.0, 0.0, 0.0, 0.0),
                (0.0, 1.0, 0.0, 0.0),
                (0.0, 0.0, 1.0, 0.0),
                (pos.x, pos.y, pos.z, 1.0),
            ),
        }   
    }

    pub fn rotate(angle: f32, axis: Vec4) -> Mat4 {
        
        let mut temp = Mat4::identity();

        let c = f32::cos(angle);
        let s = f32::sin(angle);
        let t = 1.0 - c;
        let a = axis.normalized();

        (temp.m.0).0 = c + a.x*a.x*t;
        (temp.m.1).1 = c + a.y*a.y*t;
        (temp.m.2).2 = c + a.z*a.z*t;

        {
            let tmp1 = a.x*a.y*t;
            let tmp2 = a.z*s;
            (temp.m.0).1 = tmp1 + tmp2;
            (temp.m.1).0 = tmp1 - tmp2;
        }

        {
            let tmp1 = a.x*a.z*t;
            let tmp2 = a.y*s;
            (temp.m.0).2 = tmp1 - tmp2;
            (temp.m.2).0 = tmp1 + tmp2;
        }

        {
            let tmp1 = a.y*a.z*t;
            let tmp2 = a.x*s;
            (temp.m.1).2 = tmp1 + tmp2;
            (temp.m.2).1 = tmp1 - tmp2;
        }

        temp
    }

    pub fn frustum(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
        Mat4 {
            m: (
                (2.0*near/(right-left), 0.0, 0.0, 0.0),
                (0.0, 2.0*near/(top-bottom), 0.0, 0.0),
                ((right+left)/(right-left), (top+bottom)/(top-bottom), -(far+near)/(far-near), -1.0),
                (0.0, 0.0, -2.0*far*near/(far-near), 0.0),
            ),
        }
    }

    pub fn perspective(horizontal_fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4 {

        let height = near*f32::tan(horizontal_fov*f32::consts::PI/360.0);
        let width = height*aspect_ratio;
        Mat4::frustum(-width, width, -height, height, near, far)
    }

    pub fn transposed(&self) -> Mat4 {
        Mat4 {
            m: (
                ((self.m.0).0, (self.m.1).0, (self.m.2).0, (self.m.3).0),
                ((self.m.0).1, (self.m.1).1, (self.m.2).1, (self.m.3).1),
                ((self.m.0).2, (self.m.1).2, (self.m.2).2, (self.m.3).2),
                ((self.m.0).3, (self.m.1).3, (self.m.2).3, (self.m.3).3),
            )
        }
    }

    pub fn inverted(&self) -> Option<Mat4> {
        let mut res = Mat4::identity();

        {
            let d = self.as_flat_tuple();
            let res_d = res.as_flat_tuple_mut();

            res_d.0 = d.5 * d.10 * d.15 - d.5 * d.11 * d.14 - d.9 * d.6 * d.15 + d.9 * d.7 * d.14 +d.13 * d.6 * d.11 - d.13 * d.7 * d.10;
            res_d.1 = -d.1 * d.10 * d.15 + d.1 * d.11 * d.14 + d.9 * d.2 * d.15 - d.9 * d.3 * d.14 - d.13 * d.2 * d.11 + d.13 * d.3 * d.10;
            res_d.2 = d.1 * d.6 * d.15 - d.1 * d.7 * d.14 - d.5 * d.2 * d.15 + d.5 * d.3 * d.14 + d.13 * d.2 * d.7 - d.13 * d.3 * d.6;
            res_d.3 = -d.1 * d.6 * d.11 + d.1 * d.7 * d.10 + d.5 * d.2 * d.11 - d.5 * d.3 * d.10 - d.9 * d.2 * d.7 + d.9 * d.3 * d.6;
            res_d.4 = -d.4 * d.10 * d.15 + d.4 * d.11 * d.14 + d.8 * d.6 * d.15 - d.8 * d.7 * d.14 - d.12 * d.6 * d.11 + d.12 * d.7 * d.10;
            res_d.5 = d.0 * d.10 * d.15 - d.0 * d.11 * d.14 - d.8 * d.2 * d.15 + d.8 * d.3 * d.14 + d.12 * d.2 * d.11 - d.12 * d.3 * d.10;
            res_d.6 = -d.0 * d.6 * d.15 + d.0 * d.7 * d.14 + d.4 * d.2 * d.15 - d.4 * d.3 * d.14 - d.12 * d.2 * d.7 + d.12 * d.3 * d.6;
            res_d.7 = d.0 * d.6 * d.11 - d.0 * d.7 * d.10 - d.4 * d.2 * d.11 + d.4 * d.3 * d.10 + d.8 * d.2 * d.7 - d.8 * d.3 * d.6;
            res_d.8 = d.4 * d.9 * d.15 - d.4 * d.11 * d.13 - d.8 * d.5 * d.15 + d.8 * d.7 * d.13 + d.12 * d.5 * d.11 - d.12 * d.7 * d.9;
            res_d.9 = -d.0 * d.9 * d.15 + d.0 * d.11 * d.13 + d.8 * d.1 * d.15 - d.8 * d.3 * d.13 - d.12 * d.1 * d.11 + d.12 * d.3 * d.9;
            res_d.10 = d.0 * d.5 * d.15 - d.0 * d.7 * d.13 - d.4 * d.1 * d.15 + d.4 * d.3 * d.13 + d.12 * d.1 * d.7 - d.12 * d.3 * d.5;
            res_d.11 = -d.0 * d.5 * d.11 + d.0 * d.7 * d.9 + d.4 * d.1 * d.11 - d.4 * d.3 * d.9 - d.8 * d.1 * d.7 + d.8 * d.3 * d.5;
            res_d.12 = -d.4 * d.9 * d.14 + d.4 * d.10 * d.13 +d.8 * d.5 * d.14 - d.8 * d.6 * d.13 - d.12 * d.5 * d.10 + d.12 * d.6 * d.9;
            res_d.13 = d.0 * d.9 * d.14 - d.0 * d.10 * d.13 - d.8 * d.1 * d.14 + d.8 * d.2 * d.13 + d.12 * d.1 * d.10 - d.12 * d.2 * d.9;
            res_d.14 = -d.0 * d.5 * d.14 + d.0 * d.6 * d.13 + d.4 * d.1 * d.14 - d.4 * d.2 * d.13 - d.12 * d.1 * d.6 + d.12 * d.2 * d.5;
            res_d.15 = d.0 * d.5 * d.10 - d.0 * d.6 * d.9 - d.4 * d.1 * d.10 + d.4 * d.2 * d.9 + d.8 * d.1 * d.6 - d.8 * d.2 * d.5;

            let mut det = d.0 * res_d.0 + d.1 * res_d.4 + d.2 * res_d.8 + d.3 * res_d.12;

            if det == 0.0 {
                return None;
            }

            det = 1.0 / det;

            res_d.0 *= det;
            res_d.1 *= det;
            res_d.2 *= det;
            res_d.3 *= det;
            res_d.4 *= det;
            res_d.5 *= det;
            res_d.6 *= det;
            res_d.7 *= det;
            res_d.8 *= det;
            res_d.9 *= det;
            res_d.10 *= det;
            res_d.11 *= det;
            res_d.12 *= det;
            res_d.13 *= det;
            res_d.14 *= det;
            res_d.15 *= det;
        }

        Some(res)
    }

    pub fn as_array(&self) -> &[f32; 16] {
        unsafe { mem::transmute(&self.m) }
    }

    pub fn as_array_mut(&mut self) -> &mut [f32; 16] {
        unsafe { mem::transmute(&mut self.m) }
    }

    pub fn as_simd_array(&self) -> &[f32x4; 4] {
        unsafe { mem::transmute(&self.m) }
    }

    pub fn as_simd_array_mut(&mut self) -> &mut [f32x4; 4] {
        unsafe { mem::transmute(&mut self.m) }
    }

    pub fn as_flat_tuple(&self) -> &(
        f32, f32, f32, f32,
        f32, f32, f32, f32,
        f32, f32, f32, f32,
        f32, f32, f32, f32)
    {
        unsafe { mem::transmute(&self.m) }
    }

    pub fn as_flat_tuple_mut(&mut self) -> &mut (
        f32, f32, f32, f32,
        f32, f32, f32, f32,
        f32, f32, f32, f32,
        f32, f32, f32, f32)
    {
        unsafe { mem::transmute(&mut self.m) }
    }

    pub fn as_simd_tuple(&self) -> &(f32x4, f32x4, f32x4, f32x4)
    {
        unsafe { mem::transmute(&self.m) }
    }

    pub fn as_simd_tuple_mut(&mut self) -> &mut (f32x4, f32x4, f32x4, f32x4)
    {
        unsafe { mem::transmute(&mut self.m) }
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: (self.m.0).0*rhs.x + (self.m.1).0*rhs.y + (self.m.2).0*rhs.z + (self.m.3).0*rhs.w,
            y: (self.m.0).1*rhs.x + (self.m.1).1*rhs.y + (self.m.2).1*rhs.z + (self.m.3).1*rhs.w,
            z: (self.m.0).2*rhs.x + (self.m.1).2*rhs.y + (self.m.2).2*rhs.z + (self.m.3).2*rhs.w,
            w: (self.m.0).3*rhs.x + (self.m.1).3*rhs.y + (self.m.2).3*rhs.z + (self.m.3).3*rhs.w,
        }
    }
}

extern "C" {
    fn M4x4_SSE(a: *const f32, b: *const f32, a: *mut f32);
}

impl Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Mat4 {

        /*let mut res: Mat4 = unsafe { mem::uninitialized() };

        {
            let a = self.as_array();
            let b = rhs.as_simd_tuple();
            let mut c = res.as_simd_array_mut();

            let row1 = b.0;
            let row2 = b.1;
            let row3 = b.2;
            let row4 = b.3;

            for i in 0..3 {

                let value1 = unsafe { *a.get_unchecked(4*i + 0) };
                let value2 = unsafe { *a.get_unchecked(4*i + 1) };
                let value3 = unsafe { *a.get_unchecked(4*i + 2) };
                let value4 = unsafe { *a.get_unchecked(4*i + 3) };

                let brod1 = f32x4(value1, value1, value1, value1);
                let brod2 = f32x4(value2, value2, value2, value2);
                let brod3 = f32x4(value3, value3, value3, value3);
                let brod4 = f32x4(value4, value4, value4, value4);

                let row = unsafe { simd_add(
                    simd_add(
                        simd_mul(brod1, row1),
                        simd_mul(brod2, row2)),
                    simd_add(
                        simd_mul(brod3, row3),
                        simd_mul(brod4, row4))) };

                unsafe { *c.get_unchecked_mut(i) = row; }
            }
        }

        res*/

        /*void M4x4_SSE(float *A, float *B, float *C) {
            __m128 row1 = _mm_load_ps(&B[0]);
            __m128 row2 = _mm_load_ps(&B[4]);
            __m128 row3 = _mm_load_ps(&B[8]);
            __m128 row4 = _mm_load_ps(&B[12]);
            for(int i=0; i<4; i++) {
                __m128 brod1 = _mm_set1_ps(A[4*i + 0]);
                __m128 brod2 = _mm_set1_ps(A[4*i + 1]);
                __m128 brod3 = _mm_set1_ps(A[4*i + 2]);
                __m128 brod4 = _mm_set1_ps(A[4*i + 3]);
                __m128 row = _mm_add_ps(
                            _mm_add_ps(
                                _mm_mul_ps(brod1, row1),
                                _mm_mul_ps(brod2, row2)),
                            _mm_add_ps(
                                _mm_mul_ps(brod3, row3),
                                _mm_mul_ps(brod4, row4)));
                _mm_store_ps(&C[4*i], row);
            }
        }*/

        Mat4 {
            m: (
                (
                    (rhs.m.0).0*(self.m.0).0+(rhs.m.0).1*(self.m.1).0+(rhs.m.0).2*(self.m.2).0+(rhs.m.0).3*(self.m.3).0,
                    (rhs.m.0).0*(self.m.0).1+(rhs.m.0).1*(self.m.1).1+(rhs.m.0).2*(self.m.2).1+(rhs.m.0).3*(self.m.3).1,
                    (rhs.m.0).0*(self.m.0).2+(rhs.m.0).1*(self.m.1).2+(rhs.m.0).2*(self.m.2).2+(rhs.m.0).3*(self.m.3).2,
                    (rhs.m.0).0*(self.m.0).3+(rhs.m.0).1*(self.m.1).3+(rhs.m.0).2*(self.m.2).3+(rhs.m.0).3*(self.m.3).3,
                ),
                (
                    (rhs.m.1).0*(self.m.0).0+(rhs.m.1).1*(self.m.1).0+(rhs.m.1).2*(self.m.2).0+(rhs.m.1).3*(self.m.3).0,
                    (rhs.m.1).0*(self.m.0).1+(rhs.m.1).1*(self.m.1).1+(rhs.m.1).2*(self.m.2).1+(rhs.m.1).3*(self.m.3).1,
                    (rhs.m.1).0*(self.m.0).2+(rhs.m.1).1*(self.m.1).2+(rhs.m.1).2*(self.m.2).2+(rhs.m.1).3*(self.m.3).2,
                    (rhs.m.1).0*(self.m.0).3+(rhs.m.1).1*(self.m.1).3+(rhs.m.1).2*(self.m.2).3+(rhs.m.1).3*(self.m.3).3,
                ),
                (
                    (rhs.m.2).0*(self.m.0).0+(rhs.m.2).1*(self.m.1).0+(rhs.m.2).2*(self.m.2).0+(rhs.m.2).3*(self.m.3).0,
                    (rhs.m.2).0*(self.m.0).1+(rhs.m.2).1*(self.m.1).1+(rhs.m.2).2*(self.m.2).1+(rhs.m.2).3*(self.m.3).1,
                    (rhs.m.2).0*(self.m.0).2+(rhs.m.2).1*(self.m.1).2+(rhs.m.2).2*(self.m.2).2+(rhs.m.2).3*(self.m.3).2,
                    (rhs.m.2).0*(self.m.0).3+(rhs.m.2).1*(self.m.1).3+(rhs.m.2).2*(self.m.2).3+(rhs.m.2).3*(self.m.3).3,
                ),
                (
                    (rhs.m.3).0*(self.m.0).0+(rhs.m.3).1*(self.m.1).0+(rhs.m.3).2*(self.m.2).0+(rhs.m.3).3*(self.m.3).0,
                    (rhs.m.3).0*(self.m.0).1+(rhs.m.3).1*(self.m.1).1+(rhs.m.3).2*(self.m.2).1+(rhs.m.3).3*(self.m.3).1,
                    (rhs.m.3).0*(self.m.0).2+(rhs.m.3).1*(self.m.1).2+(rhs.m.3).2*(self.m.2).2+(rhs.m.3).3*(self.m.3).2,
                    (rhs.m.3).0*(self.m.0).3+(rhs.m.3).1*(self.m.1).3+(rhs.m.3).2*(self.m.2).3+(rhs.m.3).3*(self.m.3).3,
                )
            ),
        }
    }
}

impl MulAssign for Mat4 {
    fn mul_assign(&mut self, rhs: Mat4) {
        *self = *self * rhs
    }
}