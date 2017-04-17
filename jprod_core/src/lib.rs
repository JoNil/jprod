#![feature(abi_vectorcall)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(link_args)]
#![feature(link_llvm_intrinsics)]
#![feature(platform_intrinsics)]
#![feature(repr_simd)]
#![feature(simd_ffi)]

#![cfg_attr(not(feature = "use_std"), no_std)]

#[cfg(feature = "use_telemetry")]
extern crate telemetry;

#[macro_use]
extern crate telemetry_macro;

#[cfg(feature = "use_std")]
mod core {
    pub use std::*;
}

pub mod c_types;
pub mod camera;
pub mod gen;
pub mod gfx;
pub mod intrinsics;
pub mod math;
pub mod pool;
pub mod random;
pub mod shaders;
pub mod simdty;
pub mod time;
pub mod utils;
pub mod win32;
pub mod window;