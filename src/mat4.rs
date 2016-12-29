use core::f32;
use core::mem;
use core::ops::Mul;
use core::ops::MulAssign;
use math;

pub struct Mat4 {
    pub m: (
        (f32, f32, f32, f32),
        (f32, f32, f32, f32),
        (f32, f32, f32, f32),
        (f32, f32, f32, f32),
    ),
}

impl Mat4 {

    pub fn identity() -> Mat4 {
        Mat4 {
            m: (
                (1.0, 0.0, 0.0, 0.0),
                (0.0, 1.0, 0.0, 0.0),
                (0.0, 0.0, 1.0, 0.0),
                (0.0, 0.0, 0.0, 1.0),
            ),
        }
    }

    pub fn frustum(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
        Mat4 {
            m: (
                (2.0*near/(right-left), 0.0, 0.0, 0.0),
                (0.0, 2.0*near/(top-bottom), 0.0, 0.0),
                ((right+left)/(right-left), (top+bottom)/(top-bottom), -(far+near)/(far-near), -1.0),
                (0.0, 0.0, -2.0*far*near/(far-near), 0.0),
            ),
        }
    }

    pub fn perspective(horizontal_fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4 {

        let height = near*math::tan_f32(horizontal_fov*f32::consts::PI/360.0);
        let width = height*aspect_ratio;
        Mat4::frustum(-width, width, -height, height, near, far)
    }

    pub fn as_array(&self) -> &[[f32; 4]; 4] {
        unsafe { mem::transmute(&self.m) }
    }
}

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Mat4 {
            m: (
                (
                    (rhs.m.0).0*(self.m.0).0+(rhs.m.0).1*(self.m.1).0+(rhs.m.0).2*(self.m.2).0+(rhs.m.0).3*(self.m.3).0,
                    (rhs.m.0).0*(self.m.0).1+(rhs.m.0).1*(self.m.1).1+(rhs.m.0).2*(self.m.2).1+(rhs.m.0).3*(self.m.3).1,
                    (rhs.m.0).0*(self.m.0).2+(rhs.m.0).1*(self.m.1).2+(rhs.m.0).2*(self.m.2).2+(rhs.m.0).3*(self.m.3).2,
                    (rhs.m.0).0*(self.m.0).3+(rhs.m.0).1*(self.m.1).3+(rhs.m.0).2*(self.m.2).3+(rhs.m.0).3*(self.m.3).3,
                ),
                (
                    (rhs.m.1).0*(self.m.0).0+(rhs.m.1).1*(self.m.1).0+(rhs.m.1).2*(self.m.2).0+(rhs.m.1).3*(self.m.3).0,
                    (rhs.m.1).0*(self.m.0).1+(rhs.m.1).1*(self.m.1).1+(rhs.m.1).2*(self.m.2).1+(rhs.m.1).3*(self.m.3).1,
                    (rhs.m.1).0*(self.m.0).2+(rhs.m.1).1*(self.m.1).2+(rhs.m.1).2*(self.m.2).2+(rhs.m.1).3*(self.m.3).2,
                    (rhs.m.1).0*(self.m.0).3+(rhs.m.1).1*(self.m.1).3+(rhs.m.1).2*(self.m.2).3+(rhs.m.1).3*(self.m.3).3,
                ),
                (
                    (rhs.m.2).0*(self.m.0).0+(rhs.m.2).1*(self.m.1).0+(rhs.m.2).2*(self.m.2).0+(rhs.m.2).3*(self.m.3).0,
                    (rhs.m.2).0*(self.m.0).1+(rhs.m.2).1*(self.m.1).1+(rhs.m.2).2*(self.m.2).1+(rhs.m.2).3*(self.m.3).1,
                    (rhs.m.2).0*(self.m.0).2+(rhs.m.2).1*(self.m.1).2+(rhs.m.2).2*(self.m.2).2+(rhs.m.2).3*(self.m.3).2,
                    (rhs.m.2).0*(self.m.0).3+(rhs.m.2).1*(self.m.1).3+(rhs.m.2).2*(self.m.2).3+(rhs.m.2).3*(self.m.3).3,
                ),
                (
                    (rhs.m.3).0*(self.m.0).0+(rhs.m.3).1*(self.m.1).0+(rhs.m.3).2*(self.m.2).0+(rhs.m.3).3*(self.m.3).0,
                    (rhs.m.3).0*(self.m.0).1+(rhs.m.3).1*(self.m.1).1+(rhs.m.3).2*(self.m.2).1+(rhs.m.3).3*(self.m.3).1,
                    (rhs.m.3).0*(self.m.0).2+(rhs.m.3).1*(self.m.1).2+(rhs.m.3).2*(self.m.2).2+(rhs.m.3).3*(self.m.3).2,
                    (rhs.m.3).0*(self.m.0).3+(rhs.m.3).1*(self.m.1).3+(rhs.m.3).2*(self.m.2).3+(rhs.m.3).3*(self.m.3).3,
                )
            ),
        }
    }
}