#![allow(dead_code)]

use core::intrinsics;

pub fn sin_f32(a: f32) -> f32 {
    unsafe { intrinsics::sinf32(a) }
}

pub fn cos_f32(a: f32) -> f32 {
    unsafe { intrinsics::cosf32(a) }
}

pub fn tan_f32(a: f32) -> f32 {
    sin_f32(a) / cos_f32(a)
}