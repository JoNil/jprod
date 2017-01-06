#![allow(dead_code)]

use core::mem;
use vec4::Vec4;

pub use core::f32::*;
pub use core::f32::consts::*;

pub fn sin(a: f32) -> f32 {

    let mut res: f32 = unsafe { mem::uninitialized() };

    unsafe { asm!(
        r##"
            flds $1;
            fsin;
            fstps $0;
        "##
        : "=*m"(&mut res as *mut f32)
        : "*m"(&a as *const f32)
    ) };

    res
}

pub fn cos(a: f32) -> f32 {
    
    let mut res: f32 = unsafe { mem::uninitialized() };

    unsafe { asm!(
        r##"
            flds $1;
            fcos;
            fstps $0;
        "##
        : "=*m"(&mut res as *mut f32)
        : "*m"(&a as *const f32)
    ) };

    res
}

pub fn sin_cos(a: f32) -> (f32, f32) {
    let mut res_sin: f32 = unsafe { mem::uninitialized() };
    let mut res_cos: f32 = unsafe { mem::uninitialized() };

    unsafe { asm!(
        r##"
            flds $2;
            fsincos;
            fstps $1;
            fstps $0;
        "##
        : "=*m"(&mut res_sin as *mut f32)
        , "=*m"(&mut res_cos as *mut f32)
        : "*m"(&a as *const f32)
    ) };

    (res_sin, res_cos)
}

pub fn tan(a: f32) -> f32 {
    
    let (res_sin, res_cos) = sin_cos(a);

    res_sin / res_cos
}

pub fn sqrt(a: f32) -> f32 {
    Vec4::splat(a).pairwise_sqrt().x
}

pub fn min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

pub fn max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn clamp(x: f32, lb: f32, ub: f32) -> f32{
    min(max(x, lb), ub)
}