#![allow(dead_code)]

use core::mem;
use core::ops::Mul;
use core::ops::MulAssign;
use f32;
use vec4::Vec4;

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

    pub fn identity() -> Mat4 {
        Mat4 {
            m: (
                Vec4::xyzw(1.0, 0.0, 0.0, 0.0),
                Vec4::xyzw(0.0, 1.0, 0.0, 0.0),
                Vec4::xyzw(0.0, 0.0, 1.0, 0.0),
                Vec4::xyzw(0.0, 0.0, 0.0, 1.0),
            ),
        }
    }

    pub fn axis(x: Vec4, y: Vec4, z: Vec4) -> Mat4 {
        Mat4 {
            m: (
                Vec4::xyzw(x.x, x.y, x.z, 0.0),
                Vec4::xyzw(y.x, y.y, y.z, 0.0),
                Vec4::xyzw(z.x, z.y, z.z, 0.0),
                Vec4::xyzw(0.0, 0.0, 0.0, 1.0),
            ),
        }
    }


    pub fn translate(pos: Vec4) -> Mat4 {
        Mat4 {
            m: (
                Vec4::xyzw(1.0, 0.0, 0.0, 0.0),
                Vec4::xyzw(0.0, 1.0, 0.0, 0.0),
                Vec4::xyzw(0.0, 0.0, 1.0, 0.0),
                Vec4::xyzw(pos.x, pos.y, pos.z, 1.0),
            ),
        }   
    }

    pub fn rotate(angle: f32, axis: Vec4) -> Mat4 {
        
        let mut temp = Mat4::identity();

        let c = f32::cos(angle);
        let s = f32::sin(angle);
        let t = 1.0 - c;
        let a = axis.normalized();

        temp.m.0.x = c + a.x*a.x*t;
        temp.m.1.y = c + a.y*a.y*t;
        temp.m.2.z = c + a.z*a.z*t;

        {
            let tmp1 = a.x*a.y*t;
            let tmp2 = a.z*s;
            temp.m.0.y = tmp1 + tmp2;
            temp.m.1.x = tmp1 - tmp2;
        }

        {
            let tmp1 = a.x*a.z*t;
            let tmp2 = a.y*s;
            temp.m.0.z = tmp1 - tmp2;
            temp.m.2.x = tmp1 + tmp2;
        }

        {
            let tmp1 = a.y*a.z*t;
            let tmp2 = a.x*s;
            temp.m.1.z = tmp1 + tmp2;
            temp.m.2.y = tmp1 - tmp2;
        }

        temp
    }

    pub fn frustum(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
        Mat4 {
            m: (
                Vec4::xyzw(2.0*near/(right-left), 0.0, 0.0, 0.0),
                Vec4::xyzw(0.0, 2.0*near/(top-bottom), 0.0, 0.0),
                Vec4::xyzw((right+left)/(right-left), (top+bottom)/(top-bottom), -(far+near)/(far-near), -1.0),
                Vec4::xyzw(0.0, 0.0, -2.0*far*near/(far-near), 0.0),
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
                Vec4::xyzw(self.m.0.x, self.m.1.x, self.m.2.x, self.m.3.x),
                Vec4::xyzw(self.m.0.y, self.m.1.y, self.m.2.y, self.m.3.y),
                Vec4::xyzw(self.m.0.z, self.m.1.z, self.m.2.z, self.m.3.z),
                Vec4::xyzw(self.m.0.w, self.m.1.w, self.m.2.w, self.m.3.w),
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

    pub fn as_vec4_array(&self) -> &[Vec4; 4] {
        unsafe { mem::transmute(&self.m) }
    }

    pub fn as_vec4_array_mut(&mut self) -> &mut [Vec4; 4] {
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
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    #[inline(always)]
    fn mul(self, rhs: Vec4) -> Vec4 {

        let col1 = self.m.0;
        let col2 = self.m.1;
        let col3 = self.m.2;
        let col4 = self.m.3;

        let xxxx = Vec4::splat(rhs.x);
        let yyyy = Vec4::splat(rhs.y);
        let zzzz = Vec4::splat(rhs.z);
        let wwww = Vec4::splat(rhs.w);

        (col1.pairwise_mul(xxxx) + col2.pairwise_mul(yyyy)) +
        (col3.pairwise_mul(zzzz) + col4.pairwise_mul(wwww))
    }
}

impl Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Mat4 {

        let mut res: Mat4 = unsafe { mem::uninitialized() };

        {
            let b = rhs.as_array();
            let mut c = res.as_vec4_array_mut();

            for i in 0..4 {

                let x = unsafe { *b.get_unchecked(4*i + 0) };
                let y = unsafe { *b.get_unchecked(4*i + 1) };
                let z = unsafe { *b.get_unchecked(4*i + 2) };
                let w = unsafe { *b.get_unchecked(4*i + 3) };

                unsafe { *c.get_unchecked_mut(i) = self * Vec4::xyzw(x, y, z, w); }
            }
        }

        res
    }
}

impl MulAssign for Mat4 {
    fn mul_assign(&mut self, rhs: Mat4) {
        *self = *self * rhs
    }
}