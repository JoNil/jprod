use core::{arch::asm, mem::MaybeUninit};
use math::Vec4;

pub use core::f32::{consts::*, *};

#[inline(always)]
pub fn sin(a: f32) -> f32 {
    let mut res: MaybeUninit<f32> = MaybeUninit::uninit();

    unsafe {
        asm!(
            "fld dword ptr [{1}]",
            "fsin",
            "fstp dword ptr [{0}]",
            in(reg) res.as_mut_ptr(),
            in(reg) &a,
            options(nostack),
        );

        res.assume_init()
    }
}

#[inline(always)]
pub fn cos(a: f32) -> f32 {
    let mut res: MaybeUninit<f32> = MaybeUninit::uninit();

    unsafe {
        asm!(
            "fld dword ptr [{1}]",
            "fcos",
            "fstp dword ptr [{0}]",
            in(reg) res.as_mut_ptr(),
            in(reg) &a,
            options(nostack),
        );

        res.assume_init()
    }
}

#[inline(always)]
pub fn sin_cos(a: f32) -> (f32, f32) {
    let mut res_sin: MaybeUninit<f32> = MaybeUninit::uninit();
    let mut res_cos: MaybeUninit<f32> = MaybeUninit::uninit();

    unsafe {
        asm!(
            "fld dword ptr [{2}]",
            "fsincos",
            "fstp dword ptr [{1}]",
            "fstp dword ptr [{0}]",
            in(reg) res_sin.as_mut_ptr(),
            in(reg) res_cos.as_mut_ptr(),
            in(reg) &a,
            options(nostack),
        );

        (res_sin.assume_init(), res_cos.assume_init())
    }
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
