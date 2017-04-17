#![feature(abi_vectorcall)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(link_args)]
#![feature(link_llvm_intrinsics)]
#![feature(platform_intrinsics)]
#![feature(repr_simd)]
#![feature(simd_ffi)]


#[cfg(feature = "use_telemetry")]
extern crate telemetry;

#[macro_use]
extern crate telemetry_macro;

extern crate jprod_core;

fn main() {
    println!("Hello editor");   
}