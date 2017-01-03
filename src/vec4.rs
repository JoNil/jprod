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
use f32;
use intrinsics::*;
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

    pub fn as_simd(&self) -> &f32x4 {
        unsafe { mem::transmute(self) }
    }

    pub fn as_simd_mut(&mut self) -> &mut f32x4 {
        unsafe { mem::transmute(self) }
    }

    pub fn length(self) -> f32 {
        f32::sqrt(self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w)
    }

    pub fn dot(self, rhs: Vec4) -> f32 {
        self.x*rhs.x + self.y*rhs.y + self.z*rhs.z + self.w*rhs.w
    }

    pub fn cross(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self.y*rhs.z - self.z*rhs.y,
            y: self.z*rhs.x - self.x*rhs.z,
            z: self.x*rhs.y - self.y*rhs.x,
            w: 0.0,
        }
    }

    pub fn normalized(self) -> Vec4 {
        self / self.length()
    }
}

impl Add for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Vec4 {
        unsafe { Vec4::from_simd(simd_add(self.to_simd(), rhs.to_simd())) }
    }
}

impl AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Vec4) {
        *self = unsafe { Vec4::from_simd(simd_add(self.to_simd(), rhs.to_simd())) }
    }
}

impl Div<f32> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: f32) -> Vec4 {
        unsafe { Vec4::from_simd(simd_div(self.to_simd(), f32x4(rhs, rhs, rhs, rhs))) }
    }
}

impl DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, rhs: f32) {
        *self = unsafe { Vec4::from_simd(simd_div(self.to_simd(), f32x4(rhs, rhs, rhs, rhs))) };
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: f32) -> Vec4 {
        unsafe { Vec4::from_simd(simd_mul(self.to_simd(), f32x4(rhs, rhs, rhs, rhs))) }
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Vec4 {
        unsafe { Vec4::from_simd(simd_mul(f32x4(self, self, self, self), rhs.to_simd())) }
    }
}

impl MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = unsafe { Vec4::from_simd(simd_mul(self.to_simd(), f32x4(rhs, rhs, rhs, rhs))) };
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
        unsafe { Vec4::from_simd(simd_sub(self.to_simd(), rhs.to_simd())) }
    }
}

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, rhs: Vec4) {
        *self = unsafe { Vec4::from_simd(simd_sub(self.to_simd(), rhs.to_simd())) };
    }
}