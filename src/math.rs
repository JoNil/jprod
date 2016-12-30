#![allow(dead_code)]

use win32;

pub fn sin_f32(a: f32) -> f32 {
    win32::sin(a as f64) as f32
}

pub fn cos_f32(a: f32) -> f32 {
    win32::cos(a as f64) as f32
}

pub fn tan_f32(a: f32) -> f32 {
    sin_f32(a) / cos_f32(a)
}