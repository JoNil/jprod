#![allow(dead_code)]

use core::mem;
use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Div;
use core::ops::DivAssign;
use core::ops::Mul;
use core::ops::MulAssign;
use core::ops::Neg;
use core::ops::Sub;
use core::ops::SubAssign;
use intrinsics;
use simdty::f32x4;

#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn zero() -> Vec4 {
        Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn xyz(x: f32, y: f32, z: f32) -> Vec4 {
        Vec4 {
            x: x,
            y: y,
            z: z,
            w: 0.0,
        }
    }

    pub fn xyzw(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn splat(a: f32) -> Vec4 {
        Vec4 {
            x: a,
            y: a,
            z: a,
            w: a,
        }
    }

    pub fn from_simd(simd: f32x4) -> Vec4 {
        Vec4 {
            x: simd.0,
            y: simd.1,
            z: simd.2,
            w: simd.3,
        }
    }

    pub fn with_w_0(self) -> Vec4 {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: 0.0,
        }
    }

    pub fn with_w_1(self) -> Vec4 {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: 1.0,
        }
    }

    pub fn to_simd(self) -> f32x4 {
        unsafe { mem::transmute(self) }
    }

    #[inline(never)]
    pub fn length(self) -> f32 {
        Vec4::splat(self.dot(self)).sqrt_x4().x
    }

    pub fn pairwise_mul(self, rhs: Vec4) -> Vec4 {
        unsafe { Vec4::from_simd(intrinsics::simd_mul(self.to_simd(), rhs.to_simd())) }
    }

    pub fn pairwise_div(self, rhs: Vec4) -> Vec4 {
        unsafe { Vec4::from_simd(intrinsics::simd_div(self.to_simd(), rhs.to_simd())) }
    }

    pub fn dot(self, rhs: Vec4) -> f32 {

        let temp = self.pairwise_mul(rhs);

        temp.x + temp.y + temp.z + temp.w
    }

    pub fn cross(self, rhs: Vec4) -> Vec4 {

        let a = Vec4::xyzw(self.y, self.z, self.x, 0.0);
        let b = Vec4::xyzw(rhs.z, rhs.x, rhs.y, 0.0);

        let c = Vec4::xyzw(self.z, self.x, self.y, 0.0);
        let d = Vec4::xyzw(rhs.y, rhs.z, rhs.x, 0.0);

        let temp1 = a.pairwise_mul(b);
        let temp2 = c.pairwise_mul(d);

        temp1 - temp2
    }

    pub fn normalized(self) -> Vec4 {
        self / self.length()
    }

    pub fn sqrt_x4(self) -> Vec4 {
        unsafe { Vec4::from_simd(intrinsics::sqrt_v4f32(self.to_simd())) }
    }
}

impl Add for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Vec4 {
        unsafe { Vec4::from_simd(intrinsics::simd_add(self.to_simd(), rhs.to_simd())) }
    }
}

impl AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Vec4) {
        *self = *self + rhs;
    }
}

impl Div<f32> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: f32) -> Vec4 {
        unsafe { Vec4::from_simd(intrinsics::simd_div(self.to_simd(), Vec4::splat(rhs).to_simd())) }
    }
}

impl DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: f32) -> Vec4 {
        self.pairwise_mul(Vec4::splat(rhs))
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4::splat(self).pairwise_mul(rhs)
    }
}

impl MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Neg for Vec4 {
    type Output = Vec4;

    fn neg(self) -> Vec4 {
        Vec4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Sub for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: Vec4) -> Vec4 {
        unsafe { Vec4::from_simd(intrinsics::simd_sub(self.to_simd(), rhs.to_simd())) }
    }
}

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, rhs: Vec4) {
        *self = *self - rhs;
    }
}