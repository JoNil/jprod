#![allow(dead_code)]

mod intrinsics {
    extern {
        #[link_name = "llvm.sin.f32"]
        pub fn sin_f32(a: f32) -> f32;

        #[link_name = "llvm.cos.f32"]
        pub fn cos_f32(a: f32) -> f32;
    }
}

pub fn sin_f32(a: f32) -> f32 {
    unsafe { intrinsics::sin_f32(a) }
}

pub fn cos_f32(a: f32) -> f32 {
    unsafe { intrinsics::cos_f32(a) }
}

pub fn tan_f32(a: f32) -> f32 {
    sin_f32(a) / cos_f32(a)
}