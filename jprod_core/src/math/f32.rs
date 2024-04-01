use core::arch::asm;
use core::arch::x86_64::_mm_set_ps1;
use core::mem;
use math::Vec4;

pub use core::f32::consts::*;
pub use core::f32::*;

#[inline(always)]
pub fn sin(a: f32) -> f32 {
    let mut res: f32 = unsafe { mem::uninitialized() };
    unsafe {
        asm!(
            "fldl {1:e}",
            "fsin",
            "fstpl {0:e}",
            out(reg) res,
            in(reg) a,
        );
    }

    res
}

#[inline(always)]
pub fn cos(a: f32) -> f32 {
    let mut res: f32 = unsafe { mem::uninitialized() };
    unsafe {
        asm!(
            "fldl {1:e}",
            "fcos",
            "fstpl {0:e}",
            out(reg) res,
            in(reg) a,
        );
    }

    res
}

#[inline(always)]
pub fn sin_cos(a: f32) -> (f32, f32) {
    let mut res_sin: f32 = unsafe { mem::uninitialized() };
    let mut res_cos: f32 = unsafe { mem::uninitialized() };

    unsafe {
        asm!(
            "fldl {2:e}",
            "fsincos",
            "fstps {1:e}",
            "fstpl {0:e}",
            out(reg) res_sin,
            out(reg) res_cos,
            in(reg) a,
        );
    }

    (res_sin, res_cos)
}

#[inline(always)]
pub fn tan(a: f32) -> f32 {
    let (res_sin, res_cos) = sin_cos(a);

    res_sin / res_cos
}

#[inline]
pub fn sqrt(a: f32) -> f32 {
    Vec4::splat(a).pairwise_sqrt().x()
}

#[inline]
pub fn min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
pub fn max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

#[inline]
pub fn clamp(x: f32, lb: f32, ub: f32) -> f32 {
    min(max(x, lb), ub)
}
