use core::{arch::x86_64::*, mem};

#[macro_export]
macro_rules! vec4_swizzle {
    ($v:expr, $x:expr, $y:expr, $z:expr, $w:expr) => {{
        use core::arch::x86_64::*;

        $crate::math::Vec4(
            #[allow(unused_unsafe)]
            unsafe {
                _mm_shuffle_ps($v.0, $v.0, $x | ($y << 2) | ($z << 4) | ($w << 6))
            },
        )
    }};
}

#[macro_export]
macro_rules! vec4_shuffle {
    ($lhs:expr, $rhs:expr, $x:expr, $y:expr, $z:expr, $w:expr) => {{
        use core::arch::x86_64::*;

        $crate::math::Vec4(
            #[allow(unused_unsafe)]
            unsafe {
                _mm_shuffle_ps($lhs.0, $rhs.0, $x | ($y << 2) | ($z << 4) | ($w << 6))
            },
        )
    }};
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Vec4(pub __m128);

impl Vec4 {
    #[inline]
    pub extern "vectorcall" fn zero() -> Vec4 {
        unsafe { Vec4(_mm_set_ps(0.0, 0.0, 0.0, 0.0)) }
    }

    #[inline]
    pub extern "vectorcall" fn xyz(x: f32, y: f32, z: f32) -> Vec4 {
        unsafe { Vec4(_mm_set_ps(0.0, z, y, x)) }
    }

    #[inline]
    pub extern "vectorcall" fn xyzw(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        unsafe { Vec4(_mm_set_ps(w, z, y, x)) }
    }

    #[inline]
    pub extern "vectorcall" fn splat(a: f32) -> Vec4 {
        unsafe { Vec4(_mm_set1_ps(a)) }
    }

    #[inline]
    pub extern "vectorcall" fn from_slice(slice: &[f32; 3]) -> Vec4 {
        unsafe {
            Vec4::xyzw(
                *slice.get_unchecked(0),
                *slice.get_unchecked(1),
                *slice.get_unchecked(2),
                0.0,
            )
        }
    }

    #[inline]
    pub extern "vectorcall" fn to_slice(self) -> [f32; 3] {
        [self.x(), self.y(), self.z()]
    }

    #[inline]
    pub extern "vectorcall" fn x(self) -> f32 {
        unsafe {
            let temp: [f32; 4] = mem::transmute(self);
            *temp.get_unchecked(0)
        }
    }

    #[inline]
    pub extern "vectorcall" fn y(self) -> f32 {
        unsafe {
            let temp: [f32; 4] = mem::transmute(self);
            *temp.get_unchecked(1)
        }
    }

    #[inline]
    pub extern "vectorcall" fn z(self) -> f32 {
        unsafe {
            let temp: [f32; 4] = mem::transmute(self);
            *temp.get_unchecked(2)
        }
    }

    #[inline]
    pub extern "vectorcall" fn w(self) -> f32 {
        unsafe {
            let temp: [f32; 4] = mem::transmute(self);
            *temp.get_unchecked(3)
        }
    }

    #[inline]
    pub extern "vectorcall" fn with_x(self, x: f32) -> Vec4 {
        Vec4::xyzw(x, self.y(), self.z(), self.w())
    }

    #[inline]
    pub extern "vectorcall" fn with_y(self, y: f32) -> Vec4 {
        Vec4::xyzw(self.x(), y, self.z(), self.w())
    }

    #[inline]
    pub extern "vectorcall" fn with_z(self, z: f32) -> Vec4 {
        Vec4::xyzw(self.x(), self.y(), z, self.w())
    }

    #[inline]
    pub extern "vectorcall" fn with_w(self, w: f32) -> Vec4 {
        Vec4::xyzw(self.x(), self.y(), self.z(), w)
    }

    #[inline]
    pub extern "vectorcall" fn length(self) -> f32 {
        Vec4::splat(self.dot(self)).pairwise_sqrt().x()
    }
    #[inline]
    pub extern "vectorcall" fn normalized(self) -> Vec4 {
        self.div(self.length())
    }

    #[inline]
    pub extern "vectorcall" fn add(self, rhs: Vec4) -> Vec4 {
        unsafe { Vec4(_mm_add_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub extern "vectorcall" fn sub(self, rhs: Vec4) -> Vec4 {
        unsafe { Vec4(_mm_sub_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub extern "vectorcall" fn neg(self) -> Vec4 {
        unsafe { Vec4(_mm_sub_ps(Vec4::splat(0.0).0, self.0)) }
    }

    #[inline]
    pub extern "vectorcall" fn mul(self, rhs: f32) -> Vec4 {
        unsafe { Vec4(_mm_mul_ps(self.0, Vec4::splat(rhs).0)) }
    }

    #[inline]
    pub extern "vectorcall" fn div(self, rhs: f32) -> Vec4 {
        unsafe { Vec4(_mm_div_ps(self.0, Vec4::splat(rhs).0)) }
    }

    #[inline]
    pub extern "vectorcall" fn dot(self, rhs: Vec4) -> f32 {
        let temp = self.pairwise_mul(rhs);
        temp.x() + temp.y() + temp.z() + temp.w()
    }

    #[inline]
    pub extern "vectorcall" fn cross(self, rhs: Vec4) -> Vec4 {
        let a = vec4_swizzle!(self, 1, 2, 0, 3);
        let b = vec4_swizzle!(rhs, 2, 0, 1, 3);

        let c = vec4_swizzle!(self, 2, 0, 1, 3);
        let d = vec4_swizzle!(rhs, 1, 2, 0, 3);

        let temp1 = a.pairwise_mul(b);
        let temp2 = c.pairwise_mul(d);

        temp1.sub(temp2)
    }

    #[inline]
    pub extern "vectorcall" fn pairwise_mul(self, rhs: Vec4) -> Vec4 {
        unsafe { Vec4(_mm_mul_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub extern "vectorcall" fn pairwise_div(self, rhs: Vec4) -> Vec4 {
        unsafe { Vec4(_mm_div_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub extern "vectorcall" fn pairwise_sqrt(self) -> Vec4 {
        unsafe { Vec4(_mm_sqrt_ps(self.0)) }
    }
}

#[test]
fn vec4_test_access() {
    let a = Vec4::xyzw(1.0, 2.0, 3.0, 4.0);
    assert_eq!(a.x(), 1.0);
    assert_eq!(a.y(), 2.0);
    assert_eq!(a.z(), 3.0);
    assert_eq!(a.w(), 4.0);
}

#[test]
fn vec4_test_cross() {
    let a = Vec4::xyzw(1.0, 2.0, 3.0, 4.0);
    let b = Vec4::xyzw(5.0, 6.0, 7.0, 8.0);
    let c = a.cross(b);

    assert_eq!(c.x(), -4.0);
    assert_eq!(c.y(), 8.0);
    assert_eq!(c.z(), -4.0);
}
