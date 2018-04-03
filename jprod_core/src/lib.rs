#![feature(abi_vectorcall)]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(stdsimd)]
#![feature(stmt_expr_attributes)]

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
pub mod math;
pub mod pool;
pub mod random;
pub mod shaders;
pub mod time;
pub mod utils;
pub mod win32;
pub mod window;