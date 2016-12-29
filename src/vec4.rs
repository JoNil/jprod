#![allow(dead_code)]

use core::ops::Add;
use core::ops::AddAssign;

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
}

impl Add for Vec4 {
    type Output = Vec4;

    fn add(self, other: Vec4) -> Vec4 {
        Vec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl AddAssign for Vec4 {
    fn add_assign(&mut self, other: Vec4) {
        *self = Vec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        };
    }
}