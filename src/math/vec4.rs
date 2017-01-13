#![allow(dead_code)]

use core::mem;
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
    pub extern "vectorcall" fn zero() -> Vec4 {
        Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }

    pub extern "vectorcall" fn x() -> Vec4 {
        Vec4 { x: 1.0, y: 0.0, z: 0.0, w: 0.0 }
    }

    pub extern "vectorcall" fn y() -> Vec4 {
        Vec4 { x: 0.0, y: 1.0, z: 0.0, w: 0.0 }
    }

    pub extern "vectorcall" fn z() -> Vec4 {
        Vec4 { x: 0.0, y: 0.0, z: 1.0, w: 0.0 }
    }

    pub extern "vectorcall" fn xyz(x: f32, y: f32, z: f32) -> Vec4 {
        Vec4 { x: x, y: y, z: z, w: 0.0 }
    }

    pub extern "vectorcall" fn xyzw(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 { x: x, y: y, z: z, w: w }
    }

    pub extern "vectorcall" fn splat(a: f32) -> Vec4 {
        Vec4 { x: a, y: a, z: a, w: a }
    }

    pub extern "vectorcall" fn with_w_0(self) -> Vec4 {
        Vec4 { x: self.x, y: self.y, z: self.z, w: 0.0 }
    }

    pub extern "vectorcall" fn with_w_1(self) -> Vec4 {
        Vec4 { x: self.x, y: self.y, z: self.z, w: 1.0 }
    }

    pub extern "vectorcall" fn from_simd(simd: f32x4) -> Vec4 {
        Vec4 { x: simd.0, y: simd.1, z: simd.2, w: simd.3 }
    }

    pub extern "vectorcall" fn to_simd(self) -> f32x4 {
        unsafe { mem::transmute(self) }
    }

    pub extern "vectorcall" fn length(self) -> f32 {
        Vec4::splat(self.dot(self)).pairwise_sqrt().x
    }
    pub extern "vectorcall" fn normalized(self) -> Vec4 {
        self.div(self.length())
    }

    pub extern "vectorcall" fn add(self, rhs: Vec4) -> Vec4 {
        unsafe { Vec4::from_simd(intrinsics::simd_add(self.to_simd(), rhs.to_simd())) }
    }

    pub extern "vectorcall" fn sub(self, rhs: Vec4) -> Vec4 {
        unsafe { Vec4::from_simd(intrinsics::simd_sub(self.to_simd(), rhs.to_simd())) }
    }

    pub extern "vectorcall" fn mul(self, rhs: f32) -> Vec4 {
        self.pairwise_mul(Vec4::splat(rhs))
    }

    pub extern "vectorcall" fn div(self, rhs: f32) -> Vec4 {
        unsafe { Vec4::from_simd(intrinsics::simd_div(self.to_simd(), Vec4::splat(rhs).to_simd())) }
    }

    pub extern "vectorcall" fn neg(self) -> Vec4 {
        Vec4 { x: -self.x, y: -self.y, z: -self.z, w: -self.w }
    }

    pub extern "vectorcall" fn dot(self, rhs: Vec4) -> f32 {
        
        let temp = self.pairwise_mul(rhs);
        temp.x + temp.y + temp.z + temp.w
    }

    pub extern "vectorcall" fn cross(self, rhs: Vec4) -> Vec4 {

        let a = Vec4::xyzw(self.y, self.z, self.x, 0.0);
        let b = Vec4::xyzw(rhs.z, rhs.x, rhs.y, 0.0);

        let c = Vec4::xyzw(self.z, self.x, self.y, 0.0);
        let d = Vec4::xyzw(rhs.y, rhs.z, rhs.x, 0.0);

        let temp1 = a.pairwise_mul(b);
        let temp2 = c.pairwise_mul(d);

        temp1.sub(temp2)
    }

    pub extern "vectorcall" fn pairwise_mul(self, rhs: Vec4) -> Vec4 {
        unsafe { Vec4::from_simd(intrinsics::simd_mul(self.to_simd(), rhs.to_simd())) }
    }

    pub extern "vectorcall" fn pairwise_div(self, rhs: Vec4) -> Vec4 {
        unsafe { Vec4::from_simd(intrinsics::simd_div(self.to_simd(), rhs.to_simd())) }
    }

    pub extern "vectorcall" fn pairwise_sqrt(self) -> Vec4 {
        unsafe { Vec4::from_simd(intrinsics::sqrt_v4f32(self.to_simd())) }
    }
}

#[macro_export]
macro_rules! vec4_shuffle {
    ($lhs:expr, $rhs:expr, $a:expr, $b:expr, $c:expr, $d:expr) => {
        unsafe { $crate::math::Vec4::from_simd($crate::intrinsics::simd_shuffle4($lhs.to_simd(), $rhs.to_simd(), [$a, $b, $c, $d])) }
    };
}