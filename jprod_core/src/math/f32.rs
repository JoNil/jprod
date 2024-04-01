use core::{arch::asm, mem};
use math::Vec4;

pub use core::f32::consts::*;
pub use core::f32::*;

#[inline(always)]
pub fn sin(a: f32) -> f32 {
    let mut res: f32 = unsafe { mem::uninitialized() };

    unsafe {
        asm!(
            "fld dword ptr [{1}]",
            "fsin",
            "fstp dword ptr [{0}]",
            in(reg) &mut res as *mut f32,
            in(reg) &a,
            options(nostack),
        );
    }

    res
}

#[inline(always)]
pub fn cos(a: f32) -> f32 {
    let mut res: f32 = unsafe { mem::uninitialized() };

    unsafe {
        asm!(
            "fld dword ptr [{1}]",
            "fcos",
            "fstp dword ptr [{0}]",
            in(reg) &mut res as *mut f32,
            in(reg) &a,
            options(nostack),
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
            "fld dword ptr [{2}]",
            "fsincos",
            "fstp dword ptr [{1}]",
            "fstp dword ptr [{0}]",
            in(reg) &mut res_sin as *mut f32,
            in(reg) &mut res_cos as *mut f32,
            in(reg) &a,
            options(nostack),
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
