#![allow(dead_code)]

use vec4::Vec4;
use win32;

pub use core::f32::*;

pub fn sin(a: f32) -> f32 {
    win32::sin(a as f64) as f32
}

pub fn cos(a: f32) -> f32 {
    win32::cos(a as f64) as f32
}

pub fn tan(a: f32) -> f32 {
    sin(a) / cos(a)
}

pub fn sqrt(a: f32) -> f32 {
    Vec4::splat(a).sqrt_x4().x
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