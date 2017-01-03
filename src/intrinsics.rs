#![allow(dead_code)]

extern "platform-intrinsic" {
    pub fn simd_add<T>(x: T, y: T) -> T;
    pub fn simd_sub<T>(x: T, y: T) -> T;
    pub fn simd_mul<T>(x: T, y: T) -> T;
    pub fn simd_div<T>(x: T, y: T) -> T;
    pub fn simd_shl<T>(x: T, y: T) -> T;
    pub fn simd_shr<T>(x: T, y: T) -> T;
    pub fn simd_and<T>(x: T, y: T) -> T;
    pub fn simd_or<T>(x: T, y: T) -> T;
    pub fn simd_xor<T>(x: T, y: T) -> T;
}

extern {
    /// The `llvm.va_start` intrinsic.
    #[link_name = "llvm.va_start"]
    pub fn vastart(a: *mut i8) -> ();
    /// The `llvm.va_copy` intrinsic.
    #[link_name = "llvm.va_copy"]
    pub fn vacopy(a: *mut i8, b: *mut i8) -> ();
    /// The `llvm.va_end` intrinsic.
    #[link_name = "llvm.va_end"]
    pub fn vaend(a: *mut i8) -> ();
    /// The `llvm.gcroot` intrinsic.
    #[link_name = "llvm.gcroot"]
    pub fn gcroot(a: *mut *mut i8, b: *mut i8) -> ();
    /// The `llvm.gcread` intrinsic.
    #[link_name = "llvm.gcread"]
    pub fn gcread(a: *mut i8, b: *mut *mut i8) -> *mut i8;
    /// The `llvm.gcwrite` intrinsic.
    #[link_name = "llvm.gcwrite"]
    pub fn gcwrite(a: *mut i8, b: *mut i8, c: *mut *mut i8) -> ();
    /// The `llvm.returnaddress` intrinsic.
    #[link_name = "llvm.returnaddress"]
    pub fn returnaddress(a: i32) -> *mut i8;
    /// The `llvm.frameaddress` intrinsic.
    #[link_name = "llvm.frameaddress"]
    pub fn frameaddress(a: i32) -> *mut i8;
    /// The `llvm.stacksave` intrinsic; known as `__builtin_stack_save` in GCC.
    #[link_name = "llvm.stacksave"]
    pub fn stacksave() -> *mut i8;
    /// The `llvm.stackrestore` intrinsic; known as `__builtin_stack_restore` in GCC.
    #[link_name = "llvm.stackrestore"]
    pub fn stackrestore(a: *mut i8) -> ();
    /// The `llvm.prefetch` intrinsic.
    #[link_name = "llvm.prefetch"]
    pub fn prefetch(a: *mut i8, b: i32, c: i32, d: i32) -> ();
    /// The `llvm.pcmarker` intrinsic.
    #[link_name = "llvm.pcmarker"]
    pub fn pcmarker(a: i32) -> ();
    /// The `llvm.readcyclecounter` intrinsic.
    #[link_name = "llvm.readcyclecounter"]
    pub fn readcyclecounter() -> i64;
    /// The `llvm.assume` intrinsic.
    #[link_name = "llvm.assume"]
    pub fn assume(a: bool) -> ();
    /// The `llvm.stackprotector` intrinsic.
    #[link_name = "llvm.stackprotector"]
    pub fn stackprotector(a: *mut i8, b: *mut *mut i8) -> ();
    /// The `llvm.stackprotectorcheck` intrinsic.
    #[link_name = "llvm.stackprotectorcheck"]
    pub fn stackprotectorcheck(a: *mut *mut i8) -> ();
    /// The `llvm.memcpy.p0i8.p0i8.v16i8` intrinsic.
    #[link_name = "llvm.memcpy.p0i8.p0i8.v16i8"]
    pub fn memcpy_p0i8_p0i8_v16i8(a: *mut i8, b: *mut i8, c: ::simdty::i8x16, d: i32, e: bool) -> ();
    /// The `llvm.memcpy.p0i8.p0i8.i8` intrinsic.
    #[link_name = "llvm.memcpy.p0i8.p0i8.i8"]
    pub fn memcpy_p0i8_p0i8_i8(a: *mut i8, b: *mut i8, c: i8, d: i32, e: bool) -> ();
    /// The `llvm.memcpy.p0i8.p0i8.v8i16` intrinsic.
    #[link_name = "llvm.memcpy.p0i8.p0i8.v8i16"]
    pub fn memcpy_p0i8_p0i8_v8i16(a: *mut i8, b: *mut i8, c: ::simdty::i16x8, d: i32, e: bool) -> ();
    /// The `llvm.memcpy.p0i8.p0i8.i16` intrinsic.
    #[link_name = "llvm.memcpy.p0i8.p0i8.i16"]
    pub fn memcpy_p0i8_p0i8_i16(a: *mut i8, b: *mut i8, c: i16, d: i32, e: bool) -> ();
    /// The `llvm.memcpy.p0i8.p0i8.v4i32` intrinsic.
    #[link_name = "llvm.memcpy.p0i8.p0i8.v4i32"]
    pub fn memcpy_p0i8_p0i8_v4i32(a: *mut i8, b: *mut i8, c: ::simdty::i32x4, d: i32, e: bool) -> ();
    /// The `llvm.memcpy.p0i8.p0i8.i32` intrinsic.
    #[link_name = "llvm.memcpy.p0i8.p0i8.i32"]
    pub fn memcpy_p0i8_p0i8_i32(a: *mut i8, b: *mut i8, c: i32, d: i32, e: bool) -> ();
    /// The `llvm.memcpy.p0i8.p0i8.v2i64` intrinsic.
    #[link_name = "llvm.memcpy.p0i8.p0i8.v2i64"]
    pub fn memcpy_p0i8_p0i8_v2i64(a: *mut i8, b: *mut i8, c: ::simdty::i64x2, d: i32, e: bool) -> ();
    /// The `llvm.memcpy.p0i8.p0i8.i64` intrinsic.
    #[link_name = "llvm.memcpy.p0i8.p0i8.i64"]
    pub fn memcpy_p0i8_p0i8_i64(a: *mut i8, b: *mut i8, c: i64, d: i32, e: bool) -> ();
    /// The `llvm.memmove.p0i8.p0i8.v16i8` intrinsic.
    #[link_name = "llvm.memmove.p0i8.p0i8.v16i8"]
    pub fn memmove_p0i8_p0i8_v16i8(a: *mut i8, b: *mut i8, c: ::simdty::i8x16, d: i32, e: bool) -> ();
    /// The `llvm.memmove.p0i8.p0i8.i8` intrinsic.
    #[link_name = "llvm.memmove.p0i8.p0i8.i8"]
    pub fn memmove_p0i8_p0i8_i8(a: *mut i8, b: *mut i8, c: i8, d: i32, e: bool) -> ();
    /// The `llvm.memmove.p0i8.p0i8.v8i16` intrinsic.
    #[link_name = "llvm.memmove.p0i8.p0i8.v8i16"]
    pub fn memmove_p0i8_p0i8_v8i16(a: *mut i8, b: *mut i8, c: ::simdty::i16x8, d: i32, e: bool) -> ();
    /// The `llvm.memmove.p0i8.p0i8.i16` intrinsic.
    #[link_name = "llvm.memmove.p0i8.p0i8.i16"]
    pub fn memmove_p0i8_p0i8_i16(a: *mut i8, b: *mut i8, c: i16, d: i32, e: bool) -> ();
    /// The `llvm.memmove.p0i8.p0i8.v4i32` intrinsic.
    #[link_name = "llvm.memmove.p0i8.p0i8.v4i32"]
    pub fn memmove_p0i8_p0i8_v4i32(a: *mut i8, b: *mut i8, c: ::simdty::i32x4, d: i32, e: bool) -> ();
    /// The `llvm.memmove.p0i8.p0i8.i32` intrinsic.
    #[link_name = "llvm.memmove.p0i8.p0i8.i32"]
    pub fn memmove_p0i8_p0i8_i32(a: *mut i8, b: *mut i8, c: i32, d: i32, e: bool) -> ();
    /// The `llvm.memmove.p0i8.p0i8.v2i64` intrinsic.
    #[link_name = "llvm.memmove.p0i8.p0i8.v2i64"]
    pub fn memmove_p0i8_p0i8_v2i64(a: *mut i8, b: *mut i8, c: ::simdty::i64x2, d: i32, e: bool) -> ();
    /// The `llvm.memmove.p0i8.p0i8.i64` intrinsic.
    #[link_name = "llvm.memmove.p0i8.p0i8.i64"]
    pub fn memmove_p0i8_p0i8_i64(a: *mut i8, b: *mut i8, c: i64, d: i32, e: bool) -> ();
    /// The `llvm.memset.p0i8.v16i8` intrinsic.
    #[link_name = "llvm.memset.p0i8.v16i8"]
    pub fn memset_p0i8_v16i8(a: *mut i8, b: i8, c: ::simdty::i8x16, d: i32, e: bool) -> ();
    /// The `llvm.memset.p0i8.i8` intrinsic.
    #[link_name = "llvm.memset.p0i8.i8"]
    pub fn memset_p0i8_i8(a: *mut i8, b: i8, c: i8, d: i32, e: bool) -> ();
    /// The `llvm.memset.p0i8.v8i16` intrinsic.
    #[link_name = "llvm.memset.p0i8.v8i16"]
    pub fn memset_p0i8_v8i16(a: *mut i8, b: i8, c: ::simdty::i16x8, d: i32, e: bool) -> ();
    /// The `llvm.memset.p0i8.i16` intrinsic.
    #[link_name = "llvm.memset.p0i8.i16"]
    pub fn memset_p0i8_i16(a: *mut i8, b: i8, c: i16, d: i32, e: bool) -> ();
    /// The `llvm.memset.p0i8.v4i32` intrinsic.
    #[link_name = "llvm.memset.p0i8.v4i32"]
    pub fn memset_p0i8_v4i32(a: *mut i8, b: i8, c: ::simdty::i32x4, d: i32, e: bool) -> ();
    /// The `llvm.memset.p0i8.i32` intrinsic.
    #[link_name = "llvm.memset.p0i8.i32"]
    pub fn memset_p0i8_i32(a: *mut i8, b: i8, c: i32, d: i32, e: bool) -> ();
    /// The `llvm.memset.p0i8.v2i64` intrinsic.
    #[link_name = "llvm.memset.p0i8.v2i64"]
    pub fn memset_p0i8_v2i64(a: *mut i8, b: i8, c: ::simdty::i64x2, d: i32, e: bool) -> ();
    /// The `llvm.memset.p0i8.i64` intrinsic.
    #[link_name = "llvm.memset.p0i8.i64"]
    pub fn memset_p0i8_i64(a: *mut i8, b: i8, c: i64, d: i32, e: bool) -> ();
    /// The `llvm.fma.v4f32` intrinsic.
    #[link_name = "llvm.fma.v4f32"]
    pub fn fma_v4f32(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
    /// The `llvm.fma.f32` intrinsic.
    #[link_name = "llvm.fma.f32"]
    pub fn fma_f32(a: f32, b: f32, c: f32) -> f32;
    /// The `llvm.fma.v2f64` intrinsic.
    #[link_name = "llvm.fma.v2f64"]
    pub fn fma_v2f64(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
    /// The `llvm.fma.f64` intrinsic.
    #[link_name = "llvm.fma.f64"]
    pub fn fma_f64(a: f64, b: f64, c: f64) -> f64;
    /// The `llvm.fmuladd.v4f32` intrinsic.
    #[link_name = "llvm.fmuladd.v4f32"]
    pub fn fmuladd_v4f32(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
    /// The `llvm.fmuladd.f32` intrinsic.
    #[link_name = "llvm.fmuladd.f32"]
    pub fn fmuladd_f32(a: f32, b: f32, c: f32) -> f32;
    /// The `llvm.fmuladd.v2f64` intrinsic.
    #[link_name = "llvm.fmuladd.v2f64"]
    pub fn fmuladd_v2f64(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
    /// The `llvm.fmuladd.f64` intrinsic.
    #[link_name = "llvm.fmuladd.f64"]
    pub fn fmuladd_f64(a: f64, b: f64, c: f64) -> f64;
    /// The `llvm.sqrt.v4f32` intrinsic.
    #[link_name = "llvm.sqrt.v4f32"]
    pub fn sqrt_v4f32(a: ::simdty::f32x4) -> ::simdty::f32x4;
    /// The `llvm.sqrt.f32` intrinsic.
    #[link_name = "llvm.sqrt.f32"]
    pub fn sqrt_f32(a: f32) -> f32;
    /// The `llvm.sqrt.v2f64` intrinsic.
    #[link_name = "llvm.sqrt.v2f64"]
    pub fn sqrt_v2f64(a: ::simdty::f64x2) -> ::simdty::f64x2;
    /// The `llvm.sqrt.f64` intrinsic.
    #[link_name = "llvm.sqrt.f64"]
    pub fn sqrt_f64(a: f64) -> f64;
    /// The `llvm.powi.v4f32` intrinsic.
    #[link_name = "llvm.powi.v4f32"]
    pub fn powi_v4f32(a: ::simdty::f32x4, b: i32) -> ::simdty::f32x4;
    /// The `llvm.powi.f32` intrinsic.
    #[link_name = "llvm.powi.f32"]
    pub fn powi_f32(a: f32, b: i32) -> f32;
    /// The `llvm.powi.v2f64` intrinsic.
    #[link_name = "llvm.powi.v2f64"]
    pub fn powi_v2f64(a: ::simdty::f64x2, b: i32) -> ::simdty::f64x2;
    /// The `llvm.powi.f64` intrinsic.
    #[link_name = "llvm.powi.f64"]
    pub fn powi_f64(a: f64, b: i32) -> f64;
    /// The `llvm.sin.v4f32` intrinsic.
    #[link_name = "llvm.sin.v4f32"]
    pub fn sin_v4f32(a: ::simdty::f32x4) -> ::simdty::f32x4;
    /// The `llvm.sin.f32` intrinsic.
    #[link_name = "llvm.sin.f32"]
    pub fn sin_f32(a: f32) -> f32;
    /// The `llvm.sin.v2f64` intrinsic.
    #[link_name = "llvm.sin.v2f64"]
    pub fn sin_v2f64(a: ::simdty::f64x2) -> ::simdty::f64x2;
    /// The `llvm.sin.f64` intrinsic.
    #[link_name = "llvm.sin.f64"]
    pub fn sin_f64(a: f64) -> f64;
    /// The `llvm.cos.v4f32` intrinsic.
    #[link_name = "llvm.cos.v4f32"]
    pub fn cos_v4f32(a: ::simdty::f32x4) -> ::simdty::f32x4;
    /// The `llvm.cos.f32` intrinsic.
    #[link_name = "llvm.cos.f32"]
    pub fn cos_f32(a: f32) -> f32;
    /// The `llvm.cos.v2f64` intrinsic.
    #[link_name = "llvm.cos.v2f64"]
    pub fn cos_v2f64(a: ::simdty::f64x2) -> ::simdty::f64x2;
    /// The `llvm.cos.f64` intrinsic.
    #[link_name = "llvm.cos.f64"]
    pub fn cos_f64(a: f64) -> f64;
    /// The `llvm.pow.v4f32` intrinsic.
    #[link_name = "llvm.pow.v4f32"]
    pub fn pow_v4f32(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
    /// The `llvm.pow.f32` intrinsic.
    #[link_name = "llvm.pow.f32"]
    pub fn pow_f32(a: f32, b: f32) -> f32;
    /// The `llvm.pow.v2f64` intrinsic.
    #[link_name = "llvm.pow.v2f64"]
    pub fn pow_v2f64(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
    /// The `llvm.pow.f64` intrinsic.
    #[link_name = "llvm.pow.f64"]
    pub fn pow_f64(a: f64, b: f64) -> f64;
    /// The `llvm.log.v4f32` intrinsic.
    #[link_name = "llvm.log.v4f32"]
    pub fn log_v4f32(a: ::simdty::f32x4) -> ::simdty::f32x4;
    /// The `llvm.log.f32` intrinsic.
    #[link_name = "llvm.log.f32"]
    pub fn log_f32(a: f32) -> f32;
    /// The `llvm.log.v2f64` intrinsic.
    #[link_name = "llvm.log.v2f64"]
    pub fn log_v2f64(a: ::simdty::f64x2) -> ::simdty::f64x2;
    /// The `llvm.log.f64` intrinsic.
    #[link_name = "llvm.log.f64"]
    pub fn log_f64(a: f64) -> f64;
    /// The `llvm.log10.v4f32` intrinsic.
    #[link_name = "llvm.log10.v4f32"]
    pub fn log10_v4f32(a: ::simdty::f32x4) -> ::simdty::f32x4;
    /// The `llvm.log10.f32` intrinsic.
    #[link_name = "llvm.log10.f32"]
    pub fn log10_f32(a: f32) -> f32;
    /// The `llvm.log10.v2f64` intrinsic.
    #[link_name = "llvm.log10.v2f64"]
    pub fn log10_v2f64(a: ::simdty::f64x2) -> ::simdty::f64x2;
    /// The `llvm.log10.f64` intrinsic.
    #[link_name = "llvm.log10.f64"]
    pub fn log10_f64(a: f64) -> f64;
    /// The `llvm.log2.v4f32` intrinsic.
    #[link_name = "llvm.log2.v4f32"]
    pub fn log2_v4f32(a: ::simdty::f32x4) -> ::simdty::f32x4;
    /// The `llvm.log2.f32` intrinsic.
    #[link_name = "llvm.log2.f32"]
    pub fn log2_f32(a: f32) -> f32;
    /// The `llvm.log2.v2f64` intrinsic.
    #[link_name = "llvm.log2.v2f64"]
    pub fn log2_v2f64(a: ::simdty::f64x2) -> ::simdty::f64x2;
    /// The `llvm.log2.f64` intrinsic.
    #[link_name = "llvm.log2.f64"]
    pub fn log2_f64(a: f64) -> f64;
    /// The `llvm.exp.v4f32` intrinsic.
    #[link_name = "llvm.exp.v4f32"]
    pub fn exp_v4f32(a: ::simdty::f32x4) -> ::simdty::f32x4;
    /// The `llvm.exp.f32` intrinsic.
    #[link_name = "llvm.exp.f32"]
    pub fn exp_f32(a: f32) -> f32;
    /// The `llvm.exp.v2f64` intrinsic.
    #[link_name = "llvm.exp.v2f64"]
    pub fn exp_v2f64(a: ::simdty::f64x2) -> ::simdty::f64x2;
    /// The `llvm.exp.f64` intrinsic.
    #[link_name = "llvm.exp.f64"]
    pub fn exp_f64(a: f64) -> f64;
    /// The `llvm.exp2.v4f32` intrinsic.
    #[link_name = "llvm.exp2.v4f32"]
    pub fn exp2_v4f32(a: ::simdty::f32x4) -> ::simdty::f32x4;
    /// The `llvm.exp2.f32` intrinsic.
    #[link_name = "llvm.exp2.f32"]
    pub fn exp2_f32(a: f32) -> f32;
    /// The `llvm.exp2.v2f64` intrinsic.
    #[link_name = "llvm.exp2.v2f64"]
    pub fn exp2_v2f64(a: ::simdty::f64x2) -> ::simdty::f64x2;
    /// The `llvm.exp2.f64` intrinsic.
    #[link_name = "llvm.exp2.f64"]
    pub fn exp2_f64(a: f64) -> f64;
    /// The `llvm.fabs.v4f32` intrinsic.
    #[link_name = "llvm.fabs.v4f32"]
    pub fn fabs_v4f32(a: ::simdty::f32x4) -> ::simdty::f32x4;
    /// The `llvm.fabs.f32` intrinsic.
    #[link_name = "llvm.fabs.f32"]
    pub fn fabs_f32(a: f32) -> f32;
    /// The `llvm.fabs.v2f64` intrinsic.
    #[link_name = "llvm.fabs.v2f64"]
    pub fn fabs_v2f64(a: ::simdty::f64x2) -> ::simdty::f64x2;
    /// The `llvm.fabs.f64` intrinsic.
    #[link_name = "llvm.fabs.f64"]
    pub fn fabs_f64(a: f64) -> f64;
    /// The `llvm.copysign.v4f32` intrinsic.
    #[link_name = "llvm.copysign.v4f32"]
    pub fn copysign_v4f32(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
    /// The `llvm.copysign.f32` intrinsic.
    #[link_name = "llvm.copysign.f32"]
    pub fn copysign_f32(a: f32, b: f32) -> f32;
    /// The `llvm.copysign.v2f64` intrinsic.
    #[link_name = "llvm.copysign.v2f64"]
    pub fn copysign_v2f64(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
    /// The `llvm.copysign.f64` intrinsic.
    #[link_name = "llvm.copysign.f64"]
    pub fn copysign_f64(a: f64, b: f64) -> f64;
    /// The `llvm.floor.v4f32` intrinsic.
    #[link_name = "llvm.floor.v4f32"]
    pub fn floor_v4f32(a: ::simdty::f32x4) -> ::simdty::f32x4;
    /// The `llvm.floor.f32` intrinsic.
    #[link_name = "llvm.floor.f32"]
    pub fn floor_f32(a: f32) -> f32;
    /// The `llvm.floor.v2f64` intrinsic.
    #[link_name = "llvm.floor.v2f64"]
    pub fn floor_v2f64(a: ::simdty::f64x2) -> ::simdty::f64x2;
    /// The `llvm.floor.f64` intrinsic.
    #[link_name = "llvm.floor.f64"]
    pub fn floor_f64(a: f64) -> f64;
    /// The `llvm.ceil.v4f32` intrinsic.
    #[link_name = "llvm.ceil.v4f32"]
    pub fn ceil_v4f32(a: ::simdty::f32x4) -> ::simdty::f32x4;
    /// The `llvm.ceil.f32` intrinsic.
    #[link_name = "llvm.ceil.f32"]
    pub fn ceil_f32(a: f32) -> f32;
    /// The `llvm.ceil.v2f64` intrinsic.
    #[link_name = "llvm.ceil.v2f64"]
    pub fn ceil_v2f64(a: ::simdty::f64x2) -> ::simdty::f64x2;
    /// The `llvm.ceil.f64` intrinsic.
    #[link_name = "llvm.ceil.f64"]
    pub fn ceil_f64(a: f64) -> f64;
    /// The `llvm.trunc.v4f32` intrinsic.
    #[link_name = "llvm.trunc.v4f32"]
    pub fn trunc_v4f32(a: ::simdty::f32x4) -> ::simdty::f32x4;
    /// The `llvm.trunc.f32` intrinsic.
    #[link_name = "llvm.trunc.f32"]
    pub fn trunc_f32(a: f32) -> f32;
    /// The `llvm.trunc.v2f64` intrinsic.
    #[link_name = "llvm.trunc.v2f64"]
    pub fn trunc_v2f64(a: ::simdty::f64x2) -> ::simdty::f64x2;
    /// The `llvm.trunc.f64` intrinsic.
    #[link_name = "llvm.trunc.f64"]
    pub fn trunc_f64(a: f64) -> f64;
    /// The `llvm.rint.v4f32` intrinsic.
    #[link_name = "llvm.rint.v4f32"]
    pub fn rint_v4f32(a: ::simdty::f32x4) -> ::simdty::f32x4;
    /// The `llvm.rint.f32` intrinsic.
    #[link_name = "llvm.rint.f32"]
    pub fn rint_f32(a: f32) -> f32;
    /// The `llvm.rint.v2f64` intrinsic.
    #[link_name = "llvm.rint.v2f64"]
    pub fn rint_v2f64(a: ::simdty::f64x2) -> ::simdty::f64x2;
    /// The `llvm.rint.f64` intrinsic.
    #[link_name = "llvm.rint.f64"]
    pub fn rint_f64(a: f64) -> f64;
    /// The `llvm.nearbyint.v4f32` intrinsic.
    #[link_name = "llvm.nearbyint.v4f32"]
    pub fn nearbyint_v4f32(a: ::simdty::f32x4) -> ::simdty::f32x4;
    /// The `llvm.nearbyint.f32` intrinsic.
    #[link_name = "llvm.nearbyint.f32"]
    pub fn nearbyint_f32(a: f32) -> f32;
    /// The `llvm.nearbyint.v2f64` intrinsic.
    #[link_name = "llvm.nearbyint.v2f64"]
    pub fn nearbyint_v2f64(a: ::simdty::f64x2) -> ::simdty::f64x2;
    /// The `llvm.nearbyint.f64` intrinsic.
    #[link_name = "llvm.nearbyint.f64"]
    pub fn nearbyint_f64(a: f64) -> f64;
    /// The `llvm.round.v4f32` intrinsic.
    #[link_name = "llvm.round.v4f32"]
    pub fn round_v4f32(a: ::simdty::f32x4) -> ::simdty::f32x4;
    /// The `llvm.round.f32` intrinsic.
    #[link_name = "llvm.round.f32"]
    pub fn round_f32(a: f32) -> f32;
    /// The `llvm.round.v2f64` intrinsic.
    #[link_name = "llvm.round.v2f64"]
    pub fn round_v2f64(a: ::simdty::f64x2) -> ::simdty::f64x2;
    /// The `llvm.round.f64` intrinsic.
    #[link_name = "llvm.round.f64"]
    pub fn round_f64(a: f64) -> f64;
    /// The `llvm.setjmp` intrinsic.
    #[link_name = "llvm.setjmp"]
    pub fn setjmp(a: *mut i8) -> i32;
    /// The `llvm.longjmp` intrinsic.
    #[link_name = "llvm.longjmp"]
    pub fn longjmp(a: *mut i8, b: i32) -> ();
    /// The `llvm.sigsetjmp` intrinsic.
    #[link_name = "llvm.sigsetjmp"]
    pub fn sigsetjmp(a: *mut i8, b: i32) -> i32;
    /// The `llvm.siglongjmp` intrinsic.
    #[link_name = "llvm.siglongjmp"]
    pub fn siglongjmp(a: *mut i8, b: i32) -> ();
    /// The `llvm.objectsize.v16i8.p0i8` intrinsic; known as `__builtin_object_size` in GCC.
    #[link_name = "llvm.objectsize.v16i8.p0i8"]
    pub fn objectsize_v16i8_p0i8(a: *mut i8, b: bool) -> ::simdty::i8x16;
    /// The `llvm.objectsize.i8.p0i8` intrinsic; known as `__builtin_object_size` in GCC.
    #[link_name = "llvm.objectsize.i8.p0i8"]
    pub fn objectsize_i8_p0i8(a: *mut i8, b: bool) -> i8;
    /// The `llvm.objectsize.v8i16.p0i8` intrinsic; known as `__builtin_object_size` in GCC.
    #[link_name = "llvm.objectsize.v8i16.p0i8"]
    pub fn objectsize_v8i16_p0i8(a: *mut i8, b: bool) -> ::simdty::i16x8;
    /// The `llvm.objectsize.i16.p0i8` intrinsic; known as `__builtin_object_size` in GCC.
    #[link_name = "llvm.objectsize.i16.p0i8"]
    pub fn objectsize_i16_p0i8(a: *mut i8, b: bool) -> i16;
    /// The `llvm.objectsize.v4i32.p0i8` intrinsic; known as `__builtin_object_size` in GCC.
    #[link_name = "llvm.objectsize.v4i32.p0i8"]
    pub fn objectsize_v4i32_p0i8(a: *mut i8, b: bool) -> ::simdty::i32x4;
    /// The `llvm.objectsize.i32.p0i8` intrinsic; known as `__builtin_object_size` in GCC.
    #[link_name = "llvm.objectsize.i32.p0i8"]
    pub fn objectsize_i32_p0i8(a: *mut i8, b: bool) -> i32;
    /// The `llvm.objectsize.v2i64.p0i8` intrinsic; known as `__builtin_object_size` in GCC.
    #[link_name = "llvm.objectsize.v2i64.p0i8"]
    pub fn objectsize_v2i64_p0i8(a: *mut i8, b: bool) -> ::simdty::i64x2;
    /// The `llvm.objectsize.i64.p0i8` intrinsic; known as `__builtin_object_size` in GCC.
    #[link_name = "llvm.objectsize.i64.p0i8"]
    pub fn objectsize_i64_p0i8(a: *mut i8, b: bool) -> i64;
    /// The `llvm.expect.v16i8` intrinsic.
    #[link_name = "llvm.expect.v16i8"]
    pub fn expect_v16i8(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
    /// The `llvm.expect.i8` intrinsic.
    #[link_name = "llvm.expect.i8"]
    pub fn expect_i8(a: i8, b: i8) -> i8;
    /// The `llvm.expect.v8i16` intrinsic.
    #[link_name = "llvm.expect.v8i16"]
    pub fn expect_v8i16(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
    /// The `llvm.expect.i16` intrinsic.
    #[link_name = "llvm.expect.i16"]
    pub fn expect_i16(a: i16, b: i16) -> i16;
    /// The `llvm.expect.v4i32` intrinsic.
    #[link_name = "llvm.expect.v4i32"]
    pub fn expect_v4i32(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
    /// The `llvm.expect.i32` intrinsic.
    #[link_name = "llvm.expect.i32"]
    pub fn expect_i32(a: i32, b: i32) -> i32;
    /// The `llvm.expect.v2i64` intrinsic.
    #[link_name = "llvm.expect.v2i64"]
    pub fn expect_v2i64(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
    /// The `llvm.expect.i64` intrinsic.
    #[link_name = "llvm.expect.i64"]
    pub fn expect_i64(a: i64, b: i64) -> i64;
    /// The `llvm.bswap.v16i8` intrinsic.
    #[link_name = "llvm.bswap.v16i8"]
    pub fn bswap_v16i8(a: ::simdty::i8x16) -> ::simdty::i8x16;
    /// The `llvm.bswap.i8` intrinsic.
    #[link_name = "llvm.bswap.i8"]
    pub fn bswap_i8(a: i8) -> i8;
    /// The `llvm.bswap.v8i16` intrinsic.
    #[link_name = "llvm.bswap.v8i16"]
    pub fn bswap_v8i16(a: ::simdty::i16x8) -> ::simdty::i16x8;
    /// The `llvm.bswap.i16` intrinsic.
    #[link_name = "llvm.bswap.i16"]
    pub fn bswap_i16(a: i16) -> i16;
    /// The `llvm.bswap.v4i32` intrinsic.
    #[link_name = "llvm.bswap.v4i32"]
    pub fn bswap_v4i32(a: ::simdty::i32x4) -> ::simdty::i32x4;
    /// The `llvm.bswap.i32` intrinsic.
    #[link_name = "llvm.bswap.i32"]
    pub fn bswap_i32(a: i32) -> i32;
    /// The `llvm.bswap.v2i64` intrinsic.
    #[link_name = "llvm.bswap.v2i64"]
    pub fn bswap_v2i64(a: ::simdty::i64x2) -> ::simdty::i64x2;
    /// The `llvm.bswap.i64` intrinsic.
    #[link_name = "llvm.bswap.i64"]
    pub fn bswap_i64(a: i64) -> i64;
    /// The `llvm.ctpop.v16i8` intrinsic.
    #[link_name = "llvm.ctpop.v16i8"]
    pub fn ctpop_v16i8(a: ::simdty::i8x16) -> ::simdty::i8x16;
    /// The `llvm.ctpop.i8` intrinsic.
    #[link_name = "llvm.ctpop.i8"]
    pub fn ctpop_i8(a: i8) -> i8;
    /// The `llvm.ctpop.v8i16` intrinsic.
    #[link_name = "llvm.ctpop.v8i16"]
    pub fn ctpop_v8i16(a: ::simdty::i16x8) -> ::simdty::i16x8;
    /// The `llvm.ctpop.i16` intrinsic.
    #[link_name = "llvm.ctpop.i16"]
    pub fn ctpop_i16(a: i16) -> i16;
    /// The `llvm.ctpop.v4i32` intrinsic.
    #[link_name = "llvm.ctpop.v4i32"]
    pub fn ctpop_v4i32(a: ::simdty::i32x4) -> ::simdty::i32x4;
    /// The `llvm.ctpop.i32` intrinsic.
    #[link_name = "llvm.ctpop.i32"]
    pub fn ctpop_i32(a: i32) -> i32;
    /// The `llvm.ctpop.v2i64` intrinsic.
    #[link_name = "llvm.ctpop.v2i64"]
    pub fn ctpop_v2i64(a: ::simdty::i64x2) -> ::simdty::i64x2;
    /// The `llvm.ctpop.i64` intrinsic.
    #[link_name = "llvm.ctpop.i64"]
    pub fn ctpop_i64(a: i64) -> i64;
    /// The `llvm.ctlz.v16i8` intrinsic.
    #[link_name = "llvm.ctlz.v16i8"]
    pub fn ctlz_v16i8(a: ::simdty::i8x16, b: bool) -> ::simdty::i8x16;
    /// The `llvm.ctlz.i8` intrinsic.
    #[link_name = "llvm.ctlz.i8"]
    pub fn ctlz_i8(a: i8, b: bool) -> i8;
    /// The `llvm.ctlz.v8i16` intrinsic.
    #[link_name = "llvm.ctlz.v8i16"]
    pub fn ctlz_v8i16(a: ::simdty::i16x8, b: bool) -> ::simdty::i16x8;
    /// The `llvm.ctlz.i16` intrinsic.
    #[link_name = "llvm.ctlz.i16"]
    pub fn ctlz_i16(a: i16, b: bool) -> i16;
    /// The `llvm.ctlz.v4i32` intrinsic.
    #[link_name = "llvm.ctlz.v4i32"]
    pub fn ctlz_v4i32(a: ::simdty::i32x4, b: bool) -> ::simdty::i32x4;
    /// The `llvm.ctlz.i32` intrinsic.
    #[link_name = "llvm.ctlz.i32"]
    pub fn ctlz_i32(a: i32, b: bool) -> i32;
    /// The `llvm.ctlz.v2i64` intrinsic.
    #[link_name = "llvm.ctlz.v2i64"]
    pub fn ctlz_v2i64(a: ::simdty::i64x2, b: bool) -> ::simdty::i64x2;
    /// The `llvm.ctlz.i64` intrinsic.
    #[link_name = "llvm.ctlz.i64"]
    pub fn ctlz_i64(a: i64, b: bool) -> i64;
    /// The `llvm.cttz.v16i8` intrinsic.
    #[link_name = "llvm.cttz.v16i8"]
    pub fn cttz_v16i8(a: ::simdty::i8x16, b: bool) -> ::simdty::i8x16;
    /// The `llvm.cttz.i8` intrinsic.
    #[link_name = "llvm.cttz.i8"]
    pub fn cttz_i8(a: i8, b: bool) -> i8;
    /// The `llvm.cttz.v8i16` intrinsic.
    #[link_name = "llvm.cttz.v8i16"]
    pub fn cttz_v8i16(a: ::simdty::i16x8, b: bool) -> ::simdty::i16x8;
    /// The `llvm.cttz.i16` intrinsic.
    #[link_name = "llvm.cttz.i16"]
    pub fn cttz_i16(a: i16, b: bool) -> i16;
    /// The `llvm.cttz.v4i32` intrinsic.
    #[link_name = "llvm.cttz.v4i32"]
    pub fn cttz_v4i32(a: ::simdty::i32x4, b: bool) -> ::simdty::i32x4;
    /// The `llvm.cttz.i32` intrinsic.
    #[link_name = "llvm.cttz.i32"]
    pub fn cttz_i32(a: i32, b: bool) -> i32;
    /// The `llvm.cttz.v2i64` intrinsic.
    #[link_name = "llvm.cttz.v2i64"]
    pub fn cttz_v2i64(a: ::simdty::i64x2, b: bool) -> ::simdty::i64x2;
    /// The `llvm.cttz.i64` intrinsic.
    #[link_name = "llvm.cttz.i64"]
    pub fn cttz_i64(a: i64, b: bool) -> i64;
    /// The `llvm.eh.typeid.for` intrinsic.
    #[link_name = "llvm.eh.typeid.for"]
    pub fn eh_typeid_for(a: *mut i8) -> i32;
    /// The `llvm.eh.return.i32` intrinsic.
    #[link_name = "llvm.eh.return.i32"]
    pub fn eh_return_i32(a: i32, b: *mut i8) -> ();
    /// The `llvm.eh.return.i64` intrinsic.
    #[link_name = "llvm.eh.return.i64"]
    pub fn eh_return_i64(a: i64, b: *mut i8) -> ();
    /// The `llvm.eh.unwind.init` intrinsic; known as `__builtin_unwind_init` in GCC.
    #[link_name = "llvm.eh.unwind.init"]
    pub fn eh_unwind_init() -> ();
    /// The `llvm.eh.dwarf.cfa` intrinsic.
    #[link_name = "llvm.eh.dwarf.cfa"]
    pub fn eh_dwarf_cfa(a: i32) -> *mut i8;
    /// The `llvm.eh.sjlj.lsda` intrinsic.
    #[link_name = "llvm.eh.sjlj.lsda"]
    pub fn eh_sjlj_lsda() -> *mut i8;
    /// The `llvm.eh.sjlj.callsite` intrinsic.
    #[link_name = "llvm.eh.sjlj.callsite"]
    pub fn eh_sjlj_callsite(a: i32) -> ();
    /// The `llvm.eh.sjlj.functioncontext` intrinsic.
    #[link_name = "llvm.eh.sjlj.functioncontext"]
    pub fn eh_sjlj_functioncontext(a: *mut i8) -> ();
    /// The `llvm.eh.sjlj.setjmp` intrinsic.
    #[link_name = "llvm.eh.sjlj.setjmp"]
    pub fn eh_sjlj_setjmp(a: *mut i8) -> i32;
    /// The `llvm.eh.sjlj.longjmp` intrinsic.
    #[link_name = "llvm.eh.sjlj.longjmp"]
    pub fn eh_sjlj_longjmp(a: *mut i8) -> ();
    /// The `llvm.var.annotation` intrinsic.
    #[link_name = "llvm.var.annotation"]
    pub fn var_annotation(a: *mut i8, b: *mut i8, c: *mut i8, d: i32) -> ();
    /// The `llvm.ptr.annotation.p0v16i8` intrinsic.
    #[link_name = "llvm.ptr.annotation.p0v16i8"]
    pub fn ptr_annotation_p0v16i8(a: *mut ::simdty::i8x16, b: *mut i8, c: *mut i8, d: i32) -> *mut ::simdty::i8x16;
    /// The `llvm.ptr.annotation.p0i8` intrinsic.
    #[link_name = "llvm.ptr.annotation.p0i8"]
    pub fn ptr_annotation_p0i8(a: *mut i8, b: *mut i8, c: *mut i8, d: i32) -> *mut i8;
    /// The `llvm.ptr.annotation.p0v8i16` intrinsic.
    #[link_name = "llvm.ptr.annotation.p0v8i16"]
    pub fn ptr_annotation_p0v8i16(a: *mut ::simdty::i16x8, b: *mut i8, c: *mut i8, d: i32) -> *mut ::simdty::i16x8;
    /// The `llvm.ptr.annotation.p0i16` intrinsic.
    #[link_name = "llvm.ptr.annotation.p0i16"]
    pub fn ptr_annotation_p0i16(a: *mut i16, b: *mut i8, c: *mut i8, d: i32) -> *mut i16;
    /// The `llvm.ptr.annotation.p0v4i32` intrinsic.
    #[link_name = "llvm.ptr.annotation.p0v4i32"]
    pub fn ptr_annotation_p0v4i32(a: *mut ::simdty::i32x4, b: *mut i8, c: *mut i8, d: i32) -> *mut ::simdty::i32x4;
    /// The `llvm.ptr.annotation.p0i32` intrinsic.
    #[link_name = "llvm.ptr.annotation.p0i32"]
    pub fn ptr_annotation_p0i32(a: *mut i32, b: *mut i8, c: *mut i8, d: i32) -> *mut i32;
    /// The `llvm.ptr.annotation.p0v2i64` intrinsic.
    #[link_name = "llvm.ptr.annotation.p0v2i64"]
    pub fn ptr_annotation_p0v2i64(a: *mut ::simdty::i64x2, b: *mut i8, c: *mut i8, d: i32) -> *mut ::simdty::i64x2;
    /// The `llvm.ptr.annotation.p0i64` intrinsic.
    #[link_name = "llvm.ptr.annotation.p0i64"]
    pub fn ptr_annotation_p0i64(a: *mut i64, b: *mut i8, c: *mut i8, d: i32) -> *mut i64;
    /// The `llvm.annotation.v16i8` intrinsic.
    #[link_name = "llvm.annotation.v16i8"]
    pub fn annotation_v16i8(a: ::simdty::i8x16, b: *mut i8, c: *mut i8, d: i32) -> ::simdty::i8x16;
    /// The `llvm.annotation.i8` intrinsic.
    #[link_name = "llvm.annotation.i8"]
    pub fn annotation_i8(a: i8, b: *mut i8, c: *mut i8, d: i32) -> i8;
    /// The `llvm.annotation.v8i16` intrinsic.
    #[link_name = "llvm.annotation.v8i16"]
    pub fn annotation_v8i16(a: ::simdty::i16x8, b: *mut i8, c: *mut i8, d: i32) -> ::simdty::i16x8;
    /// The `llvm.annotation.i16` intrinsic.
    #[link_name = "llvm.annotation.i16"]
    pub fn annotation_i16(a: i16, b: *mut i8, c: *mut i8, d: i32) -> i16;
    /// The `llvm.annotation.v4i32` intrinsic.
    #[link_name = "llvm.annotation.v4i32"]
    pub fn annotation_v4i32(a: ::simdty::i32x4, b: *mut i8, c: *mut i8, d: i32) -> ::simdty::i32x4;
    /// The `llvm.annotation.i32` intrinsic.
    #[link_name = "llvm.annotation.i32"]
    pub fn annotation_i32(a: i32, b: *mut i8, c: *mut i8, d: i32) -> i32;
    /// The `llvm.annotation.v2i64` intrinsic.
    #[link_name = "llvm.annotation.v2i64"]
    pub fn annotation_v2i64(a: ::simdty::i64x2, b: *mut i8, c: *mut i8, d: i32) -> ::simdty::i64x2;
    /// The `llvm.annotation.i64` intrinsic.
    #[link_name = "llvm.annotation.i64"]
    pub fn annotation_i64(a: i64, b: *mut i8, c: *mut i8, d: i32) -> i64;
    /// The `llvm.init.trampoline` intrinsic; known as `__builtin_init_trampoline` in GCC.
    #[link_name = "llvm.init.trampoline"]
    pub fn init_trampoline(a: *mut i8, b: *mut i8, c: *mut i8) -> ();
    /// The `llvm.adjust.trampoline` intrinsic; known as `__builtin_adjust_trampoline` in GCC.
    #[link_name = "llvm.adjust.trampoline"]
    pub fn adjust_trampoline(a: *mut i8) -> *mut i8;
    /// The `llvm.lifetime.start` intrinsic.
    #[link_name = "llvm.lifetime.start"]
    pub fn lifetime_start(a: i64, b: *mut i8) -> ();
    /// The `llvm.lifetime.end` intrinsic.
    #[link_name = "llvm.lifetime.end"]
    pub fn lifetime_end(a: i64, b: *mut i8) -> ();
    /// The `llvm.experimental.stackmap` intrinsic.
    #[link_name = "llvm.experimental.stackmap"]
    pub fn experimental_stackmap(a: i64, b: i32, ...) -> ();
    /// The `llvm.experimental.patchpoint.void` intrinsic.
    #[link_name = "llvm.experimental.patchpoint.void"]
    pub fn experimental_patchpoint_void(a: i64, b: i32, c: *mut i8, d: i32, ...) -> ();
    /// The `llvm.experimental.patchpoint.i64` intrinsic.
    #[link_name = "llvm.experimental.patchpoint.i64"]
    pub fn experimental_patchpoint_i64(a: i64, b: i32, c: *mut i8, d: i32, ...) -> i64;
    /// The `llvm.flt.rounds` intrinsic; known as `__builtin_flt_rounds` in GCC.
    #[link_name = "llvm.flt.rounds"]
    pub fn flt_rounds() -> i32;
    /// The `llvm.trap` intrinsic; known as `__builtin_trap` in GCC.
    #[link_name = "llvm.trap"]
    pub fn trap() -> ();
    /// The `llvm.debugtrap` intrinsic; known as `__builtin_debugtrap` in GCC.
    #[link_name = "llvm.debugtrap"]
    pub fn debugtrap() -> ();
    /// The `llvm.donothing` intrinsic.
    #[link_name = "llvm.donothing"]
    pub fn donothing() -> ();
    /// The `llvm.convert.to.fp16.v4f32` intrinsic.
    #[link_name = "llvm.convert.to.fp16.v4f32"]
    pub fn convert_to_fp16_v4f32(a: ::simdty::f32x4) -> i16;
    /// The `llvm.convert.to.fp16.f32` intrinsic.
    #[link_name = "llvm.convert.to.fp16.f32"]
    pub fn convert_to_fp16_f32(a: f32) -> i16;
    /// The `llvm.convert.to.fp16.v2f64` intrinsic.
    #[link_name = "llvm.convert.to.fp16.v2f64"]
    pub fn convert_to_fp16_v2f64(a: ::simdty::f64x2) -> i16;
    /// The `llvm.convert.to.fp16.f64` intrinsic.
    #[link_name = "llvm.convert.to.fp16.f64"]
    pub fn convert_to_fp16_f64(a: f64) -> i16;
    /// The `llvm.convert.from.fp16.v4f32` intrinsic.
    #[link_name = "llvm.convert.from.fp16.v4f32"]
    pub fn convert_from_fp16_v4f32(a: i16) -> ::simdty::f32x4;
    /// The `llvm.convert.from.fp16.f32` intrinsic.
    #[link_name = "llvm.convert.from.fp16.f32"]
    pub fn convert_from_fp16_f32(a: i16) -> f32;
    /// The `llvm.convert.from.fp16.v2f64` intrinsic.
    #[link_name = "llvm.convert.from.fp16.v2f64"]
    pub fn convert_from_fp16_v2f64(a: i16) -> ::simdty::f64x2;
    /// The `llvm.convert.from.fp16.f64` intrinsic.
    #[link_name = "llvm.convert.from.fp16.f64"]
    pub fn convert_from_fp16_f64(a: i16) -> f64;
    /// The `llvm.convertff.v4f32.v4f32` intrinsic.
    #[link_name = "llvm.convertff.v4f32.v4f32"]
    pub fn convertff_v4f32_v4f32(a: ::simdty::f32x4, b: i32, c: i32) -> ::simdty::f32x4;
    /// The `llvm.convertff.v4f32.f32` intrinsic.
    #[link_name = "llvm.convertff.v4f32.f32"]
    pub fn convertff_v4f32_f32(a: f32, b: i32, c: i32) -> ::simdty::f32x4;
    /// The `llvm.convertff.v4f32.v2f64` intrinsic.
    #[link_name = "llvm.convertff.v4f32.v2f64"]
    pub fn convertff_v4f32_v2f64(a: ::simdty::f64x2, b: i32, c: i32) -> ::simdty::f32x4;
    /// The `llvm.convertff.v4f32.f64` intrinsic.
    #[link_name = "llvm.convertff.v4f32.f64"]
    pub fn convertff_v4f32_f64(a: f64, b: i32, c: i32) -> ::simdty::f32x4;
    /// The `llvm.convertff.f32.v4f32` intrinsic.
    #[link_name = "llvm.convertff.f32.v4f32"]
    pub fn convertff_f32_v4f32(a: ::simdty::f32x4, b: i32, c: i32) -> f32;
    /// The `llvm.convertff.f32.f32` intrinsic.
    #[link_name = "llvm.convertff.f32.f32"]
    pub fn convertff_f32_f32(a: f32, b: i32, c: i32) -> f32;
    /// The `llvm.convertff.f32.v2f64` intrinsic.
    #[link_name = "llvm.convertff.f32.v2f64"]
    pub fn convertff_f32_v2f64(a: ::simdty::f64x2, b: i32, c: i32) -> f32;
    /// The `llvm.convertff.f32.f64` intrinsic.
    #[link_name = "llvm.convertff.f32.f64"]
    pub fn convertff_f32_f64(a: f64, b: i32, c: i32) -> f32;
    /// The `llvm.convertff.v2f64.v4f32` intrinsic.
    #[link_name = "llvm.convertff.v2f64.v4f32"]
    pub fn convertff_v2f64_v4f32(a: ::simdty::f32x4, b: i32, c: i32) -> ::simdty::f64x2;
    /// The `llvm.convertff.v2f64.f32` intrinsic.
    #[link_name = "llvm.convertff.v2f64.f32"]
    pub fn convertff_v2f64_f32(a: f32, b: i32, c: i32) -> ::simdty::f64x2;
    /// The `llvm.convertff.v2f64.v2f64` intrinsic.
    #[link_name = "llvm.convertff.v2f64.v2f64"]
    pub fn convertff_v2f64_v2f64(a: ::simdty::f64x2, b: i32, c: i32) -> ::simdty::f64x2;
    /// The `llvm.convertff.v2f64.f64` intrinsic.
    #[link_name = "llvm.convertff.v2f64.f64"]
    pub fn convertff_v2f64_f64(a: f64, b: i32, c: i32) -> ::simdty::f64x2;
    /// The `llvm.convertff.f64.v4f32` intrinsic.
    #[link_name = "llvm.convertff.f64.v4f32"]
    pub fn convertff_f64_v4f32(a: ::simdty::f32x4, b: i32, c: i32) -> f64;
    /// The `llvm.convertff.f64.f32` intrinsic.
    #[link_name = "llvm.convertff.f64.f32"]
    pub fn convertff_f64_f32(a: f32, b: i32, c: i32) -> f64;
    /// The `llvm.convertff.f64.v2f64` intrinsic.
    #[link_name = "llvm.convertff.f64.v2f64"]
    pub fn convertff_f64_v2f64(a: ::simdty::f64x2, b: i32, c: i32) -> f64;
    /// The `llvm.convertff.f64.f64` intrinsic.
    #[link_name = "llvm.convertff.f64.f64"]
    pub fn convertff_f64_f64(a: f64, b: i32, c: i32) -> f64;
    /// The `llvm.convertfsi.v4f32.v16i8` intrinsic.
    #[link_name = "llvm.convertfsi.v4f32.v16i8"]
    pub fn convertfsi_v4f32_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> ::simdty::f32x4;
    /// The `llvm.convertfsi.v4f32.i8` intrinsic.
    #[link_name = "llvm.convertfsi.v4f32.i8"]
    pub fn convertfsi_v4f32_i8(a: i8, b: i32, c: i32) -> ::simdty::f32x4;
    /// The `llvm.convertfsi.v4f32.v8i16` intrinsic.
    #[link_name = "llvm.convertfsi.v4f32.v8i16"]
    pub fn convertfsi_v4f32_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> ::simdty::f32x4;
    /// The `llvm.convertfsi.v4f32.i16` intrinsic.
    #[link_name = "llvm.convertfsi.v4f32.i16"]
    pub fn convertfsi_v4f32_i16(a: i16, b: i32, c: i32) -> ::simdty::f32x4;
    /// The `llvm.convertfsi.v4f32.v4i32` intrinsic.
    #[link_name = "llvm.convertfsi.v4f32.v4i32"]
    pub fn convertfsi_v4f32_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> ::simdty::f32x4;
    /// The `llvm.convertfsi.v4f32.i32` intrinsic.
    #[link_name = "llvm.convertfsi.v4f32.i32"]
    pub fn convertfsi_v4f32_i32(a: i32, b: i32, c: i32) -> ::simdty::f32x4;
    /// The `llvm.convertfsi.v4f32.v2i64` intrinsic.
    #[link_name = "llvm.convertfsi.v4f32.v2i64"]
    pub fn convertfsi_v4f32_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> ::simdty::f32x4;
    /// The `llvm.convertfsi.v4f32.i64` intrinsic.
    #[link_name = "llvm.convertfsi.v4f32.i64"]
    pub fn convertfsi_v4f32_i64(a: i64, b: i32, c: i32) -> ::simdty::f32x4;
    /// The `llvm.convertfsi.f32.v16i8` intrinsic.
    #[link_name = "llvm.convertfsi.f32.v16i8"]
    pub fn convertfsi_f32_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> f32;
    /// The `llvm.convertfsi.f32.i8` intrinsic.
    #[link_name = "llvm.convertfsi.f32.i8"]
    pub fn convertfsi_f32_i8(a: i8, b: i32, c: i32) -> f32;
    /// The `llvm.convertfsi.f32.v8i16` intrinsic.
    #[link_name = "llvm.convertfsi.f32.v8i16"]
    pub fn convertfsi_f32_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> f32;
    /// The `llvm.convertfsi.f32.i16` intrinsic.
    #[link_name = "llvm.convertfsi.f32.i16"]
    pub fn convertfsi_f32_i16(a: i16, b: i32, c: i32) -> f32;
    /// The `llvm.convertfsi.f32.v4i32` intrinsic.
    #[link_name = "llvm.convertfsi.f32.v4i32"]
    pub fn convertfsi_f32_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> f32;
    /// The `llvm.convertfsi.f32.i32` intrinsic.
    #[link_name = "llvm.convertfsi.f32.i32"]
    pub fn convertfsi_f32_i32(a: i32, b: i32, c: i32) -> f32;
    /// The `llvm.convertfsi.f32.v2i64` intrinsic.
    #[link_name = "llvm.convertfsi.f32.v2i64"]
    pub fn convertfsi_f32_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> f32;
    /// The `llvm.convertfsi.f32.i64` intrinsic.
    #[link_name = "llvm.convertfsi.f32.i64"]
    pub fn convertfsi_f32_i64(a: i64, b: i32, c: i32) -> f32;
    /// The `llvm.convertfsi.v2f64.v16i8` intrinsic.
    #[link_name = "llvm.convertfsi.v2f64.v16i8"]
    pub fn convertfsi_v2f64_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> ::simdty::f64x2;
    /// The `llvm.convertfsi.v2f64.i8` intrinsic.
    #[link_name = "llvm.convertfsi.v2f64.i8"]
    pub fn convertfsi_v2f64_i8(a: i8, b: i32, c: i32) -> ::simdty::f64x2;
    /// The `llvm.convertfsi.v2f64.v8i16` intrinsic.
    #[link_name = "llvm.convertfsi.v2f64.v8i16"]
    pub fn convertfsi_v2f64_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> ::simdty::f64x2;
    /// The `llvm.convertfsi.v2f64.i16` intrinsic.
    #[link_name = "llvm.convertfsi.v2f64.i16"]
    pub fn convertfsi_v2f64_i16(a: i16, b: i32, c: i32) -> ::simdty::f64x2;
    /// The `llvm.convertfsi.v2f64.v4i32` intrinsic.
    #[link_name = "llvm.convertfsi.v2f64.v4i32"]
    pub fn convertfsi_v2f64_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> ::simdty::f64x2;
    /// The `llvm.convertfsi.v2f64.i32` intrinsic.
    #[link_name = "llvm.convertfsi.v2f64.i32"]
    pub fn convertfsi_v2f64_i32(a: i32, b: i32, c: i32) -> ::simdty::f64x2;
    /// The `llvm.convertfsi.v2f64.v2i64` intrinsic.
    #[link_name = "llvm.convertfsi.v2f64.v2i64"]
    pub fn convertfsi_v2f64_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> ::simdty::f64x2;
    /// The `llvm.convertfsi.v2f64.i64` intrinsic.
    #[link_name = "llvm.convertfsi.v2f64.i64"]
    pub fn convertfsi_v2f64_i64(a: i64, b: i32, c: i32) -> ::simdty::f64x2;
    /// The `llvm.convertfsi.f64.v16i8` intrinsic.
    #[link_name = "llvm.convertfsi.f64.v16i8"]
    pub fn convertfsi_f64_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> f64;
    /// The `llvm.convertfsi.f64.i8` intrinsic.
    #[link_name = "llvm.convertfsi.f64.i8"]
    pub fn convertfsi_f64_i8(a: i8, b: i32, c: i32) -> f64;
    /// The `llvm.convertfsi.f64.v8i16` intrinsic.
    #[link_name = "llvm.convertfsi.f64.v8i16"]
    pub fn convertfsi_f64_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> f64;
    /// The `llvm.convertfsi.f64.i16` intrinsic.
    #[link_name = "llvm.convertfsi.f64.i16"]
    pub fn convertfsi_f64_i16(a: i16, b: i32, c: i32) -> f64;
    /// The `llvm.convertfsi.f64.v4i32` intrinsic.
    #[link_name = "llvm.convertfsi.f64.v4i32"]
    pub fn convertfsi_f64_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> f64;
    /// The `llvm.convertfsi.f64.i32` intrinsic.
    #[link_name = "llvm.convertfsi.f64.i32"]
    pub fn convertfsi_f64_i32(a: i32, b: i32, c: i32) -> f64;
    /// The `llvm.convertfsi.f64.v2i64` intrinsic.
    #[link_name = "llvm.convertfsi.f64.v2i64"]
    pub fn convertfsi_f64_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> f64;
    /// The `llvm.convertfsi.f64.i64` intrinsic.
    #[link_name = "llvm.convertfsi.f64.i64"]
    pub fn convertfsi_f64_i64(a: i64, b: i32, c: i32) -> f64;
    /// The `llvm.convertfui.v4f32.v16i8` intrinsic.
    #[link_name = "llvm.convertfui.v4f32.v16i8"]
    pub fn convertfui_v4f32_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> ::simdty::f32x4;
    /// The `llvm.convertfui.v4f32.i8` intrinsic.
    #[link_name = "llvm.convertfui.v4f32.i8"]
    pub fn convertfui_v4f32_i8(a: i8, b: i32, c: i32) -> ::simdty::f32x4;
    /// The `llvm.convertfui.v4f32.v8i16` intrinsic.
    #[link_name = "llvm.convertfui.v4f32.v8i16"]
    pub fn convertfui_v4f32_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> ::simdty::f32x4;
    /// The `llvm.convertfui.v4f32.i16` intrinsic.
    #[link_name = "llvm.convertfui.v4f32.i16"]
    pub fn convertfui_v4f32_i16(a: i16, b: i32, c: i32) -> ::simdty::f32x4;
    /// The `llvm.convertfui.v4f32.v4i32` intrinsic.
    #[link_name = "llvm.convertfui.v4f32.v4i32"]
    pub fn convertfui_v4f32_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> ::simdty::f32x4;
    /// The `llvm.convertfui.v4f32.i32` intrinsic.
    #[link_name = "llvm.convertfui.v4f32.i32"]
    pub fn convertfui_v4f32_i32(a: i32, b: i32, c: i32) -> ::simdty::f32x4;
    /// The `llvm.convertfui.v4f32.v2i64` intrinsic.
    #[link_name = "llvm.convertfui.v4f32.v2i64"]
    pub fn convertfui_v4f32_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> ::simdty::f32x4;
    /// The `llvm.convertfui.v4f32.i64` intrinsic.
    #[link_name = "llvm.convertfui.v4f32.i64"]
    pub fn convertfui_v4f32_i64(a: i64, b: i32, c: i32) -> ::simdty::f32x4;
    /// The `llvm.convertfui.f32.v16i8` intrinsic.
    #[link_name = "llvm.convertfui.f32.v16i8"]
    pub fn convertfui_f32_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> f32;
    /// The `llvm.convertfui.f32.i8` intrinsic.
    #[link_name = "llvm.convertfui.f32.i8"]
    pub fn convertfui_f32_i8(a: i8, b: i32, c: i32) -> f32;
    /// The `llvm.convertfui.f32.v8i16` intrinsic.
    #[link_name = "llvm.convertfui.f32.v8i16"]
    pub fn convertfui_f32_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> f32;
    /// The `llvm.convertfui.f32.i16` intrinsic.
    #[link_name = "llvm.convertfui.f32.i16"]
    pub fn convertfui_f32_i16(a: i16, b: i32, c: i32) -> f32;
    /// The `llvm.convertfui.f32.v4i32` intrinsic.
    #[link_name = "llvm.convertfui.f32.v4i32"]
    pub fn convertfui_f32_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> f32;
    /// The `llvm.convertfui.f32.i32` intrinsic.
    #[link_name = "llvm.convertfui.f32.i32"]
    pub fn convertfui_f32_i32(a: i32, b: i32, c: i32) -> f32;
    /// The `llvm.convertfui.f32.v2i64` intrinsic.
    #[link_name = "llvm.convertfui.f32.v2i64"]
    pub fn convertfui_f32_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> f32;
    /// The `llvm.convertfui.f32.i64` intrinsic.
    #[link_name = "llvm.convertfui.f32.i64"]
    pub fn convertfui_f32_i64(a: i64, b: i32, c: i32) -> f32;
    /// The `llvm.convertfui.v2f64.v16i8` intrinsic.
    #[link_name = "llvm.convertfui.v2f64.v16i8"]
    pub fn convertfui_v2f64_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> ::simdty::f64x2;
    /// The `llvm.convertfui.v2f64.i8` intrinsic.
    #[link_name = "llvm.convertfui.v2f64.i8"]
    pub fn convertfui_v2f64_i8(a: i8, b: i32, c: i32) -> ::simdty::f64x2;
    /// The `llvm.convertfui.v2f64.v8i16` intrinsic.
    #[link_name = "llvm.convertfui.v2f64.v8i16"]
    pub fn convertfui_v2f64_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> ::simdty::f64x2;
    /// The `llvm.convertfui.v2f64.i16` intrinsic.
    #[link_name = "llvm.convertfui.v2f64.i16"]
    pub fn convertfui_v2f64_i16(a: i16, b: i32, c: i32) -> ::simdty::f64x2;
    /// The `llvm.convertfui.v2f64.v4i32` intrinsic.
    #[link_name = "llvm.convertfui.v2f64.v4i32"]
    pub fn convertfui_v2f64_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> ::simdty::f64x2;
    /// The `llvm.convertfui.v2f64.i32` intrinsic.
    #[link_name = "llvm.convertfui.v2f64.i32"]
    pub fn convertfui_v2f64_i32(a: i32, b: i32, c: i32) -> ::simdty::f64x2;
    /// The `llvm.convertfui.v2f64.v2i64` intrinsic.
    #[link_name = "llvm.convertfui.v2f64.v2i64"]
    pub fn convertfui_v2f64_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> ::simdty::f64x2;
    /// The `llvm.convertfui.v2f64.i64` intrinsic.
    #[link_name = "llvm.convertfui.v2f64.i64"]
    pub fn convertfui_v2f64_i64(a: i64, b: i32, c: i32) -> ::simdty::f64x2;
    /// The `llvm.convertfui.f64.v16i8` intrinsic.
    #[link_name = "llvm.convertfui.f64.v16i8"]
    pub fn convertfui_f64_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> f64;
    /// The `llvm.convertfui.f64.i8` intrinsic.
    #[link_name = "llvm.convertfui.f64.i8"]
    pub fn convertfui_f64_i8(a: i8, b: i32, c: i32) -> f64;
    /// The `llvm.convertfui.f64.v8i16` intrinsic.
    #[link_name = "llvm.convertfui.f64.v8i16"]
    pub fn convertfui_f64_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> f64;
    /// The `llvm.convertfui.f64.i16` intrinsic.
    #[link_name = "llvm.convertfui.f64.i16"]
    pub fn convertfui_f64_i16(a: i16, b: i32, c: i32) -> f64;
    /// The `llvm.convertfui.f64.v4i32` intrinsic.
    #[link_name = "llvm.convertfui.f64.v4i32"]
    pub fn convertfui_f64_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> f64;
    /// The `llvm.convertfui.f64.i32` intrinsic.
    #[link_name = "llvm.convertfui.f64.i32"]
    pub fn convertfui_f64_i32(a: i32, b: i32, c: i32) -> f64;
    /// The `llvm.convertfui.f64.v2i64` intrinsic.
    #[link_name = "llvm.convertfui.f64.v2i64"]
    pub fn convertfui_f64_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> f64;
    /// The `llvm.convertfui.f64.i64` intrinsic.
    #[link_name = "llvm.convertfui.f64.i64"]
    pub fn convertfui_f64_i64(a: i64, b: i32, c: i32) -> f64;
    /// The `llvm.convertsif.v16i8.v4f32` intrinsic.
    #[link_name = "llvm.convertsif.v16i8.v4f32"]
    pub fn convertsif_v16i8_v4f32(a: ::simdty::f32x4, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertsif.v16i8.f32` intrinsic.
    #[link_name = "llvm.convertsif.v16i8.f32"]
    pub fn convertsif_v16i8_f32(a: f32, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertsif.v16i8.v2f64` intrinsic.
    #[link_name = "llvm.convertsif.v16i8.v2f64"]
    pub fn convertsif_v16i8_v2f64(a: ::simdty::f64x2, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertsif.v16i8.f64` intrinsic.
    #[link_name = "llvm.convertsif.v16i8.f64"]
    pub fn convertsif_v16i8_f64(a: f64, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertsif.i8.v4f32` intrinsic.
    #[link_name = "llvm.convertsif.i8.v4f32"]
    pub fn convertsif_i8_v4f32(a: ::simdty::f32x4, b: i32, c: i32) -> i8;
    /// The `llvm.convertsif.i8.f32` intrinsic.
    #[link_name = "llvm.convertsif.i8.f32"]
    pub fn convertsif_i8_f32(a: f32, b: i32, c: i32) -> i8;
    /// The `llvm.convertsif.i8.v2f64` intrinsic.
    #[link_name = "llvm.convertsif.i8.v2f64"]
    pub fn convertsif_i8_v2f64(a: ::simdty::f64x2, b: i32, c: i32) -> i8;
    /// The `llvm.convertsif.i8.f64` intrinsic.
    #[link_name = "llvm.convertsif.i8.f64"]
    pub fn convertsif_i8_f64(a: f64, b: i32, c: i32) -> i8;
    /// The `llvm.convertsif.v8i16.v4f32` intrinsic.
    #[link_name = "llvm.convertsif.v8i16.v4f32"]
    pub fn convertsif_v8i16_v4f32(a: ::simdty::f32x4, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertsif.v8i16.f32` intrinsic.
    #[link_name = "llvm.convertsif.v8i16.f32"]
    pub fn convertsif_v8i16_f32(a: f32, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertsif.v8i16.v2f64` intrinsic.
    #[link_name = "llvm.convertsif.v8i16.v2f64"]
    pub fn convertsif_v8i16_v2f64(a: ::simdty::f64x2, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertsif.v8i16.f64` intrinsic.
    #[link_name = "llvm.convertsif.v8i16.f64"]
    pub fn convertsif_v8i16_f64(a: f64, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertsif.i16.v4f32` intrinsic.
    #[link_name = "llvm.convertsif.i16.v4f32"]
    pub fn convertsif_i16_v4f32(a: ::simdty::f32x4, b: i32, c: i32) -> i16;
    /// The `llvm.convertsif.i16.f32` intrinsic.
    #[link_name = "llvm.convertsif.i16.f32"]
    pub fn convertsif_i16_f32(a: f32, b: i32, c: i32) -> i16;
    /// The `llvm.convertsif.i16.v2f64` intrinsic.
    #[link_name = "llvm.convertsif.i16.v2f64"]
    pub fn convertsif_i16_v2f64(a: ::simdty::f64x2, b: i32, c: i32) -> i16;
    /// The `llvm.convertsif.i16.f64` intrinsic.
    #[link_name = "llvm.convertsif.i16.f64"]
    pub fn convertsif_i16_f64(a: f64, b: i32, c: i32) -> i16;
    /// The `llvm.convertsif.v4i32.v4f32` intrinsic.
    #[link_name = "llvm.convertsif.v4i32.v4f32"]
    pub fn convertsif_v4i32_v4f32(a: ::simdty::f32x4, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertsif.v4i32.f32` intrinsic.
    #[link_name = "llvm.convertsif.v4i32.f32"]
    pub fn convertsif_v4i32_f32(a: f32, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertsif.v4i32.v2f64` intrinsic.
    #[link_name = "llvm.convertsif.v4i32.v2f64"]
    pub fn convertsif_v4i32_v2f64(a: ::simdty::f64x2, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertsif.v4i32.f64` intrinsic.
    #[link_name = "llvm.convertsif.v4i32.f64"]
    pub fn convertsif_v4i32_f64(a: f64, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertsif.i32.v4f32` intrinsic.
    #[link_name = "llvm.convertsif.i32.v4f32"]
    pub fn convertsif_i32_v4f32(a: ::simdty::f32x4, b: i32, c: i32) -> i32;
    /// The `llvm.convertsif.i32.f32` intrinsic.
    #[link_name = "llvm.convertsif.i32.f32"]
    pub fn convertsif_i32_f32(a: f32, b: i32, c: i32) -> i32;
    /// The `llvm.convertsif.i32.v2f64` intrinsic.
    #[link_name = "llvm.convertsif.i32.v2f64"]
    pub fn convertsif_i32_v2f64(a: ::simdty::f64x2, b: i32, c: i32) -> i32;
    /// The `llvm.convertsif.i32.f64` intrinsic.
    #[link_name = "llvm.convertsif.i32.f64"]
    pub fn convertsif_i32_f64(a: f64, b: i32, c: i32) -> i32;
    /// The `llvm.convertsif.v2i64.v4f32` intrinsic.
    #[link_name = "llvm.convertsif.v2i64.v4f32"]
    pub fn convertsif_v2i64_v4f32(a: ::simdty::f32x4, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertsif.v2i64.f32` intrinsic.
    #[link_name = "llvm.convertsif.v2i64.f32"]
    pub fn convertsif_v2i64_f32(a: f32, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertsif.v2i64.v2f64` intrinsic.
    #[link_name = "llvm.convertsif.v2i64.v2f64"]
    pub fn convertsif_v2i64_v2f64(a: ::simdty::f64x2, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertsif.v2i64.f64` intrinsic.
    #[link_name = "llvm.convertsif.v2i64.f64"]
    pub fn convertsif_v2i64_f64(a: f64, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertsif.i64.v4f32` intrinsic.
    #[link_name = "llvm.convertsif.i64.v4f32"]
    pub fn convertsif_i64_v4f32(a: ::simdty::f32x4, b: i32, c: i32) -> i64;
    /// The `llvm.convertsif.i64.f32` intrinsic.
    #[link_name = "llvm.convertsif.i64.f32"]
    pub fn convertsif_i64_f32(a: f32, b: i32, c: i32) -> i64;
    /// The `llvm.convertsif.i64.v2f64` intrinsic.
    #[link_name = "llvm.convertsif.i64.v2f64"]
    pub fn convertsif_i64_v2f64(a: ::simdty::f64x2, b: i32, c: i32) -> i64;
    /// The `llvm.convertsif.i64.f64` intrinsic.
    #[link_name = "llvm.convertsif.i64.f64"]
    pub fn convertsif_i64_f64(a: f64, b: i32, c: i32) -> i64;
    /// The `llvm.convertuif.v16i8.v4f32` intrinsic.
    #[link_name = "llvm.convertuif.v16i8.v4f32"]
    pub fn convertuif_v16i8_v4f32(a: ::simdty::f32x4, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertuif.v16i8.f32` intrinsic.
    #[link_name = "llvm.convertuif.v16i8.f32"]
    pub fn convertuif_v16i8_f32(a: f32, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertuif.v16i8.v2f64` intrinsic.
    #[link_name = "llvm.convertuif.v16i8.v2f64"]
    pub fn convertuif_v16i8_v2f64(a: ::simdty::f64x2, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertuif.v16i8.f64` intrinsic.
    #[link_name = "llvm.convertuif.v16i8.f64"]
    pub fn convertuif_v16i8_f64(a: f64, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertuif.i8.v4f32` intrinsic.
    #[link_name = "llvm.convertuif.i8.v4f32"]
    pub fn convertuif_i8_v4f32(a: ::simdty::f32x4, b: i32, c: i32) -> i8;
    /// The `llvm.convertuif.i8.f32` intrinsic.
    #[link_name = "llvm.convertuif.i8.f32"]
    pub fn convertuif_i8_f32(a: f32, b: i32, c: i32) -> i8;
    /// The `llvm.convertuif.i8.v2f64` intrinsic.
    #[link_name = "llvm.convertuif.i8.v2f64"]
    pub fn convertuif_i8_v2f64(a: ::simdty::f64x2, b: i32, c: i32) -> i8;
    /// The `llvm.convertuif.i8.f64` intrinsic.
    #[link_name = "llvm.convertuif.i8.f64"]
    pub fn convertuif_i8_f64(a: f64, b: i32, c: i32) -> i8;
    /// The `llvm.convertuif.v8i16.v4f32` intrinsic.
    #[link_name = "llvm.convertuif.v8i16.v4f32"]
    pub fn convertuif_v8i16_v4f32(a: ::simdty::f32x4, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertuif.v8i16.f32` intrinsic.
    #[link_name = "llvm.convertuif.v8i16.f32"]
    pub fn convertuif_v8i16_f32(a: f32, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertuif.v8i16.v2f64` intrinsic.
    #[link_name = "llvm.convertuif.v8i16.v2f64"]
    pub fn convertuif_v8i16_v2f64(a: ::simdty::f64x2, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertuif.v8i16.f64` intrinsic.
    #[link_name = "llvm.convertuif.v8i16.f64"]
    pub fn convertuif_v8i16_f64(a: f64, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertuif.i16.v4f32` intrinsic.
    #[link_name = "llvm.convertuif.i16.v4f32"]
    pub fn convertuif_i16_v4f32(a: ::simdty::f32x4, b: i32, c: i32) -> i16;
    /// The `llvm.convertuif.i16.f32` intrinsic.
    #[link_name = "llvm.convertuif.i16.f32"]
    pub fn convertuif_i16_f32(a: f32, b: i32, c: i32) -> i16;
    /// The `llvm.convertuif.i16.v2f64` intrinsic.
    #[link_name = "llvm.convertuif.i16.v2f64"]
    pub fn convertuif_i16_v2f64(a: ::simdty::f64x2, b: i32, c: i32) -> i16;
    /// The `llvm.convertuif.i16.f64` intrinsic.
    #[link_name = "llvm.convertuif.i16.f64"]
    pub fn convertuif_i16_f64(a: f64, b: i32, c: i32) -> i16;
    /// The `llvm.convertuif.v4i32.v4f32` intrinsic.
    #[link_name = "llvm.convertuif.v4i32.v4f32"]
    pub fn convertuif_v4i32_v4f32(a: ::simdty::f32x4, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertuif.v4i32.f32` intrinsic.
    #[link_name = "llvm.convertuif.v4i32.f32"]
    pub fn convertuif_v4i32_f32(a: f32, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertuif.v4i32.v2f64` intrinsic.
    #[link_name = "llvm.convertuif.v4i32.v2f64"]
    pub fn convertuif_v4i32_v2f64(a: ::simdty::f64x2, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertuif.v4i32.f64` intrinsic.
    #[link_name = "llvm.convertuif.v4i32.f64"]
    pub fn convertuif_v4i32_f64(a: f64, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertuif.i32.v4f32` intrinsic.
    #[link_name = "llvm.convertuif.i32.v4f32"]
    pub fn convertuif_i32_v4f32(a: ::simdty::f32x4, b: i32, c: i32) -> i32;
    /// The `llvm.convertuif.i32.f32` intrinsic.
    #[link_name = "llvm.convertuif.i32.f32"]
    pub fn convertuif_i32_f32(a: f32, b: i32, c: i32) -> i32;
    /// The `llvm.convertuif.i32.v2f64` intrinsic.
    #[link_name = "llvm.convertuif.i32.v2f64"]
    pub fn convertuif_i32_v2f64(a: ::simdty::f64x2, b: i32, c: i32) -> i32;
    /// The `llvm.convertuif.i32.f64` intrinsic.
    #[link_name = "llvm.convertuif.i32.f64"]
    pub fn convertuif_i32_f64(a: f64, b: i32, c: i32) -> i32;
    /// The `llvm.convertuif.v2i64.v4f32` intrinsic.
    #[link_name = "llvm.convertuif.v2i64.v4f32"]
    pub fn convertuif_v2i64_v4f32(a: ::simdty::f32x4, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertuif.v2i64.f32` intrinsic.
    #[link_name = "llvm.convertuif.v2i64.f32"]
    pub fn convertuif_v2i64_f32(a: f32, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertuif.v2i64.v2f64` intrinsic.
    #[link_name = "llvm.convertuif.v2i64.v2f64"]
    pub fn convertuif_v2i64_v2f64(a: ::simdty::f64x2, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertuif.v2i64.f64` intrinsic.
    #[link_name = "llvm.convertuif.v2i64.f64"]
    pub fn convertuif_v2i64_f64(a: f64, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertuif.i64.v4f32` intrinsic.
    #[link_name = "llvm.convertuif.i64.v4f32"]
    pub fn convertuif_i64_v4f32(a: ::simdty::f32x4, b: i32, c: i32) -> i64;
    /// The `llvm.convertuif.i64.f32` intrinsic.
    #[link_name = "llvm.convertuif.i64.f32"]
    pub fn convertuif_i64_f32(a: f32, b: i32, c: i32) -> i64;
    /// The `llvm.convertuif.i64.v2f64` intrinsic.
    #[link_name = "llvm.convertuif.i64.v2f64"]
    pub fn convertuif_i64_v2f64(a: ::simdty::f64x2, b: i32, c: i32) -> i64;
    /// The `llvm.convertuif.i64.f64` intrinsic.
    #[link_name = "llvm.convertuif.i64.f64"]
    pub fn convertuif_i64_f64(a: f64, b: i32, c: i32) -> i64;
    /// The `llvm.convertss.v16i8.v16i8` intrinsic.
    #[link_name = "llvm.convertss.v16i8.v16i8"]
    pub fn convertss_v16i8_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertss.v16i8.i8` intrinsic.
    #[link_name = "llvm.convertss.v16i8.i8"]
    pub fn convertss_v16i8_i8(a: i8, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertss.v16i8.v8i16` intrinsic.
    #[link_name = "llvm.convertss.v16i8.v8i16"]
    pub fn convertss_v16i8_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertss.v16i8.i16` intrinsic.
    #[link_name = "llvm.convertss.v16i8.i16"]
    pub fn convertss_v16i8_i16(a: i16, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertss.v16i8.v4i32` intrinsic.
    #[link_name = "llvm.convertss.v16i8.v4i32"]
    pub fn convertss_v16i8_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertss.v16i8.i32` intrinsic.
    #[link_name = "llvm.convertss.v16i8.i32"]
    pub fn convertss_v16i8_i32(a: i32, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertss.v16i8.v2i64` intrinsic.
    #[link_name = "llvm.convertss.v16i8.v2i64"]
    pub fn convertss_v16i8_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertss.v16i8.i64` intrinsic.
    #[link_name = "llvm.convertss.v16i8.i64"]
    pub fn convertss_v16i8_i64(a: i64, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertss.i8.v16i8` intrinsic.
    #[link_name = "llvm.convertss.i8.v16i8"]
    pub fn convertss_i8_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> i8;
    /// The `llvm.convertss.i8.i8` intrinsic.
    #[link_name = "llvm.convertss.i8.i8"]
    pub fn convertss_i8_i8(a: i8, b: i32, c: i32) -> i8;
    /// The `llvm.convertss.i8.v8i16` intrinsic.
    #[link_name = "llvm.convertss.i8.v8i16"]
    pub fn convertss_i8_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> i8;
    /// The `llvm.convertss.i8.i16` intrinsic.
    #[link_name = "llvm.convertss.i8.i16"]
    pub fn convertss_i8_i16(a: i16, b: i32, c: i32) -> i8;
    /// The `llvm.convertss.i8.v4i32` intrinsic.
    #[link_name = "llvm.convertss.i8.v4i32"]
    pub fn convertss_i8_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> i8;
    /// The `llvm.convertss.i8.i32` intrinsic.
    #[link_name = "llvm.convertss.i8.i32"]
    pub fn convertss_i8_i32(a: i32, b: i32, c: i32) -> i8;
    /// The `llvm.convertss.i8.v2i64` intrinsic.
    #[link_name = "llvm.convertss.i8.v2i64"]
    pub fn convertss_i8_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> i8;
    /// The `llvm.convertss.i8.i64` intrinsic.
    #[link_name = "llvm.convertss.i8.i64"]
    pub fn convertss_i8_i64(a: i64, b: i32, c: i32) -> i8;
    /// The `llvm.convertss.v8i16.v16i8` intrinsic.
    #[link_name = "llvm.convertss.v8i16.v16i8"]
    pub fn convertss_v8i16_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertss.v8i16.i8` intrinsic.
    #[link_name = "llvm.convertss.v8i16.i8"]
    pub fn convertss_v8i16_i8(a: i8, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertss.v8i16.v8i16` intrinsic.
    #[link_name = "llvm.convertss.v8i16.v8i16"]
    pub fn convertss_v8i16_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertss.v8i16.i16` intrinsic.
    #[link_name = "llvm.convertss.v8i16.i16"]
    pub fn convertss_v8i16_i16(a: i16, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertss.v8i16.v4i32` intrinsic.
    #[link_name = "llvm.convertss.v8i16.v4i32"]
    pub fn convertss_v8i16_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertss.v8i16.i32` intrinsic.
    #[link_name = "llvm.convertss.v8i16.i32"]
    pub fn convertss_v8i16_i32(a: i32, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertss.v8i16.v2i64` intrinsic.
    #[link_name = "llvm.convertss.v8i16.v2i64"]
    pub fn convertss_v8i16_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertss.v8i16.i64` intrinsic.
    #[link_name = "llvm.convertss.v8i16.i64"]
    pub fn convertss_v8i16_i64(a: i64, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertss.i16.v16i8` intrinsic.
    #[link_name = "llvm.convertss.i16.v16i8"]
    pub fn convertss_i16_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> i16;
    /// The `llvm.convertss.i16.i8` intrinsic.
    #[link_name = "llvm.convertss.i16.i8"]
    pub fn convertss_i16_i8(a: i8, b: i32, c: i32) -> i16;
    /// The `llvm.convertss.i16.v8i16` intrinsic.
    #[link_name = "llvm.convertss.i16.v8i16"]
    pub fn convertss_i16_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> i16;
    /// The `llvm.convertss.i16.i16` intrinsic.
    #[link_name = "llvm.convertss.i16.i16"]
    pub fn convertss_i16_i16(a: i16, b: i32, c: i32) -> i16;
    /// The `llvm.convertss.i16.v4i32` intrinsic.
    #[link_name = "llvm.convertss.i16.v4i32"]
    pub fn convertss_i16_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> i16;
    /// The `llvm.convertss.i16.i32` intrinsic.
    #[link_name = "llvm.convertss.i16.i32"]
    pub fn convertss_i16_i32(a: i32, b: i32, c: i32) -> i16;
    /// The `llvm.convertss.i16.v2i64` intrinsic.
    #[link_name = "llvm.convertss.i16.v2i64"]
    pub fn convertss_i16_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> i16;
    /// The `llvm.convertss.i16.i64` intrinsic.
    #[link_name = "llvm.convertss.i16.i64"]
    pub fn convertss_i16_i64(a: i64, b: i32, c: i32) -> i16;
    /// The `llvm.convertss.v4i32.v16i8` intrinsic.
    #[link_name = "llvm.convertss.v4i32.v16i8"]
    pub fn convertss_v4i32_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertss.v4i32.i8` intrinsic.
    #[link_name = "llvm.convertss.v4i32.i8"]
    pub fn convertss_v4i32_i8(a: i8, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertss.v4i32.v8i16` intrinsic.
    #[link_name = "llvm.convertss.v4i32.v8i16"]
    pub fn convertss_v4i32_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertss.v4i32.i16` intrinsic.
    #[link_name = "llvm.convertss.v4i32.i16"]
    pub fn convertss_v4i32_i16(a: i16, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertss.v4i32.v4i32` intrinsic.
    #[link_name = "llvm.convertss.v4i32.v4i32"]
    pub fn convertss_v4i32_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertss.v4i32.i32` intrinsic.
    #[link_name = "llvm.convertss.v4i32.i32"]
    pub fn convertss_v4i32_i32(a: i32, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertss.v4i32.v2i64` intrinsic.
    #[link_name = "llvm.convertss.v4i32.v2i64"]
    pub fn convertss_v4i32_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertss.v4i32.i64` intrinsic.
    #[link_name = "llvm.convertss.v4i32.i64"]
    pub fn convertss_v4i32_i64(a: i64, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertss.i32.v16i8` intrinsic.
    #[link_name = "llvm.convertss.i32.v16i8"]
    pub fn convertss_i32_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> i32;
    /// The `llvm.convertss.i32.i8` intrinsic.
    #[link_name = "llvm.convertss.i32.i8"]
    pub fn convertss_i32_i8(a: i8, b: i32, c: i32) -> i32;
    /// The `llvm.convertss.i32.v8i16` intrinsic.
    #[link_name = "llvm.convertss.i32.v8i16"]
    pub fn convertss_i32_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> i32;
    /// The `llvm.convertss.i32.i16` intrinsic.
    #[link_name = "llvm.convertss.i32.i16"]
    pub fn convertss_i32_i16(a: i16, b: i32, c: i32) -> i32;
    /// The `llvm.convertss.i32.v4i32` intrinsic.
    #[link_name = "llvm.convertss.i32.v4i32"]
    pub fn convertss_i32_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> i32;
    /// The `llvm.convertss.i32.i32` intrinsic.
    #[link_name = "llvm.convertss.i32.i32"]
    pub fn convertss_i32_i32(a: i32, b: i32, c: i32) -> i32;
    /// The `llvm.convertss.i32.v2i64` intrinsic.
    #[link_name = "llvm.convertss.i32.v2i64"]
    pub fn convertss_i32_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> i32;
    /// The `llvm.convertss.i32.i64` intrinsic.
    #[link_name = "llvm.convertss.i32.i64"]
    pub fn convertss_i32_i64(a: i64, b: i32, c: i32) -> i32;
    /// The `llvm.convertss.v2i64.v16i8` intrinsic.
    #[link_name = "llvm.convertss.v2i64.v16i8"]
    pub fn convertss_v2i64_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertss.v2i64.i8` intrinsic.
    #[link_name = "llvm.convertss.v2i64.i8"]
    pub fn convertss_v2i64_i8(a: i8, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertss.v2i64.v8i16` intrinsic.
    #[link_name = "llvm.convertss.v2i64.v8i16"]
    pub fn convertss_v2i64_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertss.v2i64.i16` intrinsic.
    #[link_name = "llvm.convertss.v2i64.i16"]
    pub fn convertss_v2i64_i16(a: i16, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertss.v2i64.v4i32` intrinsic.
    #[link_name = "llvm.convertss.v2i64.v4i32"]
    pub fn convertss_v2i64_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertss.v2i64.i32` intrinsic.
    #[link_name = "llvm.convertss.v2i64.i32"]
    pub fn convertss_v2i64_i32(a: i32, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertss.v2i64.v2i64` intrinsic.
    #[link_name = "llvm.convertss.v2i64.v2i64"]
    pub fn convertss_v2i64_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertss.v2i64.i64` intrinsic.
    #[link_name = "llvm.convertss.v2i64.i64"]
    pub fn convertss_v2i64_i64(a: i64, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertss.i64.v16i8` intrinsic.
    #[link_name = "llvm.convertss.i64.v16i8"]
    pub fn convertss_i64_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> i64;
    /// The `llvm.convertss.i64.i8` intrinsic.
    #[link_name = "llvm.convertss.i64.i8"]
    pub fn convertss_i64_i8(a: i8, b: i32, c: i32) -> i64;
    /// The `llvm.convertss.i64.v8i16` intrinsic.
    #[link_name = "llvm.convertss.i64.v8i16"]
    pub fn convertss_i64_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> i64;
    /// The `llvm.convertss.i64.i16` intrinsic.
    #[link_name = "llvm.convertss.i64.i16"]
    pub fn convertss_i64_i16(a: i16, b: i32, c: i32) -> i64;
    /// The `llvm.convertss.i64.v4i32` intrinsic.
    #[link_name = "llvm.convertss.i64.v4i32"]
    pub fn convertss_i64_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> i64;
    /// The `llvm.convertss.i64.i32` intrinsic.
    #[link_name = "llvm.convertss.i64.i32"]
    pub fn convertss_i64_i32(a: i32, b: i32, c: i32) -> i64;
    /// The `llvm.convertss.i64.v2i64` intrinsic.
    #[link_name = "llvm.convertss.i64.v2i64"]
    pub fn convertss_i64_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> i64;
    /// The `llvm.convertss.i64.i64` intrinsic.
    #[link_name = "llvm.convertss.i64.i64"]
    pub fn convertss_i64_i64(a: i64, b: i32, c: i32) -> i64;
    /// The `llvm.convertsu.v16i8.v16i8` intrinsic.
    #[link_name = "llvm.convertsu.v16i8.v16i8"]
    pub fn convertsu_v16i8_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertsu.v16i8.i8` intrinsic.
    #[link_name = "llvm.convertsu.v16i8.i8"]
    pub fn convertsu_v16i8_i8(a: i8, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertsu.v16i8.v8i16` intrinsic.
    #[link_name = "llvm.convertsu.v16i8.v8i16"]
    pub fn convertsu_v16i8_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertsu.v16i8.i16` intrinsic.
    #[link_name = "llvm.convertsu.v16i8.i16"]
    pub fn convertsu_v16i8_i16(a: i16, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertsu.v16i8.v4i32` intrinsic.
    #[link_name = "llvm.convertsu.v16i8.v4i32"]
    pub fn convertsu_v16i8_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertsu.v16i8.i32` intrinsic.
    #[link_name = "llvm.convertsu.v16i8.i32"]
    pub fn convertsu_v16i8_i32(a: i32, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertsu.v16i8.v2i64` intrinsic.
    #[link_name = "llvm.convertsu.v16i8.v2i64"]
    pub fn convertsu_v16i8_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertsu.v16i8.i64` intrinsic.
    #[link_name = "llvm.convertsu.v16i8.i64"]
    pub fn convertsu_v16i8_i64(a: i64, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertsu.i8.v16i8` intrinsic.
    #[link_name = "llvm.convertsu.i8.v16i8"]
    pub fn convertsu_i8_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> i8;
    /// The `llvm.convertsu.i8.i8` intrinsic.
    #[link_name = "llvm.convertsu.i8.i8"]
    pub fn convertsu_i8_i8(a: i8, b: i32, c: i32) -> i8;
    /// The `llvm.convertsu.i8.v8i16` intrinsic.
    #[link_name = "llvm.convertsu.i8.v8i16"]
    pub fn convertsu_i8_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> i8;
    /// The `llvm.convertsu.i8.i16` intrinsic.
    #[link_name = "llvm.convertsu.i8.i16"]
    pub fn convertsu_i8_i16(a: i16, b: i32, c: i32) -> i8;
    /// The `llvm.convertsu.i8.v4i32` intrinsic.
    #[link_name = "llvm.convertsu.i8.v4i32"]
    pub fn convertsu_i8_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> i8;
    /// The `llvm.convertsu.i8.i32` intrinsic.
    #[link_name = "llvm.convertsu.i8.i32"]
    pub fn convertsu_i8_i32(a: i32, b: i32, c: i32) -> i8;
    /// The `llvm.convertsu.i8.v2i64` intrinsic.
    #[link_name = "llvm.convertsu.i8.v2i64"]
    pub fn convertsu_i8_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> i8;
    /// The `llvm.convertsu.i8.i64` intrinsic.
    #[link_name = "llvm.convertsu.i8.i64"]
    pub fn convertsu_i8_i64(a: i64, b: i32, c: i32) -> i8;
    /// The `llvm.convertsu.v8i16.v16i8` intrinsic.
    #[link_name = "llvm.convertsu.v8i16.v16i8"]
    pub fn convertsu_v8i16_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertsu.v8i16.i8` intrinsic.
    #[link_name = "llvm.convertsu.v8i16.i8"]
    pub fn convertsu_v8i16_i8(a: i8, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertsu.v8i16.v8i16` intrinsic.
    #[link_name = "llvm.convertsu.v8i16.v8i16"]
    pub fn convertsu_v8i16_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertsu.v8i16.i16` intrinsic.
    #[link_name = "llvm.convertsu.v8i16.i16"]
    pub fn convertsu_v8i16_i16(a: i16, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertsu.v8i16.v4i32` intrinsic.
    #[link_name = "llvm.convertsu.v8i16.v4i32"]
    pub fn convertsu_v8i16_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertsu.v8i16.i32` intrinsic.
    #[link_name = "llvm.convertsu.v8i16.i32"]
    pub fn convertsu_v8i16_i32(a: i32, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertsu.v8i16.v2i64` intrinsic.
    #[link_name = "llvm.convertsu.v8i16.v2i64"]
    pub fn convertsu_v8i16_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertsu.v8i16.i64` intrinsic.
    #[link_name = "llvm.convertsu.v8i16.i64"]
    pub fn convertsu_v8i16_i64(a: i64, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertsu.i16.v16i8` intrinsic.
    #[link_name = "llvm.convertsu.i16.v16i8"]
    pub fn convertsu_i16_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> i16;
    /// The `llvm.convertsu.i16.i8` intrinsic.
    #[link_name = "llvm.convertsu.i16.i8"]
    pub fn convertsu_i16_i8(a: i8, b: i32, c: i32) -> i16;
    /// The `llvm.convertsu.i16.v8i16` intrinsic.
    #[link_name = "llvm.convertsu.i16.v8i16"]
    pub fn convertsu_i16_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> i16;
    /// The `llvm.convertsu.i16.i16` intrinsic.
    #[link_name = "llvm.convertsu.i16.i16"]
    pub fn convertsu_i16_i16(a: i16, b: i32, c: i32) -> i16;
    /// The `llvm.convertsu.i16.v4i32` intrinsic.
    #[link_name = "llvm.convertsu.i16.v4i32"]
    pub fn convertsu_i16_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> i16;
    /// The `llvm.convertsu.i16.i32` intrinsic.
    #[link_name = "llvm.convertsu.i16.i32"]
    pub fn convertsu_i16_i32(a: i32, b: i32, c: i32) -> i16;
    /// The `llvm.convertsu.i16.v2i64` intrinsic.
    #[link_name = "llvm.convertsu.i16.v2i64"]
    pub fn convertsu_i16_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> i16;
    /// The `llvm.convertsu.i16.i64` intrinsic.
    #[link_name = "llvm.convertsu.i16.i64"]
    pub fn convertsu_i16_i64(a: i64, b: i32, c: i32) -> i16;
    /// The `llvm.convertsu.v4i32.v16i8` intrinsic.
    #[link_name = "llvm.convertsu.v4i32.v16i8"]
    pub fn convertsu_v4i32_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertsu.v4i32.i8` intrinsic.
    #[link_name = "llvm.convertsu.v4i32.i8"]
    pub fn convertsu_v4i32_i8(a: i8, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertsu.v4i32.v8i16` intrinsic.
    #[link_name = "llvm.convertsu.v4i32.v8i16"]
    pub fn convertsu_v4i32_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertsu.v4i32.i16` intrinsic.
    #[link_name = "llvm.convertsu.v4i32.i16"]
    pub fn convertsu_v4i32_i16(a: i16, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertsu.v4i32.v4i32` intrinsic.
    #[link_name = "llvm.convertsu.v4i32.v4i32"]
    pub fn convertsu_v4i32_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertsu.v4i32.i32` intrinsic.
    #[link_name = "llvm.convertsu.v4i32.i32"]
    pub fn convertsu_v4i32_i32(a: i32, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertsu.v4i32.v2i64` intrinsic.
    #[link_name = "llvm.convertsu.v4i32.v2i64"]
    pub fn convertsu_v4i32_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertsu.v4i32.i64` intrinsic.
    #[link_name = "llvm.convertsu.v4i32.i64"]
    pub fn convertsu_v4i32_i64(a: i64, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertsu.i32.v16i8` intrinsic.
    #[link_name = "llvm.convertsu.i32.v16i8"]
    pub fn convertsu_i32_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> i32;
    /// The `llvm.convertsu.i32.i8` intrinsic.
    #[link_name = "llvm.convertsu.i32.i8"]
    pub fn convertsu_i32_i8(a: i8, b: i32, c: i32) -> i32;
    /// The `llvm.convertsu.i32.v8i16` intrinsic.
    #[link_name = "llvm.convertsu.i32.v8i16"]
    pub fn convertsu_i32_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> i32;
    /// The `llvm.convertsu.i32.i16` intrinsic.
    #[link_name = "llvm.convertsu.i32.i16"]
    pub fn convertsu_i32_i16(a: i16, b: i32, c: i32) -> i32;
    /// The `llvm.convertsu.i32.v4i32` intrinsic.
    #[link_name = "llvm.convertsu.i32.v4i32"]
    pub fn convertsu_i32_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> i32;
    /// The `llvm.convertsu.i32.i32` intrinsic.
    #[link_name = "llvm.convertsu.i32.i32"]
    pub fn convertsu_i32_i32(a: i32, b: i32, c: i32) -> i32;
    /// The `llvm.convertsu.i32.v2i64` intrinsic.
    #[link_name = "llvm.convertsu.i32.v2i64"]
    pub fn convertsu_i32_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> i32;
    /// The `llvm.convertsu.i32.i64` intrinsic.
    #[link_name = "llvm.convertsu.i32.i64"]
    pub fn convertsu_i32_i64(a: i64, b: i32, c: i32) -> i32;
    /// The `llvm.convertsu.v2i64.v16i8` intrinsic.
    #[link_name = "llvm.convertsu.v2i64.v16i8"]
    pub fn convertsu_v2i64_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertsu.v2i64.i8` intrinsic.
    #[link_name = "llvm.convertsu.v2i64.i8"]
    pub fn convertsu_v2i64_i8(a: i8, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertsu.v2i64.v8i16` intrinsic.
    #[link_name = "llvm.convertsu.v2i64.v8i16"]
    pub fn convertsu_v2i64_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertsu.v2i64.i16` intrinsic.
    #[link_name = "llvm.convertsu.v2i64.i16"]
    pub fn convertsu_v2i64_i16(a: i16, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertsu.v2i64.v4i32` intrinsic.
    #[link_name = "llvm.convertsu.v2i64.v4i32"]
    pub fn convertsu_v2i64_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertsu.v2i64.i32` intrinsic.
    #[link_name = "llvm.convertsu.v2i64.i32"]
    pub fn convertsu_v2i64_i32(a: i32, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertsu.v2i64.v2i64` intrinsic.
    #[link_name = "llvm.convertsu.v2i64.v2i64"]
    pub fn convertsu_v2i64_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertsu.v2i64.i64` intrinsic.
    #[link_name = "llvm.convertsu.v2i64.i64"]
    pub fn convertsu_v2i64_i64(a: i64, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertsu.i64.v16i8` intrinsic.
    #[link_name = "llvm.convertsu.i64.v16i8"]
    pub fn convertsu_i64_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> i64;
    /// The `llvm.convertsu.i64.i8` intrinsic.
    #[link_name = "llvm.convertsu.i64.i8"]
    pub fn convertsu_i64_i8(a: i8, b: i32, c: i32) -> i64;
    /// The `llvm.convertsu.i64.v8i16` intrinsic.
    #[link_name = "llvm.convertsu.i64.v8i16"]
    pub fn convertsu_i64_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> i64;
    /// The `llvm.convertsu.i64.i16` intrinsic.
    #[link_name = "llvm.convertsu.i64.i16"]
    pub fn convertsu_i64_i16(a: i16, b: i32, c: i32) -> i64;
    /// The `llvm.convertsu.i64.v4i32` intrinsic.
    #[link_name = "llvm.convertsu.i64.v4i32"]
    pub fn convertsu_i64_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> i64;
    /// The `llvm.convertsu.i64.i32` intrinsic.
    #[link_name = "llvm.convertsu.i64.i32"]
    pub fn convertsu_i64_i32(a: i32, b: i32, c: i32) -> i64;
    /// The `llvm.convertsu.i64.v2i64` intrinsic.
    #[link_name = "llvm.convertsu.i64.v2i64"]
    pub fn convertsu_i64_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> i64;
    /// The `llvm.convertsu.i64.i64` intrinsic.
    #[link_name = "llvm.convertsu.i64.i64"]
    pub fn convertsu_i64_i64(a: i64, b: i32, c: i32) -> i64;
    /// The `llvm.convertus.v16i8.v16i8` intrinsic.
    #[link_name = "llvm.convertus.v16i8.v16i8"]
    pub fn convertus_v16i8_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertus.v16i8.i8` intrinsic.
    #[link_name = "llvm.convertus.v16i8.i8"]
    pub fn convertus_v16i8_i8(a: i8, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertus.v16i8.v8i16` intrinsic.
    #[link_name = "llvm.convertus.v16i8.v8i16"]
    pub fn convertus_v16i8_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertus.v16i8.i16` intrinsic.
    #[link_name = "llvm.convertus.v16i8.i16"]
    pub fn convertus_v16i8_i16(a: i16, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertus.v16i8.v4i32` intrinsic.
    #[link_name = "llvm.convertus.v16i8.v4i32"]
    pub fn convertus_v16i8_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertus.v16i8.i32` intrinsic.
    #[link_name = "llvm.convertus.v16i8.i32"]
    pub fn convertus_v16i8_i32(a: i32, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertus.v16i8.v2i64` intrinsic.
    #[link_name = "llvm.convertus.v16i8.v2i64"]
    pub fn convertus_v16i8_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertus.v16i8.i64` intrinsic.
    #[link_name = "llvm.convertus.v16i8.i64"]
    pub fn convertus_v16i8_i64(a: i64, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertus.i8.v16i8` intrinsic.
    #[link_name = "llvm.convertus.i8.v16i8"]
    pub fn convertus_i8_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> i8;
    /// The `llvm.convertus.i8.i8` intrinsic.
    #[link_name = "llvm.convertus.i8.i8"]
    pub fn convertus_i8_i8(a: i8, b: i32, c: i32) -> i8;
    /// The `llvm.convertus.i8.v8i16` intrinsic.
    #[link_name = "llvm.convertus.i8.v8i16"]
    pub fn convertus_i8_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> i8;
    /// The `llvm.convertus.i8.i16` intrinsic.
    #[link_name = "llvm.convertus.i8.i16"]
    pub fn convertus_i8_i16(a: i16, b: i32, c: i32) -> i8;
    /// The `llvm.convertus.i8.v4i32` intrinsic.
    #[link_name = "llvm.convertus.i8.v4i32"]
    pub fn convertus_i8_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> i8;
    /// The `llvm.convertus.i8.i32` intrinsic.
    #[link_name = "llvm.convertus.i8.i32"]
    pub fn convertus_i8_i32(a: i32, b: i32, c: i32) -> i8;
    /// The `llvm.convertus.i8.v2i64` intrinsic.
    #[link_name = "llvm.convertus.i8.v2i64"]
    pub fn convertus_i8_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> i8;
    /// The `llvm.convertus.i8.i64` intrinsic.
    #[link_name = "llvm.convertus.i8.i64"]
    pub fn convertus_i8_i64(a: i64, b: i32, c: i32) -> i8;
    /// The `llvm.convertus.v8i16.v16i8` intrinsic.
    #[link_name = "llvm.convertus.v8i16.v16i8"]
    pub fn convertus_v8i16_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertus.v8i16.i8` intrinsic.
    #[link_name = "llvm.convertus.v8i16.i8"]
    pub fn convertus_v8i16_i8(a: i8, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertus.v8i16.v8i16` intrinsic.
    #[link_name = "llvm.convertus.v8i16.v8i16"]
    pub fn convertus_v8i16_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertus.v8i16.i16` intrinsic.
    #[link_name = "llvm.convertus.v8i16.i16"]
    pub fn convertus_v8i16_i16(a: i16, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertus.v8i16.v4i32` intrinsic.
    #[link_name = "llvm.convertus.v8i16.v4i32"]
    pub fn convertus_v8i16_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertus.v8i16.i32` intrinsic.
    #[link_name = "llvm.convertus.v8i16.i32"]
    pub fn convertus_v8i16_i32(a: i32, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertus.v8i16.v2i64` intrinsic.
    #[link_name = "llvm.convertus.v8i16.v2i64"]
    pub fn convertus_v8i16_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertus.v8i16.i64` intrinsic.
    #[link_name = "llvm.convertus.v8i16.i64"]
    pub fn convertus_v8i16_i64(a: i64, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertus.i16.v16i8` intrinsic.
    #[link_name = "llvm.convertus.i16.v16i8"]
    pub fn convertus_i16_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> i16;
    /// The `llvm.convertus.i16.i8` intrinsic.
    #[link_name = "llvm.convertus.i16.i8"]
    pub fn convertus_i16_i8(a: i8, b: i32, c: i32) -> i16;
    /// The `llvm.convertus.i16.v8i16` intrinsic.
    #[link_name = "llvm.convertus.i16.v8i16"]
    pub fn convertus_i16_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> i16;
    /// The `llvm.convertus.i16.i16` intrinsic.
    #[link_name = "llvm.convertus.i16.i16"]
    pub fn convertus_i16_i16(a: i16, b: i32, c: i32) -> i16;
    /// The `llvm.convertus.i16.v4i32` intrinsic.
    #[link_name = "llvm.convertus.i16.v4i32"]
    pub fn convertus_i16_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> i16;
    /// The `llvm.convertus.i16.i32` intrinsic.
    #[link_name = "llvm.convertus.i16.i32"]
    pub fn convertus_i16_i32(a: i32, b: i32, c: i32) -> i16;
    /// The `llvm.convertus.i16.v2i64` intrinsic.
    #[link_name = "llvm.convertus.i16.v2i64"]
    pub fn convertus_i16_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> i16;
    /// The `llvm.convertus.i16.i64` intrinsic.
    #[link_name = "llvm.convertus.i16.i64"]
    pub fn convertus_i16_i64(a: i64, b: i32, c: i32) -> i16;
    /// The `llvm.convertus.v4i32.v16i8` intrinsic.
    #[link_name = "llvm.convertus.v4i32.v16i8"]
    pub fn convertus_v4i32_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertus.v4i32.i8` intrinsic.
    #[link_name = "llvm.convertus.v4i32.i8"]
    pub fn convertus_v4i32_i8(a: i8, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertus.v4i32.v8i16` intrinsic.
    #[link_name = "llvm.convertus.v4i32.v8i16"]
    pub fn convertus_v4i32_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertus.v4i32.i16` intrinsic.
    #[link_name = "llvm.convertus.v4i32.i16"]
    pub fn convertus_v4i32_i16(a: i16, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertus.v4i32.v4i32` intrinsic.
    #[link_name = "llvm.convertus.v4i32.v4i32"]
    pub fn convertus_v4i32_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertus.v4i32.i32` intrinsic.
    #[link_name = "llvm.convertus.v4i32.i32"]
    pub fn convertus_v4i32_i32(a: i32, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertus.v4i32.v2i64` intrinsic.
    #[link_name = "llvm.convertus.v4i32.v2i64"]
    pub fn convertus_v4i32_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertus.v4i32.i64` intrinsic.
    #[link_name = "llvm.convertus.v4i32.i64"]
    pub fn convertus_v4i32_i64(a: i64, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertus.i32.v16i8` intrinsic.
    #[link_name = "llvm.convertus.i32.v16i8"]
    pub fn convertus_i32_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> i32;
    /// The `llvm.convertus.i32.i8` intrinsic.
    #[link_name = "llvm.convertus.i32.i8"]
    pub fn convertus_i32_i8(a: i8, b: i32, c: i32) -> i32;
    /// The `llvm.convertus.i32.v8i16` intrinsic.
    #[link_name = "llvm.convertus.i32.v8i16"]
    pub fn convertus_i32_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> i32;
    /// The `llvm.convertus.i32.i16` intrinsic.
    #[link_name = "llvm.convertus.i32.i16"]
    pub fn convertus_i32_i16(a: i16, b: i32, c: i32) -> i32;
    /// The `llvm.convertus.i32.v4i32` intrinsic.
    #[link_name = "llvm.convertus.i32.v4i32"]
    pub fn convertus_i32_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> i32;
    /// The `llvm.convertus.i32.i32` intrinsic.
    #[link_name = "llvm.convertus.i32.i32"]
    pub fn convertus_i32_i32(a: i32, b: i32, c: i32) -> i32;
    /// The `llvm.convertus.i32.v2i64` intrinsic.
    #[link_name = "llvm.convertus.i32.v2i64"]
    pub fn convertus_i32_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> i32;
    /// The `llvm.convertus.i32.i64` intrinsic.
    #[link_name = "llvm.convertus.i32.i64"]
    pub fn convertus_i32_i64(a: i64, b: i32, c: i32) -> i32;
    /// The `llvm.convertus.v2i64.v16i8` intrinsic.
    #[link_name = "llvm.convertus.v2i64.v16i8"]
    pub fn convertus_v2i64_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertus.v2i64.i8` intrinsic.
    #[link_name = "llvm.convertus.v2i64.i8"]
    pub fn convertus_v2i64_i8(a: i8, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertus.v2i64.v8i16` intrinsic.
    #[link_name = "llvm.convertus.v2i64.v8i16"]
    pub fn convertus_v2i64_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertus.v2i64.i16` intrinsic.
    #[link_name = "llvm.convertus.v2i64.i16"]
    pub fn convertus_v2i64_i16(a: i16, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertus.v2i64.v4i32` intrinsic.
    #[link_name = "llvm.convertus.v2i64.v4i32"]
    pub fn convertus_v2i64_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertus.v2i64.i32` intrinsic.
    #[link_name = "llvm.convertus.v2i64.i32"]
    pub fn convertus_v2i64_i32(a: i32, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertus.v2i64.v2i64` intrinsic.
    #[link_name = "llvm.convertus.v2i64.v2i64"]
    pub fn convertus_v2i64_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertus.v2i64.i64` intrinsic.
    #[link_name = "llvm.convertus.v2i64.i64"]
    pub fn convertus_v2i64_i64(a: i64, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertus.i64.v16i8` intrinsic.
    #[link_name = "llvm.convertus.i64.v16i8"]
    pub fn convertus_i64_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> i64;
    /// The `llvm.convertus.i64.i8` intrinsic.
    #[link_name = "llvm.convertus.i64.i8"]
    pub fn convertus_i64_i8(a: i8, b: i32, c: i32) -> i64;
    /// The `llvm.convertus.i64.v8i16` intrinsic.
    #[link_name = "llvm.convertus.i64.v8i16"]
    pub fn convertus_i64_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> i64;
    /// The `llvm.convertus.i64.i16` intrinsic.
    #[link_name = "llvm.convertus.i64.i16"]
    pub fn convertus_i64_i16(a: i16, b: i32, c: i32) -> i64;
    /// The `llvm.convertus.i64.v4i32` intrinsic.
    #[link_name = "llvm.convertus.i64.v4i32"]
    pub fn convertus_i64_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> i64;
    /// The `llvm.convertus.i64.i32` intrinsic.
    #[link_name = "llvm.convertus.i64.i32"]
    pub fn convertus_i64_i32(a: i32, b: i32, c: i32) -> i64;
    /// The `llvm.convertus.i64.v2i64` intrinsic.
    #[link_name = "llvm.convertus.i64.v2i64"]
    pub fn convertus_i64_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> i64;
    /// The `llvm.convertus.i64.i64` intrinsic.
    #[link_name = "llvm.convertus.i64.i64"]
    pub fn convertus_i64_i64(a: i64, b: i32, c: i32) -> i64;
    /// The `llvm.convertuu.v16i8.v16i8` intrinsic.
    #[link_name = "llvm.convertuu.v16i8.v16i8"]
    pub fn convertuu_v16i8_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertuu.v16i8.i8` intrinsic.
    #[link_name = "llvm.convertuu.v16i8.i8"]
    pub fn convertuu_v16i8_i8(a: i8, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertuu.v16i8.v8i16` intrinsic.
    #[link_name = "llvm.convertuu.v16i8.v8i16"]
    pub fn convertuu_v16i8_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertuu.v16i8.i16` intrinsic.
    #[link_name = "llvm.convertuu.v16i8.i16"]
    pub fn convertuu_v16i8_i16(a: i16, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertuu.v16i8.v4i32` intrinsic.
    #[link_name = "llvm.convertuu.v16i8.v4i32"]
    pub fn convertuu_v16i8_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertuu.v16i8.i32` intrinsic.
    #[link_name = "llvm.convertuu.v16i8.i32"]
    pub fn convertuu_v16i8_i32(a: i32, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertuu.v16i8.v2i64` intrinsic.
    #[link_name = "llvm.convertuu.v16i8.v2i64"]
    pub fn convertuu_v16i8_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertuu.v16i8.i64` intrinsic.
    #[link_name = "llvm.convertuu.v16i8.i64"]
    pub fn convertuu_v16i8_i64(a: i64, b: i32, c: i32) -> ::simdty::i8x16;
    /// The `llvm.convertuu.i8.v16i8` intrinsic.
    #[link_name = "llvm.convertuu.i8.v16i8"]
    pub fn convertuu_i8_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> i8;
    /// The `llvm.convertuu.i8.i8` intrinsic.
    #[link_name = "llvm.convertuu.i8.i8"]
    pub fn convertuu_i8_i8(a: i8, b: i32, c: i32) -> i8;
    /// The `llvm.convertuu.i8.v8i16` intrinsic.
    #[link_name = "llvm.convertuu.i8.v8i16"]
    pub fn convertuu_i8_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> i8;
    /// The `llvm.convertuu.i8.i16` intrinsic.
    #[link_name = "llvm.convertuu.i8.i16"]
    pub fn convertuu_i8_i16(a: i16, b: i32, c: i32) -> i8;
    /// The `llvm.convertuu.i8.v4i32` intrinsic.
    #[link_name = "llvm.convertuu.i8.v4i32"]
    pub fn convertuu_i8_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> i8;
    /// The `llvm.convertuu.i8.i32` intrinsic.
    #[link_name = "llvm.convertuu.i8.i32"]
    pub fn convertuu_i8_i32(a: i32, b: i32, c: i32) -> i8;
    /// The `llvm.convertuu.i8.v2i64` intrinsic.
    #[link_name = "llvm.convertuu.i8.v2i64"]
    pub fn convertuu_i8_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> i8;
    /// The `llvm.convertuu.i8.i64` intrinsic.
    #[link_name = "llvm.convertuu.i8.i64"]
    pub fn convertuu_i8_i64(a: i64, b: i32, c: i32) -> i8;
    /// The `llvm.convertuu.v8i16.v16i8` intrinsic.
    #[link_name = "llvm.convertuu.v8i16.v16i8"]
    pub fn convertuu_v8i16_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertuu.v8i16.i8` intrinsic.
    #[link_name = "llvm.convertuu.v8i16.i8"]
    pub fn convertuu_v8i16_i8(a: i8, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertuu.v8i16.v8i16` intrinsic.
    #[link_name = "llvm.convertuu.v8i16.v8i16"]
    pub fn convertuu_v8i16_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertuu.v8i16.i16` intrinsic.
    #[link_name = "llvm.convertuu.v8i16.i16"]
    pub fn convertuu_v8i16_i16(a: i16, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertuu.v8i16.v4i32` intrinsic.
    #[link_name = "llvm.convertuu.v8i16.v4i32"]
    pub fn convertuu_v8i16_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertuu.v8i16.i32` intrinsic.
    #[link_name = "llvm.convertuu.v8i16.i32"]
    pub fn convertuu_v8i16_i32(a: i32, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertuu.v8i16.v2i64` intrinsic.
    #[link_name = "llvm.convertuu.v8i16.v2i64"]
    pub fn convertuu_v8i16_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertuu.v8i16.i64` intrinsic.
    #[link_name = "llvm.convertuu.v8i16.i64"]
    pub fn convertuu_v8i16_i64(a: i64, b: i32, c: i32) -> ::simdty::i16x8;
    /// The `llvm.convertuu.i16.v16i8` intrinsic.
    #[link_name = "llvm.convertuu.i16.v16i8"]
    pub fn convertuu_i16_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> i16;
    /// The `llvm.convertuu.i16.i8` intrinsic.
    #[link_name = "llvm.convertuu.i16.i8"]
    pub fn convertuu_i16_i8(a: i8, b: i32, c: i32) -> i16;
    /// The `llvm.convertuu.i16.v8i16` intrinsic.
    #[link_name = "llvm.convertuu.i16.v8i16"]
    pub fn convertuu_i16_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> i16;
    /// The `llvm.convertuu.i16.i16` intrinsic.
    #[link_name = "llvm.convertuu.i16.i16"]
    pub fn convertuu_i16_i16(a: i16, b: i32, c: i32) -> i16;
    /// The `llvm.convertuu.i16.v4i32` intrinsic.
    #[link_name = "llvm.convertuu.i16.v4i32"]
    pub fn convertuu_i16_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> i16;
    /// The `llvm.convertuu.i16.i32` intrinsic.
    #[link_name = "llvm.convertuu.i16.i32"]
    pub fn convertuu_i16_i32(a: i32, b: i32, c: i32) -> i16;
    /// The `llvm.convertuu.i16.v2i64` intrinsic.
    #[link_name = "llvm.convertuu.i16.v2i64"]
    pub fn convertuu_i16_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> i16;
    /// The `llvm.convertuu.i16.i64` intrinsic.
    #[link_name = "llvm.convertuu.i16.i64"]
    pub fn convertuu_i16_i64(a: i64, b: i32, c: i32) -> i16;
    /// The `llvm.convertuu.v4i32.v16i8` intrinsic.
    #[link_name = "llvm.convertuu.v4i32.v16i8"]
    pub fn convertuu_v4i32_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertuu.v4i32.i8` intrinsic.
    #[link_name = "llvm.convertuu.v4i32.i8"]
    pub fn convertuu_v4i32_i8(a: i8, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertuu.v4i32.v8i16` intrinsic.
    #[link_name = "llvm.convertuu.v4i32.v8i16"]
    pub fn convertuu_v4i32_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertuu.v4i32.i16` intrinsic.
    #[link_name = "llvm.convertuu.v4i32.i16"]
    pub fn convertuu_v4i32_i16(a: i16, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertuu.v4i32.v4i32` intrinsic.
    #[link_name = "llvm.convertuu.v4i32.v4i32"]
    pub fn convertuu_v4i32_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertuu.v4i32.i32` intrinsic.
    #[link_name = "llvm.convertuu.v4i32.i32"]
    pub fn convertuu_v4i32_i32(a: i32, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertuu.v4i32.v2i64` intrinsic.
    #[link_name = "llvm.convertuu.v4i32.v2i64"]
    pub fn convertuu_v4i32_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertuu.v4i32.i64` intrinsic.
    #[link_name = "llvm.convertuu.v4i32.i64"]
    pub fn convertuu_v4i32_i64(a: i64, b: i32, c: i32) -> ::simdty::i32x4;
    /// The `llvm.convertuu.i32.v16i8` intrinsic.
    #[link_name = "llvm.convertuu.i32.v16i8"]
    pub fn convertuu_i32_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> i32;
    /// The `llvm.convertuu.i32.i8` intrinsic.
    #[link_name = "llvm.convertuu.i32.i8"]
    pub fn convertuu_i32_i8(a: i8, b: i32, c: i32) -> i32;
    /// The `llvm.convertuu.i32.v8i16` intrinsic.
    #[link_name = "llvm.convertuu.i32.v8i16"]
    pub fn convertuu_i32_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> i32;
    /// The `llvm.convertuu.i32.i16` intrinsic.
    #[link_name = "llvm.convertuu.i32.i16"]
    pub fn convertuu_i32_i16(a: i16, b: i32, c: i32) -> i32;
    /// The `llvm.convertuu.i32.v4i32` intrinsic.
    #[link_name = "llvm.convertuu.i32.v4i32"]
    pub fn convertuu_i32_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> i32;
    /// The `llvm.convertuu.i32.i32` intrinsic.
    #[link_name = "llvm.convertuu.i32.i32"]
    pub fn convertuu_i32_i32(a: i32, b: i32, c: i32) -> i32;
    /// The `llvm.convertuu.i32.v2i64` intrinsic.
    #[link_name = "llvm.convertuu.i32.v2i64"]
    pub fn convertuu_i32_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> i32;
    /// The `llvm.convertuu.i32.i64` intrinsic.
    #[link_name = "llvm.convertuu.i32.i64"]
    pub fn convertuu_i32_i64(a: i64, b: i32, c: i32) -> i32;
    /// The `llvm.convertuu.v2i64.v16i8` intrinsic.
    #[link_name = "llvm.convertuu.v2i64.v16i8"]
    pub fn convertuu_v2i64_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertuu.v2i64.i8` intrinsic.
    #[link_name = "llvm.convertuu.v2i64.i8"]
    pub fn convertuu_v2i64_i8(a: i8, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertuu.v2i64.v8i16` intrinsic.
    #[link_name = "llvm.convertuu.v2i64.v8i16"]
    pub fn convertuu_v2i64_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertuu.v2i64.i16` intrinsic.
    #[link_name = "llvm.convertuu.v2i64.i16"]
    pub fn convertuu_v2i64_i16(a: i16, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertuu.v2i64.v4i32` intrinsic.
    #[link_name = "llvm.convertuu.v2i64.v4i32"]
    pub fn convertuu_v2i64_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertuu.v2i64.i32` intrinsic.
    #[link_name = "llvm.convertuu.v2i64.i32"]
    pub fn convertuu_v2i64_i32(a: i32, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertuu.v2i64.v2i64` intrinsic.
    #[link_name = "llvm.convertuu.v2i64.v2i64"]
    pub fn convertuu_v2i64_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertuu.v2i64.i64` intrinsic.
    #[link_name = "llvm.convertuu.v2i64.i64"]
    pub fn convertuu_v2i64_i64(a: i64, b: i32, c: i32) -> ::simdty::i64x2;
    /// The `llvm.convertuu.i64.v16i8` intrinsic.
    #[link_name = "llvm.convertuu.i64.v16i8"]
    pub fn convertuu_i64_v16i8(a: ::simdty::i8x16, b: i32, c: i32) -> i64;
    /// The `llvm.convertuu.i64.i8` intrinsic.
    #[link_name = "llvm.convertuu.i64.i8"]
    pub fn convertuu_i64_i8(a: i8, b: i32, c: i32) -> i64;
    /// The `llvm.convertuu.i64.v8i16` intrinsic.
    #[link_name = "llvm.convertuu.i64.v8i16"]
    pub fn convertuu_i64_v8i16(a: ::simdty::i16x8, b: i32, c: i32) -> i64;
    /// The `llvm.convertuu.i64.i16` intrinsic.
    #[link_name = "llvm.convertuu.i64.i16"]
    pub fn convertuu_i64_i16(a: i16, b: i32, c: i32) -> i64;
    /// The `llvm.convertuu.i64.v4i32` intrinsic.
    #[link_name = "llvm.convertuu.i64.v4i32"]
    pub fn convertuu_i64_v4i32(a: ::simdty::i32x4, b: i32, c: i32) -> i64;
    /// The `llvm.convertuu.i64.i32` intrinsic.
    #[link_name = "llvm.convertuu.i64.i32"]
    pub fn convertuu_i64_i32(a: i32, b: i32, c: i32) -> i64;
    /// The `llvm.convertuu.i64.v2i64` intrinsic.
    #[link_name = "llvm.convertuu.i64.v2i64"]
    pub fn convertuu_i64_v2i64(a: ::simdty::i64x2, b: i32, c: i32) -> i64;
    /// The `llvm.convertuu.i64.i64` intrinsic.
    #[link_name = "llvm.convertuu.i64.i64"]
    pub fn convertuu_i64_i64(a: i64, b: i32, c: i32) -> i64;
    /// The `llvm.clear_cache` intrinsic.
    #[link_name = "llvm.clear_cache"]
    pub fn clear_cache(a: *mut i8, b: *mut i8) -> ();
}
/// LLVM intrinsics for the x86 architecture.
pub mod x86 {
    extern {
        /// The `llvm.x86.int` intrinsic.
        #[link_name = "llvm.x86.int"]
        pub fn int(a: i8) -> ();
        /// The `llvm.x86.rdtsc` intrinsic; known as `__builtin_ia32_rdtsc` in GCC.
        #[link_name = "llvm.x86.rdtsc"]
        pub fn rdtsc() -> i64;
        /// The `llvm.x86.rdtscp` intrinsic; known as `__builtin_ia32_rdtscp` in GCC.
        #[link_name = "llvm.x86.rdtscp"]
        pub fn rdtscp(a: *mut i8) -> i64;
        /// The `llvm.x86.rdpmc` intrinsic; known as `__builtin_ia32_rdpmc` in GCC.
        #[link_name = "llvm.x86.rdpmc"]
        pub fn rdpmc(a: i32) -> i64;
        /// The `llvm.x86.sse.add.ss` intrinsic; known as `__builtin_ia32_addss` in GCC.
        #[link_name = "llvm.x86.sse.add.ss"]
        pub fn sse_add_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.sse.sub.ss` intrinsic; known as `__builtin_ia32_subss` in GCC.
        #[link_name = "llvm.x86.sse.sub.ss"]
        pub fn sse_sub_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.sse.mul.ss` intrinsic; known as `__builtin_ia32_mulss` in GCC.
        #[link_name = "llvm.x86.sse.mul.ss"]
        pub fn sse_mul_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.sse.div.ss` intrinsic; known as `__builtin_ia32_divss` in GCC.
        #[link_name = "llvm.x86.sse.div.ss"]
        pub fn sse_div_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.sse.sqrt.ss` intrinsic; known as `__builtin_ia32_sqrtss` in GCC.
        #[link_name = "llvm.x86.sse.sqrt.ss"]
        pub fn sse_sqrt_ss(a: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.sse.sqrt.ps` intrinsic; known as `__builtin_ia32_sqrtps` in GCC.
        #[link_name = "llvm.x86.sse.sqrt.ps"]
        pub fn sse_sqrt_ps(a: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.sse.rcp.ss` intrinsic; known as `__builtin_ia32_rcpss` in GCC.
        #[link_name = "llvm.x86.sse.rcp.ss"]
        pub fn sse_rcp_ss(a: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.sse.rcp.ps` intrinsic; known as `__builtin_ia32_rcpps` in GCC.
        #[link_name = "llvm.x86.sse.rcp.ps"]
        pub fn sse_rcp_ps(a: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.sse.rsqrt.ss` intrinsic; known as `__builtin_ia32_rsqrtss` in GCC.
        #[link_name = "llvm.x86.sse.rsqrt.ss"]
        pub fn sse_rsqrt_ss(a: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.sse.rsqrt.ps` intrinsic; known as `__builtin_ia32_rsqrtps` in GCC.
        #[link_name = "llvm.x86.sse.rsqrt.ps"]
        pub fn sse_rsqrt_ps(a: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.sse.min.ss` intrinsic; known as `__builtin_ia32_minss` in GCC.
        #[link_name = "llvm.x86.sse.min.ss"]
        pub fn sse_min_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.sse.min.ps` intrinsic; known as `__builtin_ia32_minps` in GCC.
        #[link_name = "llvm.x86.sse.min.ps"]
        pub fn sse_min_ps(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.sse.max.ss` intrinsic; known as `__builtin_ia32_maxss` in GCC.
        #[link_name = "llvm.x86.sse.max.ss"]
        pub fn sse_max_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.sse.max.ps` intrinsic; known as `__builtin_ia32_maxps` in GCC.
        #[link_name = "llvm.x86.sse.max.ps"]
        pub fn sse_max_ps(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.sse.cmp.ss` intrinsic; known as `__builtin_ia32_cmpss` in GCC.
        #[link_name = "llvm.x86.sse.cmp.ss"]
        pub fn sse_cmp_ss(a: ::simdty::f32x4, b: ::simdty::f32x4, c: i8) -> ::simdty::f32x4;
        /// The `llvm.x86.sse.cmp.ps` intrinsic; known as `__builtin_ia32_cmpps` in GCC.
        #[link_name = "llvm.x86.sse.cmp.ps"]
        pub fn sse_cmp_ps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: i8) -> ::simdty::f32x4;
        /// The `llvm.x86.sse.comieq.ss` intrinsic; known as `__builtin_ia32_comieq` in GCC.
        #[link_name = "llvm.x86.sse.comieq.ss"]
        pub fn sse_comieq_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        /// The `llvm.x86.sse.comilt.ss` intrinsic; known as `__builtin_ia32_comilt` in GCC.
        #[link_name = "llvm.x86.sse.comilt.ss"]
        pub fn sse_comilt_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        /// The `llvm.x86.sse.comile.ss` intrinsic; known as `__builtin_ia32_comile` in GCC.
        #[link_name = "llvm.x86.sse.comile.ss"]
        pub fn sse_comile_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        /// The `llvm.x86.sse.comigt.ss` intrinsic; known as `__builtin_ia32_comigt` in GCC.
        #[link_name = "llvm.x86.sse.comigt.ss"]
        pub fn sse_comigt_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        /// The `llvm.x86.sse.comige.ss` intrinsic; known as `__builtin_ia32_comige` in GCC.
        #[link_name = "llvm.x86.sse.comige.ss"]
        pub fn sse_comige_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        /// The `llvm.x86.sse.comineq.ss` intrinsic; known as `__builtin_ia32_comineq` in GCC.
        #[link_name = "llvm.x86.sse.comineq.ss"]
        pub fn sse_comineq_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        /// The `llvm.x86.sse.ucomieq.ss` intrinsic; known as `__builtin_ia32_ucomieq` in GCC.
        #[link_name = "llvm.x86.sse.ucomieq.ss"]
        pub fn sse_ucomieq_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        /// The `llvm.x86.sse.ucomilt.ss` intrinsic; known as `__builtin_ia32_ucomilt` in GCC.
        #[link_name = "llvm.x86.sse.ucomilt.ss"]
        pub fn sse_ucomilt_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        /// The `llvm.x86.sse.ucomile.ss` intrinsic; known as `__builtin_ia32_ucomile` in GCC.
        #[link_name = "llvm.x86.sse.ucomile.ss"]
        pub fn sse_ucomile_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        /// The `llvm.x86.sse.ucomigt.ss` intrinsic; known as `__builtin_ia32_ucomigt` in GCC.
        #[link_name = "llvm.x86.sse.ucomigt.ss"]
        pub fn sse_ucomigt_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        /// The `llvm.x86.sse.ucomige.ss` intrinsic; known as `__builtin_ia32_ucomige` in GCC.
        #[link_name = "llvm.x86.sse.ucomige.ss"]
        pub fn sse_ucomige_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        /// The `llvm.x86.sse.ucomineq.ss` intrinsic; known as `__builtin_ia32_ucomineq` in GCC.
        #[link_name = "llvm.x86.sse.ucomineq.ss"]
        pub fn sse_ucomineq_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        /// The `llvm.x86.sse.cvtss2si` intrinsic; known as `__builtin_ia32_cvtss2si` in GCC.
        #[link_name = "llvm.x86.sse.cvtss2si"]
        pub fn sse_cvtss2si(a: ::simdty::f32x4) -> i32;
        /// The `llvm.x86.sse.cvtss2si64` intrinsic; known as `__builtin_ia32_cvtss2si64` in GCC.
        #[link_name = "llvm.x86.sse.cvtss2si64"]
        pub fn sse_cvtss2si64(a: ::simdty::f32x4) -> i64;
        /// The `llvm.x86.sse.cvttss2si` intrinsic; known as `__builtin_ia32_cvttss2si` in GCC.
        #[link_name = "llvm.x86.sse.cvttss2si"]
        pub fn sse_cvttss2si(a: ::simdty::f32x4) -> i32;
        /// The `llvm.x86.sse.cvttss2si64` intrinsic; known as `__builtin_ia32_cvttss2si64` in GCC.
        #[link_name = "llvm.x86.sse.cvttss2si64"]
        pub fn sse_cvttss2si64(a: ::simdty::f32x4) -> i64;
        /// The `llvm.x86.sse.cvtsi2ss` intrinsic; known as `__builtin_ia32_cvtsi2ss` in GCC.
        #[link_name = "llvm.x86.sse.cvtsi2ss"]
        pub fn sse_cvtsi2ss(a: ::simdty::f32x4, b: i32) -> ::simdty::f32x4;
        /// The `llvm.x86.sse.cvtsi642ss` intrinsic; known as `__builtin_ia32_cvtsi642ss` in GCC.
        #[link_name = "llvm.x86.sse.cvtsi642ss"]
        pub fn sse_cvtsi642ss(a: ::simdty::f32x4, b: i64) -> ::simdty::f32x4;
        /// The `llvm.x86.sse.storeu.ps` intrinsic; known as `__builtin_ia32_storeups` in GCC.
        #[link_name = "llvm.x86.sse.storeu.ps"]
        pub fn sse_storeu_ps(a: *mut i8, b: ::simdty::f32x4) -> ();
        /// The `llvm.x86.sse.sfence` intrinsic; known as `__builtin_ia32_sfence` in GCC.
        #[link_name = "llvm.x86.sse.sfence"]
        pub fn sse_sfence() -> ();
        /// The `llvm.x86.sse.stmxcsr` intrinsic.
        #[link_name = "llvm.x86.sse.stmxcsr"]
        pub fn sse_stmxcsr(a: *mut i8) -> ();
        /// The `llvm.x86.sse.ldmxcsr` intrinsic.
        #[link_name = "llvm.x86.sse.ldmxcsr"]
        pub fn sse_ldmxcsr(a: *mut i8) -> ();
        /// The `llvm.x86.sse.movmsk.ps` intrinsic; known as `__builtin_ia32_movmskps` in GCC.
        #[link_name = "llvm.x86.sse.movmsk.ps"]
        pub fn sse_movmsk_ps(a: ::simdty::f32x4) -> i32;
        /// The `llvm.x86.sse2.add.sd` intrinsic; known as `__builtin_ia32_addsd` in GCC.
        #[link_name = "llvm.x86.sse2.add.sd"]
        pub fn sse2_add_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.sse2.sub.sd` intrinsic; known as `__builtin_ia32_subsd` in GCC.
        #[link_name = "llvm.x86.sse2.sub.sd"]
        pub fn sse2_sub_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.sse2.mul.sd` intrinsic; known as `__builtin_ia32_mulsd` in GCC.
        #[link_name = "llvm.x86.sse2.mul.sd"]
        pub fn sse2_mul_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.sse2.div.sd` intrinsic; known as `__builtin_ia32_divsd` in GCC.
        #[link_name = "llvm.x86.sse2.div.sd"]
        pub fn sse2_div_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.sse2.sqrt.sd` intrinsic; known as `__builtin_ia32_sqrtsd` in GCC.
        #[link_name = "llvm.x86.sse2.sqrt.sd"]
        pub fn sse2_sqrt_sd(a: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.sse2.sqrt.pd` intrinsic; known as `__builtin_ia32_sqrtpd` in GCC.
        #[link_name = "llvm.x86.sse2.sqrt.pd"]
        pub fn sse2_sqrt_pd(a: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.sse2.min.sd` intrinsic; known as `__builtin_ia32_minsd` in GCC.
        #[link_name = "llvm.x86.sse2.min.sd"]
        pub fn sse2_min_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.sse2.min.pd` intrinsic; known as `__builtin_ia32_minpd` in GCC.
        #[link_name = "llvm.x86.sse2.min.pd"]
        pub fn sse2_min_pd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.sse2.max.sd` intrinsic; known as `__builtin_ia32_maxsd` in GCC.
        #[link_name = "llvm.x86.sse2.max.sd"]
        pub fn sse2_max_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.sse2.max.pd` intrinsic; known as `__builtin_ia32_maxpd` in GCC.
        #[link_name = "llvm.x86.sse2.max.pd"]
        pub fn sse2_max_pd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.sse2.cmp.sd` intrinsic; known as `__builtin_ia32_cmpsd` in GCC.
        #[link_name = "llvm.x86.sse2.cmp.sd"]
        pub fn sse2_cmp_sd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: i8) -> ::simdty::f64x2;
        /// The `llvm.x86.sse2.cmp.pd` intrinsic; known as `__builtin_ia32_cmppd` in GCC.
        #[link_name = "llvm.x86.sse2.cmp.pd"]
        pub fn sse2_cmp_pd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: i8) -> ::simdty::f64x2;
        /// The `llvm.x86.sse2.comieq.sd` intrinsic; known as `__builtin_ia32_comisdeq` in GCC.
        #[link_name = "llvm.x86.sse2.comieq.sd"]
        pub fn sse2_comieq_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        /// The `llvm.x86.sse2.comilt.sd` intrinsic; known as `__builtin_ia32_comisdlt` in GCC.
        #[link_name = "llvm.x86.sse2.comilt.sd"]
        pub fn sse2_comilt_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        /// The `llvm.x86.sse2.comile.sd` intrinsic; known as `__builtin_ia32_comisdle` in GCC.
        #[link_name = "llvm.x86.sse2.comile.sd"]
        pub fn sse2_comile_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        /// The `llvm.x86.sse2.comigt.sd` intrinsic; known as `__builtin_ia32_comisdgt` in GCC.
        #[link_name = "llvm.x86.sse2.comigt.sd"]
        pub fn sse2_comigt_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        /// The `llvm.x86.sse2.comige.sd` intrinsic; known as `__builtin_ia32_comisdge` in GCC.
        #[link_name = "llvm.x86.sse2.comige.sd"]
        pub fn sse2_comige_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        /// The `llvm.x86.sse2.comineq.sd` intrinsic; known as `__builtin_ia32_comisdneq` in GCC.
        #[link_name = "llvm.x86.sse2.comineq.sd"]
        pub fn sse2_comineq_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        /// The `llvm.x86.sse2.ucomieq.sd` intrinsic; known as `__builtin_ia32_ucomisdeq` in GCC.
        #[link_name = "llvm.x86.sse2.ucomieq.sd"]
        pub fn sse2_ucomieq_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        /// The `llvm.x86.sse2.ucomilt.sd` intrinsic; known as `__builtin_ia32_ucomisdlt` in GCC.
        #[link_name = "llvm.x86.sse2.ucomilt.sd"]
        pub fn sse2_ucomilt_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        /// The `llvm.x86.sse2.ucomile.sd` intrinsic; known as `__builtin_ia32_ucomisdle` in GCC.
        #[link_name = "llvm.x86.sse2.ucomile.sd"]
        pub fn sse2_ucomile_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        /// The `llvm.x86.sse2.ucomigt.sd` intrinsic; known as `__builtin_ia32_ucomisdgt` in GCC.
        #[link_name = "llvm.x86.sse2.ucomigt.sd"]
        pub fn sse2_ucomigt_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        /// The `llvm.x86.sse2.ucomige.sd` intrinsic; known as `__builtin_ia32_ucomisdge` in GCC.
        #[link_name = "llvm.x86.sse2.ucomige.sd"]
        pub fn sse2_ucomige_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        /// The `llvm.x86.sse2.ucomineq.sd` intrinsic; known as `__builtin_ia32_ucomisdneq` in GCC.
        #[link_name = "llvm.x86.sse2.ucomineq.sd"]
        pub fn sse2_ucomineq_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        /// The `llvm.x86.sse2.padds.b` intrinsic; known as `__builtin_ia32_paddsb128` in GCC.
        #[link_name = "llvm.x86.sse2.padds.b"]
        pub fn sse2_padds_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        /// The `llvm.x86.sse2.padds.w` intrinsic; known as `__builtin_ia32_paddsw128` in GCC.
        #[link_name = "llvm.x86.sse2.padds.w"]
        pub fn sse2_padds_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.sse2.paddus.b` intrinsic; known as `__builtin_ia32_paddusb128` in GCC.
        #[link_name = "llvm.x86.sse2.paddus.b"]
        pub fn sse2_paddus_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        /// The `llvm.x86.sse2.paddus.w` intrinsic; known as `__builtin_ia32_paddusw128` in GCC.
        #[link_name = "llvm.x86.sse2.paddus.w"]
        pub fn sse2_paddus_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.sse2.psubs.b` intrinsic; known as `__builtin_ia32_psubsb128` in GCC.
        #[link_name = "llvm.x86.sse2.psubs.b"]
        pub fn sse2_psubs_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        /// The `llvm.x86.sse2.psubs.w` intrinsic; known as `__builtin_ia32_psubsw128` in GCC.
        #[link_name = "llvm.x86.sse2.psubs.w"]
        pub fn sse2_psubs_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.sse2.psubus.b` intrinsic; known as `__builtin_ia32_psubusb128` in GCC.
        #[link_name = "llvm.x86.sse2.psubus.b"]
        pub fn sse2_psubus_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        /// The `llvm.x86.sse2.psubus.w` intrinsic; known as `__builtin_ia32_psubusw128` in GCC.
        #[link_name = "llvm.x86.sse2.psubus.w"]
        pub fn sse2_psubus_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.sse2.pmulhu.w` intrinsic; known as `__builtin_ia32_pmulhuw128` in GCC.
        #[link_name = "llvm.x86.sse2.pmulhu.w"]
        pub fn sse2_pmulhu_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.sse2.pmulh.w` intrinsic; known as `__builtin_ia32_pmulhw128` in GCC.
        #[link_name = "llvm.x86.sse2.pmulh.w"]
        pub fn sse2_pmulh_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.sse2.pmulu.dq` intrinsic; known as `__builtin_ia32_pmuludq128` in GCC.
        #[link_name = "llvm.x86.sse2.pmulu.dq"]
        pub fn sse2_pmulu_dq(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i64x2;
        /// The `llvm.x86.sse2.pmadd.wd` intrinsic; known as `__builtin_ia32_pmaddwd128` in GCC.
        #[link_name = "llvm.x86.sse2.pmadd.wd"]
        pub fn sse2_pmadd_wd(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i32x4;
        /// The `llvm.x86.sse2.pavg.b` intrinsic; known as `__builtin_ia32_pavgb128` in GCC.
        #[link_name = "llvm.x86.sse2.pavg.b"]
        pub fn sse2_pavg_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        /// The `llvm.x86.sse2.pavg.w` intrinsic; known as `__builtin_ia32_pavgw128` in GCC.
        #[link_name = "llvm.x86.sse2.pavg.w"]
        pub fn sse2_pavg_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.sse2.pmaxu.b` intrinsic; known as `__builtin_ia32_pmaxub128` in GCC.
        #[link_name = "llvm.x86.sse2.pmaxu.b"]
        pub fn sse2_pmaxu_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        /// The `llvm.x86.sse2.pmaxs.w` intrinsic; known as `__builtin_ia32_pmaxsw128` in GCC.
        #[link_name = "llvm.x86.sse2.pmaxs.w"]
        pub fn sse2_pmaxs_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.sse2.pminu.b` intrinsic; known as `__builtin_ia32_pminub128` in GCC.
        #[link_name = "llvm.x86.sse2.pminu.b"]
        pub fn sse2_pminu_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        /// The `llvm.x86.sse2.pmins.w` intrinsic; known as `__builtin_ia32_pminsw128` in GCC.
        #[link_name = "llvm.x86.sse2.pmins.w"]
        pub fn sse2_pmins_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.sse2.psad.bw` intrinsic; known as `__builtin_ia32_psadbw128` in GCC.
        #[link_name = "llvm.x86.sse2.psad.bw"]
        pub fn sse2_psad_bw(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i64x2;
        /// The `llvm.x86.sse2.psll.w` intrinsic; known as `__builtin_ia32_psllw128` in GCC.
        #[link_name = "llvm.x86.sse2.psll.w"]
        pub fn sse2_psll_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.sse2.psll.d` intrinsic; known as `__builtin_ia32_pslld128` in GCC.
        #[link_name = "llvm.x86.sse2.psll.d"]
        pub fn sse2_psll_d(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.sse2.psll.q` intrinsic; known as `__builtin_ia32_psllq128` in GCC.
        #[link_name = "llvm.x86.sse2.psll.q"]
        pub fn sse2_psll_q(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        /// The `llvm.x86.sse2.psrl.w` intrinsic; known as `__builtin_ia32_psrlw128` in GCC.
        #[link_name = "llvm.x86.sse2.psrl.w"]
        pub fn sse2_psrl_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.sse2.psrl.d` intrinsic; known as `__builtin_ia32_psrld128` in GCC.
        #[link_name = "llvm.x86.sse2.psrl.d"]
        pub fn sse2_psrl_d(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.sse2.psrl.q` intrinsic; known as `__builtin_ia32_psrlq128` in GCC.
        #[link_name = "llvm.x86.sse2.psrl.q"]
        pub fn sse2_psrl_q(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        /// The `llvm.x86.sse2.psra.w` intrinsic; known as `__builtin_ia32_psraw128` in GCC.
        #[link_name = "llvm.x86.sse2.psra.w"]
        pub fn sse2_psra_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.sse2.psra.d` intrinsic; known as `__builtin_ia32_psrad128` in GCC.
        #[link_name = "llvm.x86.sse2.psra.d"]
        pub fn sse2_psra_d(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.sse2.pslli.w` intrinsic; known as `__builtin_ia32_psllwi128` in GCC.
        #[link_name = "llvm.x86.sse2.pslli.w"]
        pub fn sse2_pslli_w(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        /// The `llvm.x86.sse2.pslli.d` intrinsic; known as `__builtin_ia32_pslldi128` in GCC.
        #[link_name = "llvm.x86.sse2.pslli.d"]
        pub fn sse2_pslli_d(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        /// The `llvm.x86.sse2.pslli.q` intrinsic; known as `__builtin_ia32_psllqi128` in GCC.
        #[link_name = "llvm.x86.sse2.pslli.q"]
        pub fn sse2_pslli_q(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        /// The `llvm.x86.sse2.psrli.w` intrinsic; known as `__builtin_ia32_psrlwi128` in GCC.
        #[link_name = "llvm.x86.sse2.psrli.w"]
        pub fn sse2_psrli_w(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        /// The `llvm.x86.sse2.psrli.d` intrinsic; known as `__builtin_ia32_psrldi128` in GCC.
        #[link_name = "llvm.x86.sse2.psrli.d"]
        pub fn sse2_psrli_d(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        /// The `llvm.x86.sse2.psrli.q` intrinsic; known as `__builtin_ia32_psrlqi128` in GCC.
        #[link_name = "llvm.x86.sse2.psrli.q"]
        pub fn sse2_psrli_q(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        /// The `llvm.x86.sse2.psrai.w` intrinsic; known as `__builtin_ia32_psrawi128` in GCC.
        #[link_name = "llvm.x86.sse2.psrai.w"]
        pub fn sse2_psrai_w(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        /// The `llvm.x86.sse2.psrai.d` intrinsic; known as `__builtin_ia32_psradi128` in GCC.
        #[link_name = "llvm.x86.sse2.psrai.d"]
        pub fn sse2_psrai_d(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        /// The `llvm.x86.sse2.psll.dq` intrinsic; known as `__builtin_ia32_pslldqi128` in GCC.
        #[link_name = "llvm.x86.sse2.psll.dq"]
        pub fn sse2_psll_dq(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        /// The `llvm.x86.sse2.psrl.dq` intrinsic; known as `__builtin_ia32_psrldqi128` in GCC.
        #[link_name = "llvm.x86.sse2.psrl.dq"]
        pub fn sse2_psrl_dq(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        /// The `llvm.x86.sse2.psll.dq.bs` intrinsic; known as `__builtin_ia32_pslldqi128_byteshift` in GCC.
        #[link_name = "llvm.x86.sse2.psll.dq.bs"]
        pub fn sse2_psll_dq_bs(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        /// The `llvm.x86.sse2.psrl.dq.bs` intrinsic; known as `__builtin_ia32_psrldqi128_byteshift` in GCC.
        #[link_name = "llvm.x86.sse2.psrl.dq.bs"]
        pub fn sse2_psrl_dq_bs(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        /// The `llvm.x86.sse2.cvtdq2pd` intrinsic; known as `__builtin_ia32_cvtdq2pd` in GCC.
        #[link_name = "llvm.x86.sse2.cvtdq2pd"]
        pub fn sse2_cvtdq2pd(a: ::simdty::i32x4) -> ::simdty::f64x2;
        /// The `llvm.x86.sse2.cvtdq2ps` intrinsic; known as `__builtin_ia32_cvtdq2ps` in GCC.
        #[link_name = "llvm.x86.sse2.cvtdq2ps"]
        pub fn sse2_cvtdq2ps(a: ::simdty::i32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.sse2.cvtpd2dq` intrinsic; known as `__builtin_ia32_cvtpd2dq` in GCC.
        #[link_name = "llvm.x86.sse2.cvtpd2dq"]
        pub fn sse2_cvtpd2dq(a: ::simdty::f64x2) -> ::simdty::i32x4;
        /// The `llvm.x86.sse2.cvttpd2dq` intrinsic; known as `__builtin_ia32_cvttpd2dq` in GCC.
        #[link_name = "llvm.x86.sse2.cvttpd2dq"]
        pub fn sse2_cvttpd2dq(a: ::simdty::f64x2) -> ::simdty::i32x4;
        /// The `llvm.x86.sse2.cvtpd2ps` intrinsic; known as `__builtin_ia32_cvtpd2ps` in GCC.
        #[link_name = "llvm.x86.sse2.cvtpd2ps"]
        pub fn sse2_cvtpd2ps(a: ::simdty::f64x2) -> ::simdty::f32x4;
        /// The `llvm.x86.sse2.cvtps2dq` intrinsic; known as `__builtin_ia32_cvtps2dq` in GCC.
        #[link_name = "llvm.x86.sse2.cvtps2dq"]
        pub fn sse2_cvtps2dq(a: ::simdty::f32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.sse2.cvttps2dq` intrinsic; known as `__builtin_ia32_cvttps2dq` in GCC.
        #[link_name = "llvm.x86.sse2.cvttps2dq"]
        pub fn sse2_cvttps2dq(a: ::simdty::f32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.sse2.cvtps2pd` intrinsic; known as `__builtin_ia32_cvtps2pd` in GCC.
        #[link_name = "llvm.x86.sse2.cvtps2pd"]
        pub fn sse2_cvtps2pd(a: ::simdty::f32x4) -> ::simdty::f64x2;
        /// The `llvm.x86.sse2.cvtsd2si` intrinsic; known as `__builtin_ia32_cvtsd2si` in GCC.
        #[link_name = "llvm.x86.sse2.cvtsd2si"]
        pub fn sse2_cvtsd2si(a: ::simdty::f64x2) -> i32;
        /// The `llvm.x86.sse2.cvtsd2si64` intrinsic; known as `__builtin_ia32_cvtsd2si64` in GCC.
        #[link_name = "llvm.x86.sse2.cvtsd2si64"]
        pub fn sse2_cvtsd2si64(a: ::simdty::f64x2) -> i64;
        /// The `llvm.x86.sse2.cvttsd2si` intrinsic; known as `__builtin_ia32_cvttsd2si` in GCC.
        #[link_name = "llvm.x86.sse2.cvttsd2si"]
        pub fn sse2_cvttsd2si(a: ::simdty::f64x2) -> i32;
        /// The `llvm.x86.sse2.cvttsd2si64` intrinsic; known as `__builtin_ia32_cvttsd2si64` in GCC.
        #[link_name = "llvm.x86.sse2.cvttsd2si64"]
        pub fn sse2_cvttsd2si64(a: ::simdty::f64x2) -> i64;
        /// The `llvm.x86.sse2.cvtsi2sd` intrinsic; known as `__builtin_ia32_cvtsi2sd` in GCC.
        #[link_name = "llvm.x86.sse2.cvtsi2sd"]
        pub fn sse2_cvtsi2sd(a: ::simdty::f64x2, b: i32) -> ::simdty::f64x2;
        /// The `llvm.x86.sse2.cvtsi642sd` intrinsic; known as `__builtin_ia32_cvtsi642sd` in GCC.
        #[link_name = "llvm.x86.sse2.cvtsi642sd"]
        pub fn sse2_cvtsi642sd(a: ::simdty::f64x2, b: i64) -> ::simdty::f64x2;
        /// The `llvm.x86.sse2.cvtsd2ss` intrinsic; known as `__builtin_ia32_cvtsd2ss` in GCC.
        #[link_name = "llvm.x86.sse2.cvtsd2ss"]
        pub fn sse2_cvtsd2ss(a: ::simdty::f32x4, b: ::simdty::f64x2) -> ::simdty::f32x4;
        /// The `llvm.x86.sse2.cvtss2sd` intrinsic; known as `__builtin_ia32_cvtss2sd` in GCC.
        #[link_name = "llvm.x86.sse2.cvtss2sd"]
        pub fn sse2_cvtss2sd(a: ::simdty::f64x2, b: ::simdty::f32x4) -> ::simdty::f64x2;
        /// The `llvm.x86.sse2.storeu.pd` intrinsic; known as `__builtin_ia32_storeupd` in GCC.
        #[link_name = "llvm.x86.sse2.storeu.pd"]
        pub fn sse2_storeu_pd(a: *mut i8, b: ::simdty::f64x2) -> ();
        /// The `llvm.x86.sse2.storeu.dq` intrinsic; known as `__builtin_ia32_storedqu` in GCC.
        #[link_name = "llvm.x86.sse2.storeu.dq"]
        pub fn sse2_storeu_dq(a: *mut i8, b: ::simdty::i8x16) -> ();
        /// The `llvm.x86.sse2.storel.dq` intrinsic; known as `__builtin_ia32_storelv4si` in GCC.
        #[link_name = "llvm.x86.sse2.storel.dq"]
        pub fn sse2_storel_dq(a: *mut i8, b: ::simdty::i32x4) -> ();
        /// The `llvm.x86.sse2.packsswb.128` intrinsic; known as `__builtin_ia32_packsswb128` in GCC.
        #[link_name = "llvm.x86.sse2.packsswb.128"]
        pub fn sse2_packsswb_128(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i8x16;
        /// The `llvm.x86.sse2.packssdw.128` intrinsic; known as `__builtin_ia32_packssdw128` in GCC.
        #[link_name = "llvm.x86.sse2.packssdw.128"]
        pub fn sse2_packssdw_128(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i16x8;
        /// The `llvm.x86.sse2.packuswb.128` intrinsic; known as `__builtin_ia32_packuswb128` in GCC.
        #[link_name = "llvm.x86.sse2.packuswb.128"]
        pub fn sse2_packuswb_128(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i8x16;
        /// The `llvm.x86.sse2.movmsk.pd` intrinsic; known as `__builtin_ia32_movmskpd` in GCC.
        #[link_name = "llvm.x86.sse2.movmsk.pd"]
        pub fn sse2_movmsk_pd(a: ::simdty::f64x2) -> i32;
        /// The `llvm.x86.sse2.pmovmskb.128` intrinsic; known as `__builtin_ia32_pmovmskb128` in GCC.
        #[link_name = "llvm.x86.sse2.pmovmskb.128"]
        pub fn sse2_pmovmskb_128(a: ::simdty::i8x16) -> i32;
        /// The `llvm.x86.sse2.maskmov.dqu` intrinsic; known as `__builtin_ia32_maskmovdqu` in GCC.
        #[link_name = "llvm.x86.sse2.maskmov.dqu"]
        pub fn sse2_maskmov_dqu(a: ::simdty::i8x16, b: ::simdty::i8x16, c: *mut i8) -> ();
        /// The `llvm.x86.sse2.clflush` intrinsic; known as `__builtin_ia32_clflush` in GCC.
        #[link_name = "llvm.x86.sse2.clflush"]
        pub fn sse2_clflush(a: *mut i8) -> ();
        /// The `llvm.x86.sse2.lfence` intrinsic; known as `__builtin_ia32_lfence` in GCC.
        #[link_name = "llvm.x86.sse2.lfence"]
        pub fn sse2_lfence() -> ();
        /// The `llvm.x86.sse2.mfence` intrinsic; known as `__builtin_ia32_mfence` in GCC.
        #[link_name = "llvm.x86.sse2.mfence"]
        pub fn sse2_mfence() -> ();
        /// The `llvm.x86.sse2.pause` intrinsic; known as `__builtin_ia32_pause` in GCC.
        #[link_name = "llvm.x86.sse2.pause"]
        pub fn sse2_pause() -> ();
        /// The `llvm.x86.sse3.addsub.ps` intrinsic; known as `__builtin_ia32_addsubps` in GCC.
        #[link_name = "llvm.x86.sse3.addsub.ps"]
        pub fn sse3_addsub_ps(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.sse3.addsub.pd` intrinsic; known as `__builtin_ia32_addsubpd` in GCC.
        #[link_name = "llvm.x86.sse3.addsub.pd"]
        pub fn sse3_addsub_pd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.sse3.hadd.ps` intrinsic; known as `__builtin_ia32_haddps` in GCC.
        #[link_name = "llvm.x86.sse3.hadd.ps"]
        pub fn sse3_hadd_ps(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.sse3.hadd.pd` intrinsic; known as `__builtin_ia32_haddpd` in GCC.
        #[link_name = "llvm.x86.sse3.hadd.pd"]
        pub fn sse3_hadd_pd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.sse3.hsub.ps` intrinsic; known as `__builtin_ia32_hsubps` in GCC.
        #[link_name = "llvm.x86.sse3.hsub.ps"]
        pub fn sse3_hsub_ps(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.sse3.hsub.pd` intrinsic; known as `__builtin_ia32_hsubpd` in GCC.
        #[link_name = "llvm.x86.sse3.hsub.pd"]
        pub fn sse3_hsub_pd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.sse3.ldu.dq` intrinsic; known as `__builtin_ia32_lddqu` in GCC.
        #[link_name = "llvm.x86.sse3.ldu.dq"]
        pub fn sse3_ldu_dq(a: *mut i8) -> ::simdty::i8x16;
        /// The `llvm.x86.sse3.monitor` intrinsic; known as `__builtin_ia32_monitor` in GCC.
        #[link_name = "llvm.x86.sse3.monitor"]
        pub fn sse3_monitor(a: *mut i8, b: i32, c: i32) -> ();
        /// The `llvm.x86.sse3.mwait` intrinsic; known as `__builtin_ia32_mwait` in GCC.
        #[link_name = "llvm.x86.sse3.mwait"]
        pub fn sse3_mwait(a: i32, b: i32) -> ();
        /// The `llvm.x86.ssse3.phadd.w.128` intrinsic; known as `__builtin_ia32_phaddw128` in GCC.
        #[link_name = "llvm.x86.ssse3.phadd.w.128"]
        pub fn ssse3_phadd_w_128(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.ssse3.phadd.d.128` intrinsic; known as `__builtin_ia32_phaddd128` in GCC.
        #[link_name = "llvm.x86.ssse3.phadd.d.128"]
        pub fn ssse3_phadd_d_128(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.ssse3.phadd.sw.128` intrinsic; known as `__builtin_ia32_phaddsw128` in GCC.
        #[link_name = "llvm.x86.ssse3.phadd.sw.128"]
        pub fn ssse3_phadd_sw_128(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.ssse3.phsub.w.128` intrinsic; known as `__builtin_ia32_phsubw128` in GCC.
        #[link_name = "llvm.x86.ssse3.phsub.w.128"]
        pub fn ssse3_phsub_w_128(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.ssse3.phsub.d.128` intrinsic; known as `__builtin_ia32_phsubd128` in GCC.
        #[link_name = "llvm.x86.ssse3.phsub.d.128"]
        pub fn ssse3_phsub_d_128(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.ssse3.phsub.sw.128` intrinsic; known as `__builtin_ia32_phsubsw128` in GCC.
        #[link_name = "llvm.x86.ssse3.phsub.sw.128"]
        pub fn ssse3_phsub_sw_128(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.ssse3.pmadd.ub.sw.128` intrinsic; known as `__builtin_ia32_pmaddubsw128` in GCC.
        #[link_name = "llvm.x86.ssse3.pmadd.ub.sw.128"]
        pub fn ssse3_pmadd_ub_sw_128(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i16x8;
        /// The `llvm.x86.ssse3.pmul.hr.sw.128` intrinsic; known as `__builtin_ia32_pmulhrsw128` in GCC.
        #[link_name = "llvm.x86.ssse3.pmul.hr.sw.128"]
        pub fn ssse3_pmul_hr_sw_128(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.ssse3.pshuf.b.128` intrinsic; known as `__builtin_ia32_pshufb128` in GCC.
        #[link_name = "llvm.x86.ssse3.pshuf.b.128"]
        pub fn ssse3_pshuf_b_128(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        /// The `llvm.x86.sse2.pshuf.d` intrinsic; known as `__builtin_ia32_pshufd` in GCC.
        #[link_name = "llvm.x86.sse2.pshuf.d"]
        pub fn sse2_pshuf_d(a: ::simdty::i32x4, b: i8) -> ::simdty::i32x4;
        /// The `llvm.x86.sse2.pshufl.w` intrinsic; known as `__builtin_ia32_pshuflw` in GCC.
        #[link_name = "llvm.x86.sse2.pshufl.w"]
        pub fn sse2_pshufl_w(a: ::simdty::i16x8, b: i8) -> ::simdty::i16x8;
        /// The `llvm.x86.sse2.pshufh.w` intrinsic; known as `__builtin_ia32_pshufhw` in GCC.
        #[link_name = "llvm.x86.sse2.pshufh.w"]
        pub fn sse2_pshufh_w(a: ::simdty::i16x8, b: i8) -> ::simdty::i16x8;
        /// The `llvm.x86.ssse3.psign.b.128` intrinsic; known as `__builtin_ia32_psignb128` in GCC.
        #[link_name = "llvm.x86.ssse3.psign.b.128"]
        pub fn ssse3_psign_b_128(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        /// The `llvm.x86.ssse3.psign.w.128` intrinsic; known as `__builtin_ia32_psignw128` in GCC.
        #[link_name = "llvm.x86.ssse3.psign.w.128"]
        pub fn ssse3_psign_w_128(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.ssse3.psign.d.128` intrinsic; known as `__builtin_ia32_psignd128` in GCC.
        #[link_name = "llvm.x86.ssse3.psign.d.128"]
        pub fn ssse3_psign_d_128(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.ssse3.pabs.b.128` intrinsic; known as `__builtin_ia32_pabsb128` in GCC.
        #[link_name = "llvm.x86.ssse3.pabs.b.128"]
        pub fn ssse3_pabs_b_128(a: ::simdty::i8x16) -> ::simdty::i8x16;
        /// The `llvm.x86.ssse3.pabs.w.128` intrinsic; known as `__builtin_ia32_pabsw128` in GCC.
        #[link_name = "llvm.x86.ssse3.pabs.w.128"]
        pub fn ssse3_pabs_w_128(a: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.ssse3.pabs.d.128` intrinsic; known as `__builtin_ia32_pabsd128` in GCC.
        #[link_name = "llvm.x86.ssse3.pabs.d.128"]
        pub fn ssse3_pabs_d_128(a: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.sse41.round.ss` intrinsic; known as `__builtin_ia32_roundss` in GCC.
        #[link_name = "llvm.x86.sse41.round.ss"]
        pub fn sse41_round_ss(a: ::simdty::f32x4, b: ::simdty::f32x4, c: i32) -> ::simdty::f32x4;
        /// The `llvm.x86.sse41.round.ps` intrinsic; known as `__builtin_ia32_roundps` in GCC.
        #[link_name = "llvm.x86.sse41.round.ps"]
        pub fn sse41_round_ps(a: ::simdty::f32x4, b: i32) -> ::simdty::f32x4;
        /// The `llvm.x86.sse41.round.sd` intrinsic; known as `__builtin_ia32_roundsd` in GCC.
        #[link_name = "llvm.x86.sse41.round.sd"]
        pub fn sse41_round_sd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: i32) -> ::simdty::f64x2;
        /// The `llvm.x86.sse41.round.pd` intrinsic; known as `__builtin_ia32_roundpd` in GCC.
        #[link_name = "llvm.x86.sse41.round.pd"]
        pub fn sse41_round_pd(a: ::simdty::f64x2, b: i32) -> ::simdty::f64x2;
        /// The `llvm.x86.sse41.pmovsxbd` intrinsic; known as `__builtin_ia32_pmovsxbd128` in GCC.
        #[link_name = "llvm.x86.sse41.pmovsxbd"]
        pub fn sse41_pmovsxbd(a: ::simdty::i8x16) -> ::simdty::i32x4;
        /// The `llvm.x86.sse41.pmovsxbq` intrinsic; known as `__builtin_ia32_pmovsxbq128` in GCC.
        #[link_name = "llvm.x86.sse41.pmovsxbq"]
        pub fn sse41_pmovsxbq(a: ::simdty::i8x16) -> ::simdty::i64x2;
        /// The `llvm.x86.sse41.pmovsxbw` intrinsic; known as `__builtin_ia32_pmovsxbw128` in GCC.
        #[link_name = "llvm.x86.sse41.pmovsxbw"]
        pub fn sse41_pmovsxbw(a: ::simdty::i8x16) -> ::simdty::i16x8;
        /// The `llvm.x86.sse41.pmovsxdq` intrinsic; known as `__builtin_ia32_pmovsxdq128` in GCC.
        #[link_name = "llvm.x86.sse41.pmovsxdq"]
        pub fn sse41_pmovsxdq(a: ::simdty::i32x4) -> ::simdty::i64x2;
        /// The `llvm.x86.sse41.pmovsxwd` intrinsic; known as `__builtin_ia32_pmovsxwd128` in GCC.
        #[link_name = "llvm.x86.sse41.pmovsxwd"]
        pub fn sse41_pmovsxwd(a: ::simdty::i16x8) -> ::simdty::i32x4;
        /// The `llvm.x86.sse41.pmovsxwq` intrinsic; known as `__builtin_ia32_pmovsxwq128` in GCC.
        #[link_name = "llvm.x86.sse41.pmovsxwq"]
        pub fn sse41_pmovsxwq(a: ::simdty::i16x8) -> ::simdty::i64x2;
        /// The `llvm.x86.sse41.pmovzxbd` intrinsic; known as `__builtin_ia32_pmovzxbd128` in GCC.
        #[link_name = "llvm.x86.sse41.pmovzxbd"]
        pub fn sse41_pmovzxbd(a: ::simdty::i8x16) -> ::simdty::i32x4;
        /// The `llvm.x86.sse41.pmovzxbq` intrinsic; known as `__builtin_ia32_pmovzxbq128` in GCC.
        #[link_name = "llvm.x86.sse41.pmovzxbq"]
        pub fn sse41_pmovzxbq(a: ::simdty::i8x16) -> ::simdty::i64x2;
        /// The `llvm.x86.sse41.pmovzxbw` intrinsic; known as `__builtin_ia32_pmovzxbw128` in GCC.
        #[link_name = "llvm.x86.sse41.pmovzxbw"]
        pub fn sse41_pmovzxbw(a: ::simdty::i8x16) -> ::simdty::i16x8;
        /// The `llvm.x86.sse41.pmovzxdq` intrinsic; known as `__builtin_ia32_pmovzxdq128` in GCC.
        #[link_name = "llvm.x86.sse41.pmovzxdq"]
        pub fn sse41_pmovzxdq(a: ::simdty::i32x4) -> ::simdty::i64x2;
        /// The `llvm.x86.sse41.pmovzxwd` intrinsic; known as `__builtin_ia32_pmovzxwd128` in GCC.
        #[link_name = "llvm.x86.sse41.pmovzxwd"]
        pub fn sse41_pmovzxwd(a: ::simdty::i16x8) -> ::simdty::i32x4;
        /// The `llvm.x86.sse41.pmovzxwq` intrinsic; known as `__builtin_ia32_pmovzxwq128` in GCC.
        #[link_name = "llvm.x86.sse41.pmovzxwq"]
        pub fn sse41_pmovzxwq(a: ::simdty::i16x8) -> ::simdty::i64x2;
        /// The `llvm.x86.sse41.phminposuw` intrinsic; known as `__builtin_ia32_phminposuw128` in GCC.
        #[link_name = "llvm.x86.sse41.phminposuw"]
        pub fn sse41_phminposuw(a: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.sse41.pmaxsb` intrinsic; known as `__builtin_ia32_pmaxsb128` in GCC.
        #[link_name = "llvm.x86.sse41.pmaxsb"]
        pub fn sse41_pmaxsb(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        /// The `llvm.x86.sse41.pmaxsd` intrinsic; known as `__builtin_ia32_pmaxsd128` in GCC.
        #[link_name = "llvm.x86.sse41.pmaxsd"]
        pub fn sse41_pmaxsd(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.sse41.pmaxud` intrinsic; known as `__builtin_ia32_pmaxud128` in GCC.
        #[link_name = "llvm.x86.sse41.pmaxud"]
        pub fn sse41_pmaxud(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.sse41.pmaxuw` intrinsic; known as `__builtin_ia32_pmaxuw128` in GCC.
        #[link_name = "llvm.x86.sse41.pmaxuw"]
        pub fn sse41_pmaxuw(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.sse41.pminsb` intrinsic; known as `__builtin_ia32_pminsb128` in GCC.
        #[link_name = "llvm.x86.sse41.pminsb"]
        pub fn sse41_pminsb(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        /// The `llvm.x86.sse41.pminsd` intrinsic; known as `__builtin_ia32_pminsd128` in GCC.
        #[link_name = "llvm.x86.sse41.pminsd"]
        pub fn sse41_pminsd(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.sse41.pminud` intrinsic; known as `__builtin_ia32_pminud128` in GCC.
        #[link_name = "llvm.x86.sse41.pminud"]
        pub fn sse41_pminud(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.sse41.pminuw` intrinsic; known as `__builtin_ia32_pminuw128` in GCC.
        #[link_name = "llvm.x86.sse41.pminuw"]
        pub fn sse41_pminuw(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.aesni.aesimc` intrinsic; known as `__builtin_ia32_aesimc128` in GCC.
        #[link_name = "llvm.x86.aesni.aesimc"]
        pub fn aesni_aesimc(a: ::simdty::i64x2) -> ::simdty::i64x2;
        /// The `llvm.x86.aesni.aesenc` intrinsic; known as `__builtin_ia32_aesenc128` in GCC.
        #[link_name = "llvm.x86.aesni.aesenc"]
        pub fn aesni_aesenc(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        /// The `llvm.x86.aesni.aesenclast` intrinsic; known as `__builtin_ia32_aesenclast128` in GCC.
        #[link_name = "llvm.x86.aesni.aesenclast"]
        pub fn aesni_aesenclast(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        /// The `llvm.x86.aesni.aesdec` intrinsic; known as `__builtin_ia32_aesdec128` in GCC.
        #[link_name = "llvm.x86.aesni.aesdec"]
        pub fn aesni_aesdec(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        /// The `llvm.x86.aesni.aesdeclast` intrinsic; known as `__builtin_ia32_aesdeclast128` in GCC.
        #[link_name = "llvm.x86.aesni.aesdeclast"]
        pub fn aesni_aesdeclast(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        /// The `llvm.x86.aesni.aeskeygenassist` intrinsic; known as `__builtin_ia32_aeskeygenassist128` in GCC.
        #[link_name = "llvm.x86.aesni.aeskeygenassist"]
        pub fn aesni_aeskeygenassist(a: ::simdty::i64x2, b: i8) -> ::simdty::i64x2;
        /// The `llvm.x86.pclmulqdq` intrinsic; known as `__builtin_ia32_pclmulqdq128` in GCC.
        #[link_name = "llvm.x86.pclmulqdq"]
        pub fn pclmulqdq(a: ::simdty::i64x2, b: ::simdty::i64x2, c: i8) -> ::simdty::i64x2;
        /// The `llvm.x86.sse41.packusdw` intrinsic; known as `__builtin_ia32_packusdw128` in GCC.
        #[link_name = "llvm.x86.sse41.packusdw"]
        pub fn sse41_packusdw(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i16x8;
        /// The `llvm.x86.sse41.pmuldq` intrinsic; known as `__builtin_ia32_pmuldq128` in GCC.
        #[link_name = "llvm.x86.sse41.pmuldq"]
        pub fn sse41_pmuldq(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i64x2;
        /// The `llvm.x86.sse41.pextrb` intrinsic.
        #[link_name = "llvm.x86.sse41.pextrb"]
        pub fn sse41_pextrb(a: ::simdty::i8x16, b: i32) -> i32;
        /// The `llvm.x86.sse41.pextrd` intrinsic.
        #[link_name = "llvm.x86.sse41.pextrd"]
        pub fn sse41_pextrd(a: ::simdty::i32x4, b: i32) -> i32;
        /// The `llvm.x86.sse41.pextrq` intrinsic.
        #[link_name = "llvm.x86.sse41.pextrq"]
        pub fn sse41_pextrq(a: ::simdty::i64x2, b: i32) -> i64;
        /// The `llvm.x86.sse41.extractps` intrinsic; known as `__builtin_ia32_extractps128` in GCC.
        #[link_name = "llvm.x86.sse41.extractps"]
        pub fn sse41_extractps(a: ::simdty::f32x4, b: i32) -> i32;
        /// The `llvm.x86.sse41.insertps` intrinsic; known as `__builtin_ia32_insertps128` in GCC.
        #[link_name = "llvm.x86.sse41.insertps"]
        pub fn sse41_insertps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: i8) -> ::simdty::f32x4;
        /// The `llvm.x86.sse41.pblendvb` intrinsic; known as `__builtin_ia32_pblendvb128` in GCC.
        #[link_name = "llvm.x86.sse41.pblendvb"]
        pub fn sse41_pblendvb(a: ::simdty::i8x16, b: ::simdty::i8x16, c: ::simdty::i8x16) -> ::simdty::i8x16;
        /// The `llvm.x86.sse41.pblendw` intrinsic; known as `__builtin_ia32_pblendw128` in GCC.
        #[link_name = "llvm.x86.sse41.pblendw"]
        pub fn sse41_pblendw(a: ::simdty::i16x8, b: ::simdty::i16x8, c: i8) -> ::simdty::i16x8;
        /// The `llvm.x86.sse41.blendpd` intrinsic; known as `__builtin_ia32_blendpd` in GCC.
        #[link_name = "llvm.x86.sse41.blendpd"]
        pub fn sse41_blendpd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: i8) -> ::simdty::f64x2;
        /// The `llvm.x86.sse41.blendps` intrinsic; known as `__builtin_ia32_blendps` in GCC.
        #[link_name = "llvm.x86.sse41.blendps"]
        pub fn sse41_blendps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: i8) -> ::simdty::f32x4;
        /// The `llvm.x86.sse41.blendvpd` intrinsic; known as `__builtin_ia32_blendvpd` in GCC.
        #[link_name = "llvm.x86.sse41.blendvpd"]
        pub fn sse41_blendvpd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.sse41.blendvps` intrinsic; known as `__builtin_ia32_blendvps` in GCC.
        #[link_name = "llvm.x86.sse41.blendvps"]
        pub fn sse41_blendvps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.sse41.dppd` intrinsic; known as `__builtin_ia32_dppd` in GCC.
        #[link_name = "llvm.x86.sse41.dppd"]
        pub fn sse41_dppd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: i8) -> ::simdty::f64x2;
        /// The `llvm.x86.sse41.dpps` intrinsic; known as `__builtin_ia32_dpps` in GCC.
        #[link_name = "llvm.x86.sse41.dpps"]
        pub fn sse41_dpps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: i8) -> ::simdty::f32x4;
        /// The `llvm.x86.sse41.mpsadbw` intrinsic; known as `__builtin_ia32_mpsadbw128` in GCC.
        #[link_name = "llvm.x86.sse41.mpsadbw"]
        pub fn sse41_mpsadbw(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i8) -> ::simdty::i16x8;
        /// The `llvm.x86.sse41.movntdqa` intrinsic; known as `__builtin_ia32_movntdqa` in GCC.
        #[link_name = "llvm.x86.sse41.movntdqa"]
        pub fn sse41_movntdqa(a: *mut i8) -> ::simdty::i64x2;
        /// The `llvm.x86.sse41.ptestz` intrinsic; known as `__builtin_ia32_ptestz128` in GCC.
        #[link_name = "llvm.x86.sse41.ptestz"]
        pub fn sse41_ptestz(a: ::simdty::i64x2, b: ::simdty::i64x2) -> i32;
        /// The `llvm.x86.sse41.ptestc` intrinsic; known as `__builtin_ia32_ptestc128` in GCC.
        #[link_name = "llvm.x86.sse41.ptestc"]
        pub fn sse41_ptestc(a: ::simdty::i64x2, b: ::simdty::i64x2) -> i32;
        /// The `llvm.x86.sse41.ptestnzc` intrinsic; known as `__builtin_ia32_ptestnzc128` in GCC.
        #[link_name = "llvm.x86.sse41.ptestnzc"]
        pub fn sse41_ptestnzc(a: ::simdty::i64x2, b: ::simdty::i64x2) -> i32;
        /// The `llvm.x86.sse42.crc32.32.8` intrinsic; known as `__builtin_ia32_crc32qi` in GCC.
        #[link_name = "llvm.x86.sse42.crc32.32.8"]
        pub fn sse42_crc32_32_8(a: i32, b: i8) -> i32;
        /// The `llvm.x86.sse42.crc32.32.16` intrinsic; known as `__builtin_ia32_crc32hi` in GCC.
        #[link_name = "llvm.x86.sse42.crc32.32.16"]
        pub fn sse42_crc32_32_16(a: i32, b: i16) -> i32;
        /// The `llvm.x86.sse42.crc32.32.32` intrinsic; known as `__builtin_ia32_crc32si` in GCC.
        #[link_name = "llvm.x86.sse42.crc32.32.32"]
        pub fn sse42_crc32_32_32(a: i32, b: i32) -> i32;
        /// The `llvm.x86.sse42.crc32.64.64` intrinsic; known as `__builtin_ia32_crc32di` in GCC.
        #[link_name = "llvm.x86.sse42.crc32.64.64"]
        pub fn sse42_crc32_64_64(a: i64, b: i64) -> i64;
        /// The `llvm.x86.sse42.pcmpistrm128` intrinsic; known as `__builtin_ia32_pcmpistrm128` in GCC.
        #[link_name = "llvm.x86.sse42.pcmpistrm128"]
        pub fn sse42_pcmpistrm128(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i8) -> ::simdty::i8x16;
        /// The `llvm.x86.sse42.pcmpistri128` intrinsic; known as `__builtin_ia32_pcmpistri128` in GCC.
        #[link_name = "llvm.x86.sse42.pcmpistri128"]
        pub fn sse42_pcmpistri128(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i8) -> i32;
        /// The `llvm.x86.sse42.pcmpistria128` intrinsic; known as `__builtin_ia32_pcmpistria128` in GCC.
        #[link_name = "llvm.x86.sse42.pcmpistria128"]
        pub fn sse42_pcmpistria128(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i8) -> i32;
        /// The `llvm.x86.sse42.pcmpistric128` intrinsic; known as `__builtin_ia32_pcmpistric128` in GCC.
        #[link_name = "llvm.x86.sse42.pcmpistric128"]
        pub fn sse42_pcmpistric128(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i8) -> i32;
        /// The `llvm.x86.sse42.pcmpistrio128` intrinsic; known as `__builtin_ia32_pcmpistrio128` in GCC.
        #[link_name = "llvm.x86.sse42.pcmpistrio128"]
        pub fn sse42_pcmpistrio128(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i8) -> i32;
        /// The `llvm.x86.sse42.pcmpistris128` intrinsic; known as `__builtin_ia32_pcmpistris128` in GCC.
        #[link_name = "llvm.x86.sse42.pcmpistris128"]
        pub fn sse42_pcmpistris128(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i8) -> i32;
        /// The `llvm.x86.sse42.pcmpistriz128` intrinsic; known as `__builtin_ia32_pcmpistriz128` in GCC.
        #[link_name = "llvm.x86.sse42.pcmpistriz128"]
        pub fn sse42_pcmpistriz128(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i8) -> i32;
        /// The `llvm.x86.sse42.pcmpestrm128` intrinsic; known as `__builtin_ia32_pcmpestrm128` in GCC.
        #[link_name = "llvm.x86.sse42.pcmpestrm128"]
        pub fn sse42_pcmpestrm128(a: ::simdty::i8x16, b: i32, c: ::simdty::i8x16, d: i32, e: i8) -> ::simdty::i8x16;
        /// The `llvm.x86.sse42.pcmpestri128` intrinsic; known as `__builtin_ia32_pcmpestri128` in GCC.
        #[link_name = "llvm.x86.sse42.pcmpestri128"]
        pub fn sse42_pcmpestri128(a: ::simdty::i8x16, b: i32, c: ::simdty::i8x16, d: i32, e: i8) -> i32;
        /// The `llvm.x86.sse42.pcmpestria128` intrinsic; known as `__builtin_ia32_pcmpestria128` in GCC.
        #[link_name = "llvm.x86.sse42.pcmpestria128"]
        pub fn sse42_pcmpestria128(a: ::simdty::i8x16, b: i32, c: ::simdty::i8x16, d: i32, e: i8) -> i32;
        /// The `llvm.x86.sse42.pcmpestric128` intrinsic; known as `__builtin_ia32_pcmpestric128` in GCC.
        #[link_name = "llvm.x86.sse42.pcmpestric128"]
        pub fn sse42_pcmpestric128(a: ::simdty::i8x16, b: i32, c: ::simdty::i8x16, d: i32, e: i8) -> i32;
        /// The `llvm.x86.sse42.pcmpestrio128` intrinsic; known as `__builtin_ia32_pcmpestrio128` in GCC.
        #[link_name = "llvm.x86.sse42.pcmpestrio128"]
        pub fn sse42_pcmpestrio128(a: ::simdty::i8x16, b: i32, c: ::simdty::i8x16, d: i32, e: i8) -> i32;
        /// The `llvm.x86.sse42.pcmpestris128` intrinsic; known as `__builtin_ia32_pcmpestris128` in GCC.
        #[link_name = "llvm.x86.sse42.pcmpestris128"]
        pub fn sse42_pcmpestris128(a: ::simdty::i8x16, b: i32, c: ::simdty::i8x16, d: i32, e: i8) -> i32;
        /// The `llvm.x86.sse42.pcmpestriz128` intrinsic; known as `__builtin_ia32_pcmpestriz128` in GCC.
        #[link_name = "llvm.x86.sse42.pcmpestriz128"]
        pub fn sse42_pcmpestriz128(a: ::simdty::i8x16, b: i32, c: ::simdty::i8x16, d: i32, e: i8) -> i32;
        /// The `llvm.x86.sse4a.extrqi` intrinsic; known as `__builtin_ia32_extrqi` in GCC.
        #[link_name = "llvm.x86.sse4a.extrqi"]
        pub fn sse4a_extrqi(a: ::simdty::i64x2, b: i8, c: i8) -> ::simdty::i64x2;
        /// The `llvm.x86.sse4a.extrq` intrinsic; known as `__builtin_ia32_extrq` in GCC.
        #[link_name = "llvm.x86.sse4a.extrq"]
        pub fn sse4a_extrq(a: ::simdty::i64x2, b: ::simdty::i8x16) -> ::simdty::i64x2;
        /// The `llvm.x86.sse4a.insertqi` intrinsic; known as `__builtin_ia32_insertqi` in GCC.
        #[link_name = "llvm.x86.sse4a.insertqi"]
        pub fn sse4a_insertqi(a: ::simdty::i64x2, b: ::simdty::i64x2, c: i8, d: i8) -> ::simdty::i64x2;
        /// The `llvm.x86.sse4a.insertq` intrinsic; known as `__builtin_ia32_insertq` in GCC.
        #[link_name = "llvm.x86.sse4a.insertq"]
        pub fn sse4a_insertq(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        /// The `llvm.x86.sse4a.movnt.ss` intrinsic; known as `__builtin_ia32_movntss` in GCC.
        #[link_name = "llvm.x86.sse4a.movnt.ss"]
        pub fn sse4a_movnt_ss(a: *mut i8, b: ::simdty::f32x4) -> ();
        /// The `llvm.x86.sse4a.movnt.sd` intrinsic; known as `__builtin_ia32_movntsd` in GCC.
        #[link_name = "llvm.x86.sse4a.movnt.sd"]
        pub fn sse4a_movnt_sd(a: *mut i8, b: ::simdty::f64x2) -> ();
        /// The `llvm.x86.avx.addsub.pd.256` intrinsic; known as `__builtin_ia32_addsubpd256` in GCC.
        #[link_name = "llvm.x86.avx.addsub.pd.256"]
        pub fn avx_addsub_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4) -> ::simdty::f64x4;
        /// The `llvm.x86.avx.addsub.ps.256` intrinsic; known as `__builtin_ia32_addsubps256` in GCC.
        #[link_name = "llvm.x86.avx.addsub.ps.256"]
        pub fn avx_addsub_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8) -> ::simdty::f32x8;
        /// The `llvm.x86.avx.max.pd.256` intrinsic; known as `__builtin_ia32_maxpd256` in GCC.
        #[link_name = "llvm.x86.avx.max.pd.256"]
        pub fn avx_max_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4) -> ::simdty::f64x4;
        /// The `llvm.x86.avx.max.ps.256` intrinsic; known as `__builtin_ia32_maxps256` in GCC.
        #[link_name = "llvm.x86.avx.max.ps.256"]
        pub fn avx_max_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8) -> ::simdty::f32x8;
        /// The `llvm.x86.avx.min.pd.256` intrinsic; known as `__builtin_ia32_minpd256` in GCC.
        #[link_name = "llvm.x86.avx.min.pd.256"]
        pub fn avx_min_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4) -> ::simdty::f64x4;
        /// The `llvm.x86.avx.min.ps.256` intrinsic; known as `__builtin_ia32_minps256` in GCC.
        #[link_name = "llvm.x86.avx.min.ps.256"]
        pub fn avx_min_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8) -> ::simdty::f32x8;
        /// The `llvm.x86.avx.sqrt.pd.256` intrinsic; known as `__builtin_ia32_sqrtpd256` in GCC.
        #[link_name = "llvm.x86.avx.sqrt.pd.256"]
        pub fn avx_sqrt_pd_256(a: ::simdty::f64x4) -> ::simdty::f64x4;
        /// The `llvm.x86.avx.sqrt.ps.256` intrinsic; known as `__builtin_ia32_sqrtps256` in GCC.
        #[link_name = "llvm.x86.avx.sqrt.ps.256"]
        pub fn avx_sqrt_ps_256(a: ::simdty::f32x8) -> ::simdty::f32x8;
        /// The `llvm.x86.avx.rsqrt.ps.256` intrinsic; known as `__builtin_ia32_rsqrtps256` in GCC.
        #[link_name = "llvm.x86.avx.rsqrt.ps.256"]
        pub fn avx_rsqrt_ps_256(a: ::simdty::f32x8) -> ::simdty::f32x8;
        /// The `llvm.x86.avx.rcp.ps.256` intrinsic; known as `__builtin_ia32_rcpps256` in GCC.
        #[link_name = "llvm.x86.avx.rcp.ps.256"]
        pub fn avx_rcp_ps_256(a: ::simdty::f32x8) -> ::simdty::f32x8;
        /// The `llvm.x86.avx.round.pd.256` intrinsic; known as `__builtin_ia32_roundpd256` in GCC.
        #[link_name = "llvm.x86.avx.round.pd.256"]
        pub fn avx_round_pd_256(a: ::simdty::f64x4, b: i32) -> ::simdty::f64x4;
        /// The `llvm.x86.avx.round.ps.256` intrinsic; known as `__builtin_ia32_roundps256` in GCC.
        #[link_name = "llvm.x86.avx.round.ps.256"]
        pub fn avx_round_ps_256(a: ::simdty::f32x8, b: i32) -> ::simdty::f32x8;
        /// The `llvm.x86.avx.hadd.pd.256` intrinsic; known as `__builtin_ia32_haddpd256` in GCC.
        #[link_name = "llvm.x86.avx.hadd.pd.256"]
        pub fn avx_hadd_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4) -> ::simdty::f64x4;
        /// The `llvm.x86.avx.hsub.ps.256` intrinsic; known as `__builtin_ia32_hsubps256` in GCC.
        #[link_name = "llvm.x86.avx.hsub.ps.256"]
        pub fn avx_hsub_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8) -> ::simdty::f32x8;
        /// The `llvm.x86.avx.hsub.pd.256` intrinsic; known as `__builtin_ia32_hsubpd256` in GCC.
        #[link_name = "llvm.x86.avx.hsub.pd.256"]
        pub fn avx_hsub_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4) -> ::simdty::f64x4;
        /// The `llvm.x86.avx.hadd.ps.256` intrinsic; known as `__builtin_ia32_haddps256` in GCC.
        #[link_name = "llvm.x86.avx.hadd.ps.256"]
        pub fn avx_hadd_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8) -> ::simdty::f32x8;
        /// The `llvm.x86.avx.vpermilvar.pd` intrinsic; known as `__builtin_ia32_vpermilvarpd` in GCC.
        #[link_name = "llvm.x86.avx.vpermilvar.pd"]
        pub fn avx_vpermilvar_pd(a: ::simdty::f64x2, b: ::simdty::i64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.avx.vpermilvar.ps` intrinsic; known as `__builtin_ia32_vpermilvarps` in GCC.
        #[link_name = "llvm.x86.avx.vpermilvar.ps"]
        pub fn avx_vpermilvar_ps(a: ::simdty::f32x4, b: ::simdty::i32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.avx.vpermilvar.pd.256` intrinsic; known as `__builtin_ia32_vpermilvarpd256` in GCC.
        #[link_name = "llvm.x86.avx.vpermilvar.pd.256"]
        pub fn avx_vpermilvar_pd_256(a: ::simdty::f64x4, b: ::simdty::i64x4) -> ::simdty::f64x4;
        /// The `llvm.x86.avx.vpermilvar.ps.256` intrinsic; known as `__builtin_ia32_vpermilvarps256` in GCC.
        #[link_name = "llvm.x86.avx.vpermilvar.ps.256"]
        pub fn avx_vpermilvar_ps_256(a: ::simdty::f32x8, b: ::simdty::i32x8) -> ::simdty::f32x8;
        /// The `llvm.x86.avx.vperm2f128.pd.256` intrinsic; known as `__builtin_ia32_vperm2f128_pd256` in GCC.
        #[link_name = "llvm.x86.avx.vperm2f128.pd.256"]
        pub fn avx_vperm2f128_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4, c: i8) -> ::simdty::f64x4;
        /// The `llvm.x86.avx.vperm2f128.ps.256` intrinsic; known as `__builtin_ia32_vperm2f128_ps256` in GCC.
        #[link_name = "llvm.x86.avx.vperm2f128.ps.256"]
        pub fn avx_vperm2f128_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: i8) -> ::simdty::f32x8;
        /// The `llvm.x86.avx.vperm2f128.si.256` intrinsic; known as `__builtin_ia32_vperm2f128_si256` in GCC.
        #[link_name = "llvm.x86.avx.vperm2f128.si.256"]
        pub fn avx_vperm2f128_si_256(a: ::simdty::i32x8, b: ::simdty::i32x8, c: i8) -> ::simdty::i32x8;
        /// The `llvm.x86.avx512.mask.vpermt.d.512` intrinsic; known as `__builtin_ia32_vpermt2vard512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.vpermt.d.512"]
        pub fn avx512_mask_vpermt_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: ::simdty::i32x16, d: i16) -> ::simdty::i32x16;
        /// The `llvm.x86.avx512.mask.vpermt.q.512` intrinsic; known as `__builtin_ia32_vpermt2varq512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.vpermt.q.512"]
        pub fn avx512_mask_vpermt_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: ::simdty::i64x8, d: i8) -> ::simdty::i64x8;
        /// The `llvm.x86.avx512.mask.vpermt.ps.512` intrinsic; known as `__builtin_ia32_vpermt2varps512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.vpermt.ps.512"]
        pub fn avx512_mask_vpermt_ps_512(a: ::simdty::i32x16, b: ::simdty::f32x16, c: ::simdty::f32x16, d: i16) -> ::simdty::f32x16;
        /// The `llvm.x86.avx512.mask.vpermt.pd.512` intrinsic; known as `__builtin_ia32_vpermt2varpd512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.vpermt.pd.512"]
        pub fn avx512_mask_vpermt_pd_512(a: ::simdty::i64x8, b: ::simdty::f64x8, c: ::simdty::f64x8, d: i8) -> ::simdty::f64x8;
        /// The `llvm.x86.avx.blend.pd.256` intrinsic; known as `__builtin_ia32_blendpd256` in GCC.
        #[link_name = "llvm.x86.avx.blend.pd.256"]
        pub fn avx_blend_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4, c: i8) -> ::simdty::f64x4;
        /// The `llvm.x86.avx.blend.ps.256` intrinsic; known as `__builtin_ia32_blendps256` in GCC.
        #[link_name = "llvm.x86.avx.blend.ps.256"]
        pub fn avx_blend_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: i8) -> ::simdty::f32x8;
        /// The `llvm.x86.avx.blendv.pd.256` intrinsic; known as `__builtin_ia32_blendvpd256` in GCC.
        #[link_name = "llvm.x86.avx.blendv.pd.256"]
        pub fn avx_blendv_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4, c: ::simdty::f64x4) -> ::simdty::f64x4;
        /// The `llvm.x86.avx.blendv.ps.256` intrinsic; known as `__builtin_ia32_blendvps256` in GCC.
        #[link_name = "llvm.x86.avx.blendv.ps.256"]
        pub fn avx_blendv_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: ::simdty::f32x8) -> ::simdty::f32x8;
        /// The `llvm.x86.avx.dp.ps.256` intrinsic; known as `__builtin_ia32_dpps256` in GCC.
        #[link_name = "llvm.x86.avx.dp.ps.256"]
        pub fn avx_dp_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: i8) -> ::simdty::f32x8;
        /// The `llvm.x86.avx.cmp.pd.256` intrinsic; known as `__builtin_ia32_cmppd256` in GCC.
        #[link_name = "llvm.x86.avx.cmp.pd.256"]
        pub fn avx_cmp_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4, c: i8) -> ::simdty::f64x4;
        /// The `llvm.x86.avx.cmp.ps.256` intrinsic; known as `__builtin_ia32_cmpps256` in GCC.
        #[link_name = "llvm.x86.avx.cmp.ps.256"]
        pub fn avx_cmp_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: i8) -> ::simdty::f32x8;
        /// The `llvm.x86.avx.vextractf128.pd.256` intrinsic; known as `__builtin_ia32_vextractf128_pd256` in GCC.
        #[link_name = "llvm.x86.avx.vextractf128.pd.256"]
        pub fn avx_vextractf128_pd_256(a: ::simdty::f64x4, b: i8) -> ::simdty::f64x2;
        /// The `llvm.x86.avx.vextractf128.ps.256` intrinsic; known as `__builtin_ia32_vextractf128_ps256` in GCC.
        #[link_name = "llvm.x86.avx.vextractf128.ps.256"]
        pub fn avx_vextractf128_ps_256(a: ::simdty::f32x8, b: i8) -> ::simdty::f32x4;
        /// The `llvm.x86.avx.vextractf128.si.256` intrinsic; known as `__builtin_ia32_vextractf128_si256` in GCC.
        #[link_name = "llvm.x86.avx.vextractf128.si.256"]
        pub fn avx_vextractf128_si_256(a: ::simdty::i32x8, b: i8) -> ::simdty::i32x4;
        /// The `llvm.x86.avx.vinsertf128.pd.256` intrinsic; known as `__builtin_ia32_vinsertf128_pd256` in GCC.
        #[link_name = "llvm.x86.avx.vinsertf128.pd.256"]
        pub fn avx_vinsertf128_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x2, c: i8) -> ::simdty::f64x4;
        /// The `llvm.x86.avx.vinsertf128.ps.256` intrinsic; known as `__builtin_ia32_vinsertf128_ps256` in GCC.
        #[link_name = "llvm.x86.avx.vinsertf128.ps.256"]
        pub fn avx_vinsertf128_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x4, c: i8) -> ::simdty::f32x8;
        /// The `llvm.x86.avx.vinsertf128.si.256` intrinsic; known as `__builtin_ia32_vinsertf128_si256` in GCC.
        #[link_name = "llvm.x86.avx.vinsertf128.si.256"]
        pub fn avx_vinsertf128_si_256(a: ::simdty::i32x8, b: ::simdty::i32x4, c: i8) -> ::simdty::i32x8;
        /// The `llvm.x86.avx.cvtdq2.pd.256` intrinsic; known as `__builtin_ia32_cvtdq2pd256` in GCC.
        #[link_name = "llvm.x86.avx.cvtdq2.pd.256"]
        pub fn avx_cvtdq2_pd_256(a: ::simdty::i32x4) -> ::simdty::f64x4;
        /// The `llvm.x86.avx.cvtdq2.ps.256` intrinsic; known as `__builtin_ia32_cvtdq2ps256` in GCC.
        #[link_name = "llvm.x86.avx.cvtdq2.ps.256"]
        pub fn avx_cvtdq2_ps_256(a: ::simdty::i32x8) -> ::simdty::f32x8;
        /// The `llvm.x86.avx.cvt.pd2.ps.256` intrinsic; known as `__builtin_ia32_cvtpd2ps256` in GCC.
        #[link_name = "llvm.x86.avx.cvt.pd2.ps.256"]
        pub fn avx_cvt_pd2_ps_256(a: ::simdty::f64x4) -> ::simdty::f32x4;
        /// The `llvm.x86.avx.cvt.ps2dq.256` intrinsic; known as `__builtin_ia32_cvtps2dq256` in GCC.
        #[link_name = "llvm.x86.avx.cvt.ps2dq.256"]
        pub fn avx_cvt_ps2dq_256(a: ::simdty::f32x8) -> ::simdty::i32x8;
        /// The `llvm.x86.avx.cvt.ps2.pd.256` intrinsic; known as `__builtin_ia32_cvtps2pd256` in GCC.
        #[link_name = "llvm.x86.avx.cvt.ps2.pd.256"]
        pub fn avx_cvt_ps2_pd_256(a: ::simdty::f32x4) -> ::simdty::f64x4;
        /// The `llvm.x86.avx.cvtt.pd2dq.256` intrinsic; known as `__builtin_ia32_cvttpd2dq256` in GCC.
        #[link_name = "llvm.x86.avx.cvtt.pd2dq.256"]
        pub fn avx_cvtt_pd2dq_256(a: ::simdty::f64x4) -> ::simdty::i32x4;
        /// The `llvm.x86.avx.cvt.pd2dq.256` intrinsic; known as `__builtin_ia32_cvtpd2dq256` in GCC.
        #[link_name = "llvm.x86.avx.cvt.pd2dq.256"]
        pub fn avx_cvt_pd2dq_256(a: ::simdty::f64x4) -> ::simdty::i32x4;
        /// The `llvm.x86.avx.cvtt.ps2dq.256` intrinsic; known as `__builtin_ia32_cvttps2dq256` in GCC.
        #[link_name = "llvm.x86.avx.cvtt.ps2dq.256"]
        pub fn avx_cvtt_ps2dq_256(a: ::simdty::f32x8) -> ::simdty::i32x8;
        /// The `llvm.x86.avx.vtestz.pd` intrinsic; known as `__builtin_ia32_vtestzpd` in GCC.
        #[link_name = "llvm.x86.avx.vtestz.pd"]
        pub fn avx_vtestz_pd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        /// The `llvm.x86.avx.vtestc.pd` intrinsic; known as `__builtin_ia32_vtestcpd` in GCC.
        #[link_name = "llvm.x86.avx.vtestc.pd"]
        pub fn avx_vtestc_pd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        /// The `llvm.x86.avx.vtestnzc.pd` intrinsic; known as `__builtin_ia32_vtestnzcpd` in GCC.
        #[link_name = "llvm.x86.avx.vtestnzc.pd"]
        pub fn avx_vtestnzc_pd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        /// The `llvm.x86.avx.vtestz.ps` intrinsic; known as `__builtin_ia32_vtestzps` in GCC.
        #[link_name = "llvm.x86.avx.vtestz.ps"]
        pub fn avx_vtestz_ps(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        /// The `llvm.x86.avx.vtestc.ps` intrinsic; known as `__builtin_ia32_vtestcps` in GCC.
        #[link_name = "llvm.x86.avx.vtestc.ps"]
        pub fn avx_vtestc_ps(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        /// The `llvm.x86.avx.vtestnzc.ps` intrinsic; known as `__builtin_ia32_vtestnzcps` in GCC.
        #[link_name = "llvm.x86.avx.vtestnzc.ps"]
        pub fn avx_vtestnzc_ps(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        /// The `llvm.x86.avx.vtestz.pd.256` intrinsic; known as `__builtin_ia32_vtestzpd256` in GCC.
        #[link_name = "llvm.x86.avx.vtestz.pd.256"]
        pub fn avx_vtestz_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4) -> i32;
        /// The `llvm.x86.avx.vtestc.pd.256` intrinsic; known as `__builtin_ia32_vtestcpd256` in GCC.
        #[link_name = "llvm.x86.avx.vtestc.pd.256"]
        pub fn avx_vtestc_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4) -> i32;
        /// The `llvm.x86.avx.vtestnzc.pd.256` intrinsic; known as `__builtin_ia32_vtestnzcpd256` in GCC.
        #[link_name = "llvm.x86.avx.vtestnzc.pd.256"]
        pub fn avx_vtestnzc_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4) -> i32;
        /// The `llvm.x86.avx.vtestz.ps.256` intrinsic; known as `__builtin_ia32_vtestzps256` in GCC.
        #[link_name = "llvm.x86.avx.vtestz.ps.256"]
        pub fn avx_vtestz_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8) -> i32;
        /// The `llvm.x86.avx.vtestc.ps.256` intrinsic; known as `__builtin_ia32_vtestcps256` in GCC.
        #[link_name = "llvm.x86.avx.vtestc.ps.256"]
        pub fn avx_vtestc_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8) -> i32;
        /// The `llvm.x86.avx.vtestnzc.ps.256` intrinsic; known as `__builtin_ia32_vtestnzcps256` in GCC.
        #[link_name = "llvm.x86.avx.vtestnzc.ps.256"]
        pub fn avx_vtestnzc_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8) -> i32;
        /// The `llvm.x86.avx.ptestz.256` intrinsic; known as `__builtin_ia32_ptestz256` in GCC.
        #[link_name = "llvm.x86.avx.ptestz.256"]
        pub fn avx_ptestz_256(a: ::simdty::i64x4, b: ::simdty::i64x4) -> i32;
        /// The `llvm.x86.avx.ptestc.256` intrinsic; known as `__builtin_ia32_ptestc256` in GCC.
        #[link_name = "llvm.x86.avx.ptestc.256"]
        pub fn avx_ptestc_256(a: ::simdty::i64x4, b: ::simdty::i64x4) -> i32;
        /// The `llvm.x86.avx.ptestnzc.256` intrinsic; known as `__builtin_ia32_ptestnzc256` in GCC.
        #[link_name = "llvm.x86.avx.ptestnzc.256"]
        pub fn avx_ptestnzc_256(a: ::simdty::i64x4, b: ::simdty::i64x4) -> i32;
        /// The `llvm.x86.avx512.mask.ptestm.d.512` intrinsic; known as `__builtin_ia32_ptestmd512` in GCC.
        #[link_name = "llvm.x86.avx512.mask.ptestm.d.512"]
        pub fn avx512_mask_ptestm_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: i16) -> i16;
        /// The `llvm.x86.avx512.mask.ptestm.q.512` intrinsic; known as `__builtin_ia32_ptestmq512` in GCC.
        #[link_name = "llvm.x86.avx512.mask.ptestm.q.512"]
        pub fn avx512_mask_ptestm_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: i8) -> i8;
        /// The `llvm.x86.avx.movmsk.pd.256` intrinsic; known as `__builtin_ia32_movmskpd256` in GCC.
        #[link_name = "llvm.x86.avx.movmsk.pd.256"]
        pub fn avx_movmsk_pd_256(a: ::simdty::f64x4) -> i32;
        /// The `llvm.x86.avx.movmsk.ps.256` intrinsic; known as `__builtin_ia32_movmskps256` in GCC.
        #[link_name = "llvm.x86.avx.movmsk.ps.256"]
        pub fn avx_movmsk_ps_256(a: ::simdty::f32x8) -> i32;
        /// The `llvm.x86.avx.vzeroall` intrinsic; known as `__builtin_ia32_vzeroall` in GCC.
        #[link_name = "llvm.x86.avx.vzeroall"]
        pub fn avx_vzeroall() -> ();
        /// The `llvm.x86.avx.vzeroupper` intrinsic; known as `__builtin_ia32_vzeroupper` in GCC.
        #[link_name = "llvm.x86.avx.vzeroupper"]
        pub fn avx_vzeroupper() -> ();
        /// The `llvm.x86.avx.vbroadcastf128.pd.256` intrinsic; known as `__builtin_ia32_vbroadcastf128_pd256` in GCC.
        #[link_name = "llvm.x86.avx.vbroadcastf128.pd.256"]
        pub fn avx_vbroadcastf128_pd_256(a: *mut i8) -> ::simdty::f64x4;
        /// The `llvm.x86.avx.vbroadcastf128.ps.256` intrinsic; known as `__builtin_ia32_vbroadcastf128_ps256` in GCC.
        #[link_name = "llvm.x86.avx.vbroadcastf128.ps.256"]
        pub fn avx_vbroadcastf128_ps_256(a: *mut i8) -> ::simdty::f32x8;
        /// The `llvm.x86.avx.ldu.dq.256` intrinsic; known as `__builtin_ia32_lddqu256` in GCC.
        #[link_name = "llvm.x86.avx.ldu.dq.256"]
        pub fn avx_ldu_dq_256(a: *mut i8) -> ::simdty::i8x32;
        /// The `llvm.x86.avx.storeu.pd.256` intrinsic; known as `__builtin_ia32_storeupd256` in GCC.
        #[link_name = "llvm.x86.avx.storeu.pd.256"]
        pub fn avx_storeu_pd_256(a: *mut i8, b: ::simdty::f64x4) -> ();
        /// The `llvm.x86.avx.storeu.ps.256` intrinsic; known as `__builtin_ia32_storeups256` in GCC.
        #[link_name = "llvm.x86.avx.storeu.ps.256"]
        pub fn avx_storeu_ps_256(a: *mut i8, b: ::simdty::f32x8) -> ();
        /// The `llvm.x86.avx.storeu.dq.256` intrinsic; known as `__builtin_ia32_storedqu256` in GCC.
        #[link_name = "llvm.x86.avx.storeu.dq.256"]
        pub fn avx_storeu_dq_256(a: *mut i8, b: ::simdty::i8x32) -> ();
        /// The `llvm.x86.avx.maskload.pd` intrinsic; known as `__builtin_ia32_maskloadpd` in GCC.
        #[link_name = "llvm.x86.avx.maskload.pd"]
        pub fn avx_maskload_pd(a: *mut i8, b: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.avx.maskload.ps` intrinsic; known as `__builtin_ia32_maskloadps` in GCC.
        #[link_name = "llvm.x86.avx.maskload.ps"]
        pub fn avx_maskload_ps(a: *mut i8, b: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.avx.maskload.pd.256` intrinsic; known as `__builtin_ia32_maskloadpd256` in GCC.
        #[link_name = "llvm.x86.avx.maskload.pd.256"]
        pub fn avx_maskload_pd_256(a: *mut i8, b: ::simdty::f64x4) -> ::simdty::f64x4;
        /// The `llvm.x86.avx.maskload.ps.256` intrinsic; known as `__builtin_ia32_maskloadps256` in GCC.
        #[link_name = "llvm.x86.avx.maskload.ps.256"]
        pub fn avx_maskload_ps_256(a: *mut i8, b: ::simdty::f32x8) -> ::simdty::f32x8;
        /// The `llvm.x86.avx512.mask.loadu.ps.512` intrinsic; known as `__builtin_ia32_loadups512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.loadu.ps.512"]
        pub fn avx512_mask_loadu_ps_512(a: *mut i8, b: ::simdty::f32x16, c: i16) -> ::simdty::f32x16;
        /// The `llvm.x86.avx512.mask.loadu.pd.512` intrinsic; known as `__builtin_ia32_loadupd512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.loadu.pd.512"]
        pub fn avx512_mask_loadu_pd_512(a: *mut i8, b: ::simdty::f64x8, c: i8) -> ::simdty::f64x8;
        /// The `llvm.x86.avx.maskstore.pd` intrinsic; known as `__builtin_ia32_maskstorepd` in GCC.
        #[link_name = "llvm.x86.avx.maskstore.pd"]
        pub fn avx_maskstore_pd(a: *mut i8, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ();
        /// The `llvm.x86.avx.maskstore.ps` intrinsic; known as `__builtin_ia32_maskstoreps` in GCC.
        #[link_name = "llvm.x86.avx.maskstore.ps"]
        pub fn avx_maskstore_ps(a: *mut i8, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ();
        /// The `llvm.x86.avx.maskstore.pd.256` intrinsic; known as `__builtin_ia32_maskstorepd256` in GCC.
        #[link_name = "llvm.x86.avx.maskstore.pd.256"]
        pub fn avx_maskstore_pd_256(a: *mut i8, b: ::simdty::f64x4, c: ::simdty::f64x4) -> ();
        /// The `llvm.x86.avx.maskstore.ps.256` intrinsic; known as `__builtin_ia32_maskstoreps256` in GCC.
        #[link_name = "llvm.x86.avx.maskstore.ps.256"]
        pub fn avx_maskstore_ps_256(a: *mut i8, b: ::simdty::f32x8, c: ::simdty::f32x8) -> ();
        /// The `llvm.x86.avx512.mask.storeu.ps.512` intrinsic; known as `__builtin_ia32_storeups512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.storeu.ps.512"]
        pub fn avx512_mask_storeu_ps_512(a: *mut i8, b: ::simdty::f32x16, c: i16) -> ();
        /// The `llvm.x86.avx512.mask.storeu.pd.512` intrinsic; known as `__builtin_ia32_storeupd512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.storeu.pd.512"]
        pub fn avx512_mask_storeu_pd_512(a: *mut i8, b: ::simdty::f64x8, c: i8) -> ();
        /// The `llvm.x86.avx512.mask.store.ss` intrinsic; known as `__builtin_ia32_storess_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.store.ss"]
        pub fn avx512_mask_store_ss(a: *mut i8, b: ::simdty::f32x4, c: i8) -> ();
        /// The `llvm.x86.avx2.padds.b` intrinsic; known as `__builtin_ia32_paddsb256` in GCC.
        #[link_name = "llvm.x86.avx2.padds.b"]
        pub fn avx2_padds_b(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i8x32;
        /// The `llvm.x86.avx2.padds.w` intrinsic; known as `__builtin_ia32_paddsw256` in GCC.
        #[link_name = "llvm.x86.avx2.padds.w"]
        pub fn avx2_padds_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.paddus.b` intrinsic; known as `__builtin_ia32_paddusb256` in GCC.
        #[link_name = "llvm.x86.avx2.paddus.b"]
        pub fn avx2_paddus_b(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i8x32;
        /// The `llvm.x86.avx2.paddus.w` intrinsic; known as `__builtin_ia32_paddusw256` in GCC.
        #[link_name = "llvm.x86.avx2.paddus.w"]
        pub fn avx2_paddus_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.psubs.b` intrinsic; known as `__builtin_ia32_psubsb256` in GCC.
        #[link_name = "llvm.x86.avx2.psubs.b"]
        pub fn avx2_psubs_b(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i8x32;
        /// The `llvm.x86.avx2.psubs.w` intrinsic; known as `__builtin_ia32_psubsw256` in GCC.
        #[link_name = "llvm.x86.avx2.psubs.w"]
        pub fn avx2_psubs_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.psubus.b` intrinsic; known as `__builtin_ia32_psubusb256` in GCC.
        #[link_name = "llvm.x86.avx2.psubus.b"]
        pub fn avx2_psubus_b(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i8x32;
        /// The `llvm.x86.avx2.psubus.w` intrinsic; known as `__builtin_ia32_psubusw256` in GCC.
        #[link_name = "llvm.x86.avx2.psubus.w"]
        pub fn avx2_psubus_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.pmulhu.w` intrinsic; known as `__builtin_ia32_pmulhuw256` in GCC.
        #[link_name = "llvm.x86.avx2.pmulhu.w"]
        pub fn avx2_pmulhu_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.pmulh.w` intrinsic; known as `__builtin_ia32_pmulhw256` in GCC.
        #[link_name = "llvm.x86.avx2.pmulh.w"]
        pub fn avx2_pmulh_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.pmulu.dq` intrinsic; known as `__builtin_ia32_pmuludq256` in GCC.
        #[link_name = "llvm.x86.avx2.pmulu.dq"]
        pub fn avx2_pmulu_dq(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.pmul.dq` intrinsic; known as `__builtin_ia32_pmuldq256` in GCC.
        #[link_name = "llvm.x86.avx2.pmul.dq"]
        pub fn avx2_pmul_dq(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.pmadd.wd` intrinsic; known as `__builtin_ia32_pmaddwd256` in GCC.
        #[link_name = "llvm.x86.avx2.pmadd.wd"]
        pub fn avx2_pmadd_wd(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.pavg.b` intrinsic; known as `__builtin_ia32_pavgb256` in GCC.
        #[link_name = "llvm.x86.avx2.pavg.b"]
        pub fn avx2_pavg_b(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i8x32;
        /// The `llvm.x86.avx2.pavg.w` intrinsic; known as `__builtin_ia32_pavgw256` in GCC.
        #[link_name = "llvm.x86.avx2.pavg.w"]
        pub fn avx2_pavg_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.psad.bw` intrinsic; known as `__builtin_ia32_psadbw256` in GCC.
        #[link_name = "llvm.x86.avx2.psad.bw"]
        pub fn avx2_psad_bw(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i64x4;
        /// The `llvm.x86.avx512.mask.pmulu.dq.512` intrinsic; known as `__builtin_ia32_pmuludq512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pmulu.dq.512"]
        pub fn avx512_mask_pmulu_dq_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: ::simdty::i64x8, d: i8) -> ::simdty::i64x8;
        /// The `llvm.x86.avx512.mask.pmul.dq.512` intrinsic; known as `__builtin_ia32_pmuldq512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pmul.dq.512"]
        pub fn avx512_mask_pmul_dq_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: ::simdty::i64x8, d: i8) -> ::simdty::i64x8;
        /// The `llvm.x86.avx2.pmaxu.b` intrinsic; known as `__builtin_ia32_pmaxub256` in GCC.
        #[link_name = "llvm.x86.avx2.pmaxu.b"]
        pub fn avx2_pmaxu_b(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i8x32;
        /// The `llvm.x86.avx2.pmaxu.w` intrinsic; known as `__builtin_ia32_pmaxuw256` in GCC.
        #[link_name = "llvm.x86.avx2.pmaxu.w"]
        pub fn avx2_pmaxu_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.pmaxu.d` intrinsic; known as `__builtin_ia32_pmaxud256` in GCC.
        #[link_name = "llvm.x86.avx2.pmaxu.d"]
        pub fn avx2_pmaxu_d(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.pmaxs.b` intrinsic; known as `__builtin_ia32_pmaxsb256` in GCC.
        #[link_name = "llvm.x86.avx2.pmaxs.b"]
        pub fn avx2_pmaxs_b(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i8x32;
        /// The `llvm.x86.avx2.pmaxs.w` intrinsic; known as `__builtin_ia32_pmaxsw256` in GCC.
        #[link_name = "llvm.x86.avx2.pmaxs.w"]
        pub fn avx2_pmaxs_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.pmaxs.d` intrinsic; known as `__builtin_ia32_pmaxsd256` in GCC.
        #[link_name = "llvm.x86.avx2.pmaxs.d"]
        pub fn avx2_pmaxs_d(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.pminu.b` intrinsic; known as `__builtin_ia32_pminub256` in GCC.
        #[link_name = "llvm.x86.avx2.pminu.b"]
        pub fn avx2_pminu_b(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i8x32;
        /// The `llvm.x86.avx2.pminu.w` intrinsic; known as `__builtin_ia32_pminuw256` in GCC.
        #[link_name = "llvm.x86.avx2.pminu.w"]
        pub fn avx2_pminu_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.pminu.d` intrinsic; known as `__builtin_ia32_pminud256` in GCC.
        #[link_name = "llvm.x86.avx2.pminu.d"]
        pub fn avx2_pminu_d(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.pmins.b` intrinsic; known as `__builtin_ia32_pminsb256` in GCC.
        #[link_name = "llvm.x86.avx2.pmins.b"]
        pub fn avx2_pmins_b(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i8x32;
        /// The `llvm.x86.avx2.pmins.w` intrinsic; known as `__builtin_ia32_pminsw256` in GCC.
        #[link_name = "llvm.x86.avx2.pmins.w"]
        pub fn avx2_pmins_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.pmins.d` intrinsic; known as `__builtin_ia32_pminsd256` in GCC.
        #[link_name = "llvm.x86.avx2.pmins.d"]
        pub fn avx2_pmins_d(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        /// The `llvm.x86.avx512.mask.pmaxu.d.512` intrinsic; known as `__builtin_ia32_pmaxud512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pmaxu.d.512"]
        pub fn avx512_mask_pmaxu_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: ::simdty::i32x16, d: i16) -> ::simdty::i32x16;
        /// The `llvm.x86.avx512.mask.pmaxs.d.512` intrinsic; known as `__builtin_ia32_pmaxsd512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pmaxs.d.512"]
        pub fn avx512_mask_pmaxs_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: ::simdty::i32x16, d: i16) -> ::simdty::i32x16;
        /// The `llvm.x86.avx512.mask.pmaxu.q.512` intrinsic; known as `__builtin_ia32_pmaxuq512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pmaxu.q.512"]
        pub fn avx512_mask_pmaxu_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: ::simdty::i64x8, d: i8) -> ::simdty::i64x8;
        /// The `llvm.x86.avx512.mask.pmaxs.q.512` intrinsic; known as `__builtin_ia32_pmaxsq512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pmaxs.q.512"]
        pub fn avx512_mask_pmaxs_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: ::simdty::i64x8, d: i8) -> ::simdty::i64x8;
        /// The `llvm.x86.avx512.mask.pminu.d.512` intrinsic; known as `__builtin_ia32_pminud512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pminu.d.512"]
        pub fn avx512_mask_pminu_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: ::simdty::i32x16, d: i16) -> ::simdty::i32x16;
        /// The `llvm.x86.avx512.mask.pmins.d.512` intrinsic; known as `__builtin_ia32_pminsd512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pmins.d.512"]
        pub fn avx512_mask_pmins_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: ::simdty::i32x16, d: i16) -> ::simdty::i32x16;
        /// The `llvm.x86.avx512.mask.pminu.q.512` intrinsic; known as `__builtin_ia32_pminuq512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pminu.q.512"]
        pub fn avx512_mask_pminu_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: ::simdty::i64x8, d: i8) -> ::simdty::i64x8;
        /// The `llvm.x86.avx512.mask.pmins.q.512` intrinsic; known as `__builtin_ia32_pminsq512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pmins.q.512"]
        pub fn avx512_mask_pmins_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: ::simdty::i64x8, d: i8) -> ::simdty::i64x8;
        /// The `llvm.x86.avx2.psll.w` intrinsic; known as `__builtin_ia32_psllw256` in GCC.
        #[link_name = "llvm.x86.avx2.psll.w"]
        pub fn avx2_psll_w(a: ::simdty::i16x16, b: ::simdty::i16x8) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.psll.d` intrinsic; known as `__builtin_ia32_pslld256` in GCC.
        #[link_name = "llvm.x86.avx2.psll.d"]
        pub fn avx2_psll_d(a: ::simdty::i32x8, b: ::simdty::i32x4) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.psll.q` intrinsic; known as `__builtin_ia32_psllq256` in GCC.
        #[link_name = "llvm.x86.avx2.psll.q"]
        pub fn avx2_psll_q(a: ::simdty::i64x4, b: ::simdty::i64x2) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.psrl.w` intrinsic; known as `__builtin_ia32_psrlw256` in GCC.
        #[link_name = "llvm.x86.avx2.psrl.w"]
        pub fn avx2_psrl_w(a: ::simdty::i16x16, b: ::simdty::i16x8) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.psrl.d` intrinsic; known as `__builtin_ia32_psrld256` in GCC.
        #[link_name = "llvm.x86.avx2.psrl.d"]
        pub fn avx2_psrl_d(a: ::simdty::i32x8, b: ::simdty::i32x4) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.psrl.q` intrinsic; known as `__builtin_ia32_psrlq256` in GCC.
        #[link_name = "llvm.x86.avx2.psrl.q"]
        pub fn avx2_psrl_q(a: ::simdty::i64x4, b: ::simdty::i64x2) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.psra.w` intrinsic; known as `__builtin_ia32_psraw256` in GCC.
        #[link_name = "llvm.x86.avx2.psra.w"]
        pub fn avx2_psra_w(a: ::simdty::i16x16, b: ::simdty::i16x8) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.psra.d` intrinsic; known as `__builtin_ia32_psrad256` in GCC.
        #[link_name = "llvm.x86.avx2.psra.d"]
        pub fn avx2_psra_d(a: ::simdty::i32x8, b: ::simdty::i32x4) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.pslli.w` intrinsic; known as `__builtin_ia32_psllwi256` in GCC.
        #[link_name = "llvm.x86.avx2.pslli.w"]
        pub fn avx2_pslli_w(a: ::simdty::i16x16, b: i32) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.pslli.d` intrinsic; known as `__builtin_ia32_pslldi256` in GCC.
        #[link_name = "llvm.x86.avx2.pslli.d"]
        pub fn avx2_pslli_d(a: ::simdty::i32x8, b: i32) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.pslli.q` intrinsic; known as `__builtin_ia32_psllqi256` in GCC.
        #[link_name = "llvm.x86.avx2.pslli.q"]
        pub fn avx2_pslli_q(a: ::simdty::i64x4, b: i32) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.psrli.w` intrinsic; known as `__builtin_ia32_psrlwi256` in GCC.
        #[link_name = "llvm.x86.avx2.psrli.w"]
        pub fn avx2_psrli_w(a: ::simdty::i16x16, b: i32) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.psrli.d` intrinsic; known as `__builtin_ia32_psrldi256` in GCC.
        #[link_name = "llvm.x86.avx2.psrli.d"]
        pub fn avx2_psrli_d(a: ::simdty::i32x8, b: i32) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.psrli.q` intrinsic; known as `__builtin_ia32_psrlqi256` in GCC.
        #[link_name = "llvm.x86.avx2.psrli.q"]
        pub fn avx2_psrli_q(a: ::simdty::i64x4, b: i32) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.psrai.w` intrinsic; known as `__builtin_ia32_psrawi256` in GCC.
        #[link_name = "llvm.x86.avx2.psrai.w"]
        pub fn avx2_psrai_w(a: ::simdty::i16x16, b: i32) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.psrai.d` intrinsic; known as `__builtin_ia32_psradi256` in GCC.
        #[link_name = "llvm.x86.avx2.psrai.d"]
        pub fn avx2_psrai_d(a: ::simdty::i32x8, b: i32) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.psll.dq` intrinsic; known as `__builtin_ia32_pslldqi256` in GCC.
        #[link_name = "llvm.x86.avx2.psll.dq"]
        pub fn avx2_psll_dq(a: ::simdty::i64x4, b: i32) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.psrl.dq` intrinsic; known as `__builtin_ia32_psrldqi256` in GCC.
        #[link_name = "llvm.x86.avx2.psrl.dq"]
        pub fn avx2_psrl_dq(a: ::simdty::i64x4, b: i32) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.psll.dq.bs` intrinsic; known as `__builtin_ia32_pslldqi256_byteshift` in GCC.
        #[link_name = "llvm.x86.avx2.psll.dq.bs"]
        pub fn avx2_psll_dq_bs(a: ::simdty::i64x4, b: i32) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.psrl.dq.bs` intrinsic; known as `__builtin_ia32_psrldqi256_byteshift` in GCC.
        #[link_name = "llvm.x86.avx2.psrl.dq.bs"]
        pub fn avx2_psrl_dq_bs(a: ::simdty::i64x4, b: i32) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.packsswb` intrinsic; known as `__builtin_ia32_packsswb256` in GCC.
        #[link_name = "llvm.x86.avx2.packsswb"]
        pub fn avx2_packsswb(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i8x32;
        /// The `llvm.x86.avx2.packssdw` intrinsic; known as `__builtin_ia32_packssdw256` in GCC.
        #[link_name = "llvm.x86.avx2.packssdw"]
        pub fn avx2_packssdw(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.packuswb` intrinsic; known as `__builtin_ia32_packuswb256` in GCC.
        #[link_name = "llvm.x86.avx2.packuswb"]
        pub fn avx2_packuswb(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i8x32;
        /// The `llvm.x86.avx2.packusdw` intrinsic; known as `__builtin_ia32_packusdw256` in GCC.
        #[link_name = "llvm.x86.avx2.packusdw"]
        pub fn avx2_packusdw(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.pabs.b` intrinsic; known as `__builtin_ia32_pabsb256` in GCC.
        #[link_name = "llvm.x86.avx2.pabs.b"]
        pub fn avx2_pabs_b(a: ::simdty::i8x32) -> ::simdty::i8x32;
        /// The `llvm.x86.avx2.pabs.w` intrinsic; known as `__builtin_ia32_pabsw256` in GCC.
        #[link_name = "llvm.x86.avx2.pabs.w"]
        pub fn avx2_pabs_w(a: ::simdty::i16x16) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.pabs.d` intrinsic; known as `__builtin_ia32_pabsd256` in GCC.
        #[link_name = "llvm.x86.avx2.pabs.d"]
        pub fn avx2_pabs_d(a: ::simdty::i32x8) -> ::simdty::i32x8;
        /// The `llvm.x86.avx512.mask.pabs.d.512` intrinsic; known as `__builtin_ia32_pabsd512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pabs.d.512"]
        pub fn avx512_mask_pabs_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: i16) -> ::simdty::i32x16;
        /// The `llvm.x86.avx512.mask.pabs.q.512` intrinsic; known as `__builtin_ia32_pabsq512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pabs.q.512"]
        pub fn avx512_mask_pabs_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: i8) -> ::simdty::i64x8;
        /// The `llvm.x86.avx2.phadd.w` intrinsic; known as `__builtin_ia32_phaddw256` in GCC.
        #[link_name = "llvm.x86.avx2.phadd.w"]
        pub fn avx2_phadd_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.phadd.d` intrinsic; known as `__builtin_ia32_phaddd256` in GCC.
        #[link_name = "llvm.x86.avx2.phadd.d"]
        pub fn avx2_phadd_d(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.phadd.sw` intrinsic; known as `__builtin_ia32_phaddsw256` in GCC.
        #[link_name = "llvm.x86.avx2.phadd.sw"]
        pub fn avx2_phadd_sw(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.phsub.w` intrinsic; known as `__builtin_ia32_phsubw256` in GCC.
        #[link_name = "llvm.x86.avx2.phsub.w"]
        pub fn avx2_phsub_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.phsub.d` intrinsic; known as `__builtin_ia32_phsubd256` in GCC.
        #[link_name = "llvm.x86.avx2.phsub.d"]
        pub fn avx2_phsub_d(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.phsub.sw` intrinsic; known as `__builtin_ia32_phsubsw256` in GCC.
        #[link_name = "llvm.x86.avx2.phsub.sw"]
        pub fn avx2_phsub_sw(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.pmadd.ub.sw` intrinsic; known as `__builtin_ia32_pmaddubsw256` in GCC.
        #[link_name = "llvm.x86.avx2.pmadd.ub.sw"]
        pub fn avx2_pmadd_ub_sw(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.psign.b` intrinsic; known as `__builtin_ia32_psignb256` in GCC.
        #[link_name = "llvm.x86.avx2.psign.b"]
        pub fn avx2_psign_b(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i8x32;
        /// The `llvm.x86.avx2.psign.w` intrinsic; known as `__builtin_ia32_psignw256` in GCC.
        #[link_name = "llvm.x86.avx2.psign.w"]
        pub fn avx2_psign_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.psign.d` intrinsic; known as `__builtin_ia32_psignd256` in GCC.
        #[link_name = "llvm.x86.avx2.psign.d"]
        pub fn avx2_psign_d(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.pmul.hr.sw` intrinsic; known as `__builtin_ia32_pmulhrsw256` in GCC.
        #[link_name = "llvm.x86.avx2.pmul.hr.sw"]
        pub fn avx2_pmul_hr_sw(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.pmovsxbd` intrinsic; known as `__builtin_ia32_pmovsxbd256` in GCC.
        #[link_name = "llvm.x86.avx2.pmovsxbd"]
        pub fn avx2_pmovsxbd(a: ::simdty::i8x16) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.pmovsxbq` intrinsic; known as `__builtin_ia32_pmovsxbq256` in GCC.
        #[link_name = "llvm.x86.avx2.pmovsxbq"]
        pub fn avx2_pmovsxbq(a: ::simdty::i8x16) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.pmovsxbw` intrinsic; known as `__builtin_ia32_pmovsxbw256` in GCC.
        #[link_name = "llvm.x86.avx2.pmovsxbw"]
        pub fn avx2_pmovsxbw(a: ::simdty::i8x16) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.pmovsxdq` intrinsic; known as `__builtin_ia32_pmovsxdq256` in GCC.
        #[link_name = "llvm.x86.avx2.pmovsxdq"]
        pub fn avx2_pmovsxdq(a: ::simdty::i32x4) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.pmovsxwd` intrinsic; known as `__builtin_ia32_pmovsxwd256` in GCC.
        #[link_name = "llvm.x86.avx2.pmovsxwd"]
        pub fn avx2_pmovsxwd(a: ::simdty::i16x8) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.pmovsxwq` intrinsic; known as `__builtin_ia32_pmovsxwq256` in GCC.
        #[link_name = "llvm.x86.avx2.pmovsxwq"]
        pub fn avx2_pmovsxwq(a: ::simdty::i16x8) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.pmovzxbd` intrinsic; known as `__builtin_ia32_pmovzxbd256` in GCC.
        #[link_name = "llvm.x86.avx2.pmovzxbd"]
        pub fn avx2_pmovzxbd(a: ::simdty::i8x16) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.pmovzxbq` intrinsic; known as `__builtin_ia32_pmovzxbq256` in GCC.
        #[link_name = "llvm.x86.avx2.pmovzxbq"]
        pub fn avx2_pmovzxbq(a: ::simdty::i8x16) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.pmovzxbw` intrinsic; known as `__builtin_ia32_pmovzxbw256` in GCC.
        #[link_name = "llvm.x86.avx2.pmovzxbw"]
        pub fn avx2_pmovzxbw(a: ::simdty::i8x16) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.pmovzxdq` intrinsic; known as `__builtin_ia32_pmovzxdq256` in GCC.
        #[link_name = "llvm.x86.avx2.pmovzxdq"]
        pub fn avx2_pmovzxdq(a: ::simdty::i32x4) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.pmovzxwd` intrinsic; known as `__builtin_ia32_pmovzxwd256` in GCC.
        #[link_name = "llvm.x86.avx2.pmovzxwd"]
        pub fn avx2_pmovzxwd(a: ::simdty::i16x8) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.pmovzxwq` intrinsic; known as `__builtin_ia32_pmovzxwq256` in GCC.
        #[link_name = "llvm.x86.avx2.pmovzxwq"]
        pub fn avx2_pmovzxwq(a: ::simdty::i16x8) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.pblendvb` intrinsic; known as `__builtin_ia32_pblendvb256` in GCC.
        #[link_name = "llvm.x86.avx2.pblendvb"]
        pub fn avx2_pblendvb(a: ::simdty::i8x32, b: ::simdty::i8x32, c: ::simdty::i8x32) -> ::simdty::i8x32;
        /// The `llvm.x86.avx2.pblendw` intrinsic; known as `__builtin_ia32_pblendw256` in GCC.
        #[link_name = "llvm.x86.avx2.pblendw"]
        pub fn avx2_pblendw(a: ::simdty::i16x16, b: ::simdty::i16x16, c: i8) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.pblendd.128` intrinsic; known as `__builtin_ia32_pblendd128` in GCC.
        #[link_name = "llvm.x86.avx2.pblendd.128"]
        pub fn avx2_pblendd_128(a: ::simdty::i32x4, b: ::simdty::i32x4, c: i8) -> ::simdty::i32x4;
        /// The `llvm.x86.avx2.pblendd.256` intrinsic; known as `__builtin_ia32_pblendd256` in GCC.
        #[link_name = "llvm.x86.avx2.pblendd.256"]
        pub fn avx2_pblendd_256(a: ::simdty::i32x8, b: ::simdty::i32x8, c: i8) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.vbroadcast.ss.ps` intrinsic; known as `__builtin_ia32_vbroadcastss_ps` in GCC.
        #[link_name = "llvm.x86.avx2.vbroadcast.ss.ps"]
        pub fn avx2_vbroadcast_ss_ps(a: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.avx2.vbroadcast.sd.pd.256` intrinsic; known as `__builtin_ia32_vbroadcastsd_pd256` in GCC.
        #[link_name = "llvm.x86.avx2.vbroadcast.sd.pd.256"]
        pub fn avx2_vbroadcast_sd_pd_256(a: ::simdty::f64x2) -> ::simdty::f64x4;
        /// The `llvm.x86.avx2.vbroadcast.ss.ps.256` intrinsic; known as `__builtin_ia32_vbroadcastss_ps256` in GCC.
        #[link_name = "llvm.x86.avx2.vbroadcast.ss.ps.256"]
        pub fn avx2_vbroadcast_ss_ps_256(a: ::simdty::f32x4) -> ::simdty::f32x8;
        /// The `llvm.x86.avx2.vbroadcasti128` intrinsic.
        #[link_name = "llvm.x86.avx2.vbroadcasti128"]
        pub fn avx2_vbroadcasti128(a: *mut i8) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.pbroadcastb.128` intrinsic; known as `__builtin_ia32_pbroadcastb128` in GCC.
        #[link_name = "llvm.x86.avx2.pbroadcastb.128"]
        pub fn avx2_pbroadcastb_128(a: ::simdty::i8x16) -> ::simdty::i8x16;
        /// The `llvm.x86.avx2.pbroadcastb.256` intrinsic; known as `__builtin_ia32_pbroadcastb256` in GCC.
        #[link_name = "llvm.x86.avx2.pbroadcastb.256"]
        pub fn avx2_pbroadcastb_256(a: ::simdty::i8x16) -> ::simdty::i8x32;
        /// The `llvm.x86.avx2.pbroadcastw.128` intrinsic; known as `__builtin_ia32_pbroadcastw128` in GCC.
        #[link_name = "llvm.x86.avx2.pbroadcastw.128"]
        pub fn avx2_pbroadcastw_128(a: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.avx2.pbroadcastw.256` intrinsic; known as `__builtin_ia32_pbroadcastw256` in GCC.
        #[link_name = "llvm.x86.avx2.pbroadcastw.256"]
        pub fn avx2_pbroadcastw_256(a: ::simdty::i16x8) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.pbroadcastd.128` intrinsic; known as `__builtin_ia32_pbroadcastd128` in GCC.
        #[link_name = "llvm.x86.avx2.pbroadcastd.128"]
        pub fn avx2_pbroadcastd_128(a: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.avx2.pbroadcastd.256` intrinsic; known as `__builtin_ia32_pbroadcastd256` in GCC.
        #[link_name = "llvm.x86.avx2.pbroadcastd.256"]
        pub fn avx2_pbroadcastd_256(a: ::simdty::i32x4) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.pbroadcastq.128` intrinsic; known as `__builtin_ia32_pbroadcastq128` in GCC.
        #[link_name = "llvm.x86.avx2.pbroadcastq.128"]
        pub fn avx2_pbroadcastq_128(a: ::simdty::i64x2) -> ::simdty::i64x2;
        /// The `llvm.x86.avx2.pbroadcastq.256` intrinsic; known as `__builtin_ia32_pbroadcastq256` in GCC.
        #[link_name = "llvm.x86.avx2.pbroadcastq.256"]
        pub fn avx2_pbroadcastq_256(a: ::simdty::i64x2) -> ::simdty::i64x4;
        /// The `llvm.x86.avx512.mask.pbroadcast.d.gpr.512` intrinsic; known as `__builtin_ia32_pbroadcastd512_gpr_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pbroadcast.d.gpr.512"]
        pub fn avx512_mask_pbroadcast_d_gpr_512(a: i32, b: ::simdty::i32x16, c: i16) -> ::simdty::i32x16;
        /// The `llvm.x86.avx512.mask.pbroadcast.q.gpr.512` intrinsic; known as `__builtin_ia32_pbroadcastq512_gpr_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pbroadcast.q.gpr.512"]
        pub fn avx512_mask_pbroadcast_q_gpr_512(a: i64, b: ::simdty::i64x8, c: i8) -> ::simdty::i64x8;
        /// The `llvm.x86.avx512.mask.pbroadcast.q.mem.512` intrinsic; known as `__builtin_ia32_pbroadcastq512_mem_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pbroadcast.q.mem.512"]
        pub fn avx512_mask_pbroadcast_q_mem_512(a: i64, b: ::simdty::i64x8, c: i8) -> ::simdty::i64x8;
        /// The `llvm.x86.avx2.permd` intrinsic; known as `__builtin_ia32_permvarsi256` in GCC.
        #[link_name = "llvm.x86.avx2.permd"]
        pub fn avx2_permd(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.permps` intrinsic; known as `__builtin_ia32_permvarsf256` in GCC.
        #[link_name = "llvm.x86.avx2.permps"]
        pub fn avx2_permps(a: ::simdty::f32x8, b: ::simdty::f32x8) -> ::simdty::f32x8;
        /// The `llvm.x86.avx2.vperm2i128` intrinsic; known as `__builtin_ia32_permti256` in GCC.
        #[link_name = "llvm.x86.avx2.vperm2i128"]
        pub fn avx2_vperm2i128(a: ::simdty::i64x4, b: ::simdty::i64x4, c: i8) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.vextracti128` intrinsic; known as `__builtin_ia32_extract128i256` in GCC.
        #[link_name = "llvm.x86.avx2.vextracti128"]
        pub fn avx2_vextracti128(a: ::simdty::i64x4, b: i8) -> ::simdty::i64x2;
        /// The `llvm.x86.avx2.vinserti128` intrinsic; known as `__builtin_ia32_insert128i256` in GCC.
        #[link_name = "llvm.x86.avx2.vinserti128"]
        pub fn avx2_vinserti128(a: ::simdty::i64x4, b: ::simdty::i64x2, c: i8) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.maskload.d` intrinsic; known as `__builtin_ia32_maskloadd` in GCC.
        #[link_name = "llvm.x86.avx2.maskload.d"]
        pub fn avx2_maskload_d(a: *mut i8, b: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.avx2.maskload.q` intrinsic; known as `__builtin_ia32_maskloadq` in GCC.
        #[link_name = "llvm.x86.avx2.maskload.q"]
        pub fn avx2_maskload_q(a: *mut i8, b: ::simdty::i64x2) -> ::simdty::i64x2;
        /// The `llvm.x86.avx2.maskload.d.256` intrinsic; known as `__builtin_ia32_maskloadd256` in GCC.
        #[link_name = "llvm.x86.avx2.maskload.d.256"]
        pub fn avx2_maskload_d_256(a: *mut i8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.maskload.q.256` intrinsic; known as `__builtin_ia32_maskloadq256` in GCC.
        #[link_name = "llvm.x86.avx2.maskload.q.256"]
        pub fn avx2_maskload_q_256(a: *mut i8, b: ::simdty::i64x4) -> ::simdty::i64x4;
        /// The `llvm.x86.avx512.mask.loadu.d.512` intrinsic; known as `__builtin_ia32_loaddqusi512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.loadu.d.512"]
        pub fn avx512_mask_loadu_d_512(a: *mut i8, b: ::simdty::i32x16, c: i16) -> ::simdty::i32x16;
        /// The `llvm.x86.avx512.mask.loadu.q.512` intrinsic; known as `__builtin_ia32_loaddqudi512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.loadu.q.512"]
        pub fn avx512_mask_loadu_q_512(a: *mut i8, b: ::simdty::i64x8, c: i8) -> ::simdty::i64x8;
        /// The `llvm.x86.avx2.maskstore.d` intrinsic; known as `__builtin_ia32_maskstored` in GCC.
        #[link_name = "llvm.x86.avx2.maskstore.d"]
        pub fn avx2_maskstore_d(a: *mut i8, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ();
        /// The `llvm.x86.avx2.maskstore.q` intrinsic; known as `__builtin_ia32_maskstoreq` in GCC.
        #[link_name = "llvm.x86.avx2.maskstore.q"]
        pub fn avx2_maskstore_q(a: *mut i8, b: ::simdty::i64x2, c: ::simdty::i64x2) -> ();
        /// The `llvm.x86.avx2.maskstore.d.256` intrinsic; known as `__builtin_ia32_maskstored256` in GCC.
        #[link_name = "llvm.x86.avx2.maskstore.d.256"]
        pub fn avx2_maskstore_d_256(a: *mut i8, b: ::simdty::i32x8, c: ::simdty::i32x8) -> ();
        /// The `llvm.x86.avx2.maskstore.q.256` intrinsic; known as `__builtin_ia32_maskstoreq256` in GCC.
        #[link_name = "llvm.x86.avx2.maskstore.q.256"]
        pub fn avx2_maskstore_q_256(a: *mut i8, b: ::simdty::i64x4, c: ::simdty::i64x4) -> ();
        /// The `llvm.x86.avx512.mask.storeu.d.512` intrinsic; known as `__builtin_ia32_storedqusi512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.storeu.d.512"]
        pub fn avx512_mask_storeu_d_512(a: *mut i8, b: ::simdty::i32x16, c: i16) -> ();
        /// The `llvm.x86.avx512.mask.storeu.q.512` intrinsic; known as `__builtin_ia32_storedqudi512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.storeu.q.512"]
        pub fn avx512_mask_storeu_q_512(a: *mut i8, b: ::simdty::i64x8, c: i8) -> ();
        /// The `llvm.x86.avx2.psllv.d` intrinsic; known as `__builtin_ia32_psllv4si` in GCC.
        #[link_name = "llvm.x86.avx2.psllv.d"]
        pub fn avx2_psllv_d(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.avx2.psllv.d.256` intrinsic; known as `__builtin_ia32_psllv8si` in GCC.
        #[link_name = "llvm.x86.avx2.psllv.d.256"]
        pub fn avx2_psllv_d_256(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.psllv.q` intrinsic; known as `__builtin_ia32_psllv2di` in GCC.
        #[link_name = "llvm.x86.avx2.psllv.q"]
        pub fn avx2_psllv_q(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        /// The `llvm.x86.avx2.psllv.q.256` intrinsic; known as `__builtin_ia32_psllv4di` in GCC.
        #[link_name = "llvm.x86.avx2.psllv.q.256"]
        pub fn avx2_psllv_q_256(a: ::simdty::i64x4, b: ::simdty::i64x4) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.psrlv.d` intrinsic; known as `__builtin_ia32_psrlv4si` in GCC.
        #[link_name = "llvm.x86.avx2.psrlv.d"]
        pub fn avx2_psrlv_d(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.avx2.psrlv.d.256` intrinsic; known as `__builtin_ia32_psrlv8si` in GCC.
        #[link_name = "llvm.x86.avx2.psrlv.d.256"]
        pub fn avx2_psrlv_d_256(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.psrlv.q` intrinsic; known as `__builtin_ia32_psrlv2di` in GCC.
        #[link_name = "llvm.x86.avx2.psrlv.q"]
        pub fn avx2_psrlv_q(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        /// The `llvm.x86.avx2.psrlv.q.256` intrinsic; known as `__builtin_ia32_psrlv4di` in GCC.
        #[link_name = "llvm.x86.avx2.psrlv.q.256"]
        pub fn avx2_psrlv_q_256(a: ::simdty::i64x4, b: ::simdty::i64x4) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.psrav.d` intrinsic; known as `__builtin_ia32_psrav4si` in GCC.
        #[link_name = "llvm.x86.avx2.psrav.d"]
        pub fn avx2_psrav_d(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.avx2.psrav.d.256` intrinsic; known as `__builtin_ia32_psrav8si` in GCC.
        #[link_name = "llvm.x86.avx2.psrav.d.256"]
        pub fn avx2_psrav_d_256(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.gather.d.pd` intrinsic; known as `__builtin_ia32_gatherd_pd` in GCC.
        #[link_name = "llvm.x86.avx2.gather.d.pd"]
        pub fn avx2_gather_d_pd(a: ::simdty::f64x2, b: *mut i8, c: ::simdty::i32x4, d: ::simdty::f64x2, e: i8) -> ::simdty::f64x2;
        /// The `llvm.x86.avx2.gather.d.pd.256` intrinsic; known as `__builtin_ia32_gatherd_pd256` in GCC.
        #[link_name = "llvm.x86.avx2.gather.d.pd.256"]
        pub fn avx2_gather_d_pd_256(a: ::simdty::f64x4, b: *mut i8, c: ::simdty::i32x4, d: ::simdty::f64x4, e: i8) -> ::simdty::f64x4;
        /// The `llvm.x86.avx2.gather.q.pd` intrinsic; known as `__builtin_ia32_gatherq_pd` in GCC.
        #[link_name = "llvm.x86.avx2.gather.q.pd"]
        pub fn avx2_gather_q_pd(a: ::simdty::f64x2, b: *mut i8, c: ::simdty::i64x2, d: ::simdty::f64x2, e: i8) -> ::simdty::f64x2;
        /// The `llvm.x86.avx2.gather.q.pd.256` intrinsic; known as `__builtin_ia32_gatherq_pd256` in GCC.
        #[link_name = "llvm.x86.avx2.gather.q.pd.256"]
        pub fn avx2_gather_q_pd_256(a: ::simdty::f64x4, b: *mut i8, c: ::simdty::i64x4, d: ::simdty::f64x4, e: i8) -> ::simdty::f64x4;
        /// The `llvm.x86.avx2.gather.d.ps` intrinsic; known as `__builtin_ia32_gatherd_ps` in GCC.
        #[link_name = "llvm.x86.avx2.gather.d.ps"]
        pub fn avx2_gather_d_ps(a: ::simdty::f32x4, b: *mut i8, c: ::simdty::i32x4, d: ::simdty::f32x4, e: i8) -> ::simdty::f32x4;
        /// The `llvm.x86.avx2.gather.d.ps.256` intrinsic; known as `__builtin_ia32_gatherd_ps256` in GCC.
        #[link_name = "llvm.x86.avx2.gather.d.ps.256"]
        pub fn avx2_gather_d_ps_256(a: ::simdty::f32x8, b: *mut i8, c: ::simdty::i32x8, d: ::simdty::f32x8, e: i8) -> ::simdty::f32x8;
        /// The `llvm.x86.avx2.gather.q.ps` intrinsic; known as `__builtin_ia32_gatherq_ps` in GCC.
        #[link_name = "llvm.x86.avx2.gather.q.ps"]
        pub fn avx2_gather_q_ps(a: ::simdty::f32x4, b: *mut i8, c: ::simdty::i64x2, d: ::simdty::f32x4, e: i8) -> ::simdty::f32x4;
        /// The `llvm.x86.avx2.gather.q.ps.256` intrinsic; known as `__builtin_ia32_gatherq_ps256` in GCC.
        #[link_name = "llvm.x86.avx2.gather.q.ps.256"]
        pub fn avx2_gather_q_ps_256(a: ::simdty::f32x4, b: *mut i8, c: ::simdty::i64x4, d: ::simdty::f32x4, e: i8) -> ::simdty::f32x4;
        /// The `llvm.x86.avx2.gather.d.q` intrinsic; known as `__builtin_ia32_gatherd_q` in GCC.
        #[link_name = "llvm.x86.avx2.gather.d.q"]
        pub fn avx2_gather_d_q(a: ::simdty::i64x2, b: *mut i8, c: ::simdty::i32x4, d: ::simdty::i64x2, e: i8) -> ::simdty::i64x2;
        /// The `llvm.x86.avx2.gather.d.q.256` intrinsic; known as `__builtin_ia32_gatherd_q256` in GCC.
        #[link_name = "llvm.x86.avx2.gather.d.q.256"]
        pub fn avx2_gather_d_q_256(a: ::simdty::i64x4, b: *mut i8, c: ::simdty::i32x4, d: ::simdty::i64x4, e: i8) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.gather.q.q` intrinsic; known as `__builtin_ia32_gatherq_q` in GCC.
        #[link_name = "llvm.x86.avx2.gather.q.q"]
        pub fn avx2_gather_q_q(a: ::simdty::i64x2, b: *mut i8, c: ::simdty::i64x2, d: ::simdty::i64x2, e: i8) -> ::simdty::i64x2;
        /// The `llvm.x86.avx2.gather.q.q.256` intrinsic; known as `__builtin_ia32_gatherq_q256` in GCC.
        #[link_name = "llvm.x86.avx2.gather.q.q.256"]
        pub fn avx2_gather_q_q_256(a: ::simdty::i64x4, b: *mut i8, c: ::simdty::i64x4, d: ::simdty::i64x4, e: i8) -> ::simdty::i64x4;
        /// The `llvm.x86.avx2.gather.d.d` intrinsic; known as `__builtin_ia32_gatherd_d` in GCC.
        #[link_name = "llvm.x86.avx2.gather.d.d"]
        pub fn avx2_gather_d_d(a: ::simdty::i32x4, b: *mut i8, c: ::simdty::i32x4, d: ::simdty::i32x4, e: i8) -> ::simdty::i32x4;
        /// The `llvm.x86.avx2.gather.d.d.256` intrinsic; known as `__builtin_ia32_gatherd_d256` in GCC.
        #[link_name = "llvm.x86.avx2.gather.d.d.256"]
        pub fn avx2_gather_d_d_256(a: ::simdty::i32x8, b: *mut i8, c: ::simdty::i32x8, d: ::simdty::i32x8, e: i8) -> ::simdty::i32x8;
        /// The `llvm.x86.avx2.gather.q.d` intrinsic; known as `__builtin_ia32_gatherq_d` in GCC.
        #[link_name = "llvm.x86.avx2.gather.q.d"]
        pub fn avx2_gather_q_d(a: ::simdty::i32x4, b: *mut i8, c: ::simdty::i64x2, d: ::simdty::i32x4, e: i8) -> ::simdty::i32x4;
        /// The `llvm.x86.avx2.gather.q.d.256` intrinsic; known as `__builtin_ia32_gatherq_d256` in GCC.
        #[link_name = "llvm.x86.avx2.gather.q.d.256"]
        pub fn avx2_gather_q_d_256(a: ::simdty::i32x4, b: *mut i8, c: ::simdty::i64x4, d: ::simdty::i32x4, e: i8) -> ::simdty::i32x4;
        /// The `llvm.x86.avx2.pmovmskb` intrinsic; known as `__builtin_ia32_pmovmskb256` in GCC.
        #[link_name = "llvm.x86.avx2.pmovmskb"]
        pub fn avx2_pmovmskb(a: ::simdty::i8x32) -> i32;
        /// The `llvm.x86.avx2.pshuf.b` intrinsic; known as `__builtin_ia32_pshufb256` in GCC.
        #[link_name = "llvm.x86.avx2.pshuf.b"]
        pub fn avx2_pshuf_b(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i8x32;
        /// The `llvm.x86.avx2.mpsadbw` intrinsic; known as `__builtin_ia32_mpsadbw256` in GCC.
        #[link_name = "llvm.x86.avx2.mpsadbw"]
        pub fn avx2_mpsadbw(a: ::simdty::i8x32, b: ::simdty::i8x32, c: i8) -> ::simdty::i16x16;
        /// The `llvm.x86.avx2.movntdqa` intrinsic; known as `__builtin_ia32_movntdqa256` in GCC.
        #[link_name = "llvm.x86.avx2.movntdqa"]
        pub fn avx2_movntdqa(a: *mut i8) -> ::simdty::i64x4;
        /// The `llvm.x86.fma.vfmadd.ss` intrinsic; known as `__builtin_ia32_vfmaddss` in GCC.
        #[link_name = "llvm.x86.fma.vfmadd.ss"]
        pub fn fma_vfmadd_ss(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.fma.vfmadd.sd` intrinsic; known as `__builtin_ia32_vfmaddsd` in GCC.
        #[link_name = "llvm.x86.fma.vfmadd.sd"]
        pub fn fma_vfmadd_sd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.fma.vfmadd.ps` intrinsic; known as `__builtin_ia32_vfmaddps` in GCC.
        #[link_name = "llvm.x86.fma.vfmadd.ps"]
        pub fn fma_vfmadd_ps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.fma.vfmadd.pd` intrinsic; known as `__builtin_ia32_vfmaddpd` in GCC.
        #[link_name = "llvm.x86.fma.vfmadd.pd"]
        pub fn fma_vfmadd_pd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.fma.vfmadd.ps.256` intrinsic; known as `__builtin_ia32_vfmaddps256` in GCC.
        #[link_name = "llvm.x86.fma.vfmadd.ps.256"]
        pub fn fma_vfmadd_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: ::simdty::f32x8) -> ::simdty::f32x8;
        /// The `llvm.x86.fma.vfmadd.pd.256` intrinsic; known as `__builtin_ia32_vfmaddpd256` in GCC.
        #[link_name = "llvm.x86.fma.vfmadd.pd.256"]
        pub fn fma_vfmadd_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4, c: ::simdty::f64x4) -> ::simdty::f64x4;
        /// The `llvm.x86.fma.mask.vfmadd.ps.512` intrinsic; known as `__builtin_ia32_vfmaddps512_mask` in GCC.
        #[link_name = "llvm.x86.fma.mask.vfmadd.ps.512"]
        pub fn fma_mask_vfmadd_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: ::simdty::f32x16, d: i16, e: i32) -> ::simdty::f32x16;
        /// The `llvm.x86.fma.mask.vfmadd.pd.512` intrinsic; known as `__builtin_ia32_vfmaddpd512_mask` in GCC.
        #[link_name = "llvm.x86.fma.mask.vfmadd.pd.512"]
        pub fn fma_mask_vfmadd_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: ::simdty::f64x8, d: i8, e: i32) -> ::simdty::f64x8;
        /// The `llvm.x86.fma.vfmsub.ss` intrinsic; known as `__builtin_ia32_vfmsubss` in GCC.
        #[link_name = "llvm.x86.fma.vfmsub.ss"]
        pub fn fma_vfmsub_ss(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.fma.vfmsub.sd` intrinsic; known as `__builtin_ia32_vfmsubsd` in GCC.
        #[link_name = "llvm.x86.fma.vfmsub.sd"]
        pub fn fma_vfmsub_sd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.fma.vfmsub.ps` intrinsic; known as `__builtin_ia32_vfmsubps` in GCC.
        #[link_name = "llvm.x86.fma.vfmsub.ps"]
        pub fn fma_vfmsub_ps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.fma.vfmsub.pd` intrinsic; known as `__builtin_ia32_vfmsubpd` in GCC.
        #[link_name = "llvm.x86.fma.vfmsub.pd"]
        pub fn fma_vfmsub_pd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.fma.vfmsub.ps.256` intrinsic; known as `__builtin_ia32_vfmsubps256` in GCC.
        #[link_name = "llvm.x86.fma.vfmsub.ps.256"]
        pub fn fma_vfmsub_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: ::simdty::f32x8) -> ::simdty::f32x8;
        /// The `llvm.x86.fma.vfmsub.pd.256` intrinsic; known as `__builtin_ia32_vfmsubpd256` in GCC.
        #[link_name = "llvm.x86.fma.vfmsub.pd.256"]
        pub fn fma_vfmsub_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4, c: ::simdty::f64x4) -> ::simdty::f64x4;
        /// The `llvm.x86.fma.mask.vfmsub.ps.512` intrinsic; known as `__builtin_ia32_vfmsubps512_mask` in GCC.
        #[link_name = "llvm.x86.fma.mask.vfmsub.ps.512"]
        pub fn fma_mask_vfmsub_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: ::simdty::f32x16, d: i16, e: i32) -> ::simdty::f32x16;
        /// The `llvm.x86.fma.mask.vfmsub.pd.512` intrinsic; known as `__builtin_ia32_vfmsubpd512_mask` in GCC.
        #[link_name = "llvm.x86.fma.mask.vfmsub.pd.512"]
        pub fn fma_mask_vfmsub_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: ::simdty::f64x8, d: i8, e: i32) -> ::simdty::f64x8;
        /// The `llvm.x86.fma.vfnmadd.ss` intrinsic; known as `__builtin_ia32_vfnmaddss` in GCC.
        #[link_name = "llvm.x86.fma.vfnmadd.ss"]
        pub fn fma_vfnmadd_ss(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.fma.vfnmadd.sd` intrinsic; known as `__builtin_ia32_vfnmaddsd` in GCC.
        #[link_name = "llvm.x86.fma.vfnmadd.sd"]
        pub fn fma_vfnmadd_sd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.fma.vfnmadd.ps` intrinsic; known as `__builtin_ia32_vfnmaddps` in GCC.
        #[link_name = "llvm.x86.fma.vfnmadd.ps"]
        pub fn fma_vfnmadd_ps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.fma.vfnmadd.pd` intrinsic; known as `__builtin_ia32_vfnmaddpd` in GCC.
        #[link_name = "llvm.x86.fma.vfnmadd.pd"]
        pub fn fma_vfnmadd_pd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.fma.vfnmadd.ps.256` intrinsic; known as `__builtin_ia32_vfnmaddps256` in GCC.
        #[link_name = "llvm.x86.fma.vfnmadd.ps.256"]
        pub fn fma_vfnmadd_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: ::simdty::f32x8) -> ::simdty::f32x8;
        /// The `llvm.x86.fma.vfnmadd.pd.256` intrinsic; known as `__builtin_ia32_vfnmaddpd256` in GCC.
        #[link_name = "llvm.x86.fma.vfnmadd.pd.256"]
        pub fn fma_vfnmadd_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4, c: ::simdty::f64x4) -> ::simdty::f64x4;
        /// The `llvm.x86.fma.mask.vfnmadd.ps.512` intrinsic; known as `__builtin_ia32_vfnmaddps512_mask` in GCC.
        #[link_name = "llvm.x86.fma.mask.vfnmadd.ps.512"]
        pub fn fma_mask_vfnmadd_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: ::simdty::f32x16, d: i16, e: i32) -> ::simdty::f32x16;
        /// The `llvm.x86.fma.mask.vfnmadd.pd.512` intrinsic; known as `__builtin_ia32_vfnmaddpd512_mask` in GCC.
        #[link_name = "llvm.x86.fma.mask.vfnmadd.pd.512"]
        pub fn fma_mask_vfnmadd_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: ::simdty::f64x8, d: i8, e: i32) -> ::simdty::f64x8;
        /// The `llvm.x86.fma.vfnmsub.ss` intrinsic; known as `__builtin_ia32_vfnmsubss` in GCC.
        #[link_name = "llvm.x86.fma.vfnmsub.ss"]
        pub fn fma_vfnmsub_ss(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.fma.vfnmsub.sd` intrinsic; known as `__builtin_ia32_vfnmsubsd` in GCC.
        #[link_name = "llvm.x86.fma.vfnmsub.sd"]
        pub fn fma_vfnmsub_sd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.fma.vfnmsub.ps` intrinsic; known as `__builtin_ia32_vfnmsubps` in GCC.
        #[link_name = "llvm.x86.fma.vfnmsub.ps"]
        pub fn fma_vfnmsub_ps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.fma.vfnmsub.pd` intrinsic; known as `__builtin_ia32_vfnmsubpd` in GCC.
        #[link_name = "llvm.x86.fma.vfnmsub.pd"]
        pub fn fma_vfnmsub_pd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.fma.vfnmsub.ps.256` intrinsic; known as `__builtin_ia32_vfnmsubps256` in GCC.
        #[link_name = "llvm.x86.fma.vfnmsub.ps.256"]
        pub fn fma_vfnmsub_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: ::simdty::f32x8) -> ::simdty::f32x8;
        /// The `llvm.x86.fma.vfnmsub.pd.256` intrinsic; known as `__builtin_ia32_vfnmsubpd256` in GCC.
        #[link_name = "llvm.x86.fma.vfnmsub.pd.256"]
        pub fn fma_vfnmsub_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4, c: ::simdty::f64x4) -> ::simdty::f64x4;
        /// The `llvm.x86.fma.mask.vfnmsub.ps.512` intrinsic; known as `__builtin_ia32_vfnmsubps512_mask` in GCC.
        #[link_name = "llvm.x86.fma.mask.vfnmsub.ps.512"]
        pub fn fma_mask_vfnmsub_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: ::simdty::f32x16, d: i16, e: i32) -> ::simdty::f32x16;
        /// The `llvm.x86.fma.mask.vfnmsub.pd.512` intrinsic; known as `__builtin_ia32_vfnmsubpd512_mask` in GCC.
        #[link_name = "llvm.x86.fma.mask.vfnmsub.pd.512"]
        pub fn fma_mask_vfnmsub_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: ::simdty::f64x8, d: i8, e: i32) -> ::simdty::f64x8;
        /// The `llvm.x86.fma.vfmaddsub.ps` intrinsic; known as `__builtin_ia32_vfmaddsubps` in GCC.
        #[link_name = "llvm.x86.fma.vfmaddsub.ps"]
        pub fn fma_vfmaddsub_ps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.fma.vfmaddsub.pd` intrinsic; known as `__builtin_ia32_vfmaddsubpd` in GCC.
        #[link_name = "llvm.x86.fma.vfmaddsub.pd"]
        pub fn fma_vfmaddsub_pd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.fma.vfmaddsub.ps.256` intrinsic; known as `__builtin_ia32_vfmaddsubps256` in GCC.
        #[link_name = "llvm.x86.fma.vfmaddsub.ps.256"]
        pub fn fma_vfmaddsub_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: ::simdty::f32x8) -> ::simdty::f32x8;
        /// The `llvm.x86.fma.vfmaddsub.pd.256` intrinsic; known as `__builtin_ia32_vfmaddsubpd256` in GCC.
        #[link_name = "llvm.x86.fma.vfmaddsub.pd.256"]
        pub fn fma_vfmaddsub_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4, c: ::simdty::f64x4) -> ::simdty::f64x4;
        /// The `llvm.x86.fma.mask.vfmaddsub.ps.512` intrinsic; known as `__builtin_ia32_vfmaddsubps512_mask` in GCC.
        #[link_name = "llvm.x86.fma.mask.vfmaddsub.ps.512"]
        pub fn fma_mask_vfmaddsub_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: ::simdty::f32x16, d: i16, e: i32) -> ::simdty::f32x16;
        /// The `llvm.x86.fma.mask.vfmaddsub.pd.512` intrinsic; known as `__builtin_ia32_vfmaddsubpd512_mask` in GCC.
        #[link_name = "llvm.x86.fma.mask.vfmaddsub.pd.512"]
        pub fn fma_mask_vfmaddsub_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: ::simdty::f64x8, d: i8, e: i32) -> ::simdty::f64x8;
        /// The `llvm.x86.fma.vfmsubadd.ps` intrinsic; known as `__builtin_ia32_vfmsubaddps` in GCC.
        #[link_name = "llvm.x86.fma.vfmsubadd.ps"]
        pub fn fma_vfmsubadd_ps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.fma.vfmsubadd.pd` intrinsic; known as `__builtin_ia32_vfmsubaddpd` in GCC.
        #[link_name = "llvm.x86.fma.vfmsubadd.pd"]
        pub fn fma_vfmsubadd_pd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.fma.vfmsubadd.ps.256` intrinsic; known as `__builtin_ia32_vfmsubaddps256` in GCC.
        #[link_name = "llvm.x86.fma.vfmsubadd.ps.256"]
        pub fn fma_vfmsubadd_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: ::simdty::f32x8) -> ::simdty::f32x8;
        /// The `llvm.x86.fma.vfmsubadd.pd.256` intrinsic; known as `__builtin_ia32_vfmsubaddpd256` in GCC.
        #[link_name = "llvm.x86.fma.vfmsubadd.pd.256"]
        pub fn fma_vfmsubadd_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4, c: ::simdty::f64x4) -> ::simdty::f64x4;
        /// The `llvm.x86.fma.mask.vfmsubadd.ps.512` intrinsic; known as `__builtin_ia32_vfmsubaddps512_mask` in GCC.
        #[link_name = "llvm.x86.fma.mask.vfmsubadd.ps.512"]
        pub fn fma_mask_vfmsubadd_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: ::simdty::f32x16, d: i16, e: i32) -> ::simdty::f32x16;
        /// The `llvm.x86.fma.mask.vfmsubadd.pd.512` intrinsic; known as `__builtin_ia32_vfmsubaddpd512_mask` in GCC.
        #[link_name = "llvm.x86.fma.mask.vfmsubadd.pd.512"]
        pub fn fma_mask_vfmsubadd_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: ::simdty::f64x8, d: i8, e: i32) -> ::simdty::f64x8;
        /// The `llvm.x86.xop.vpermil2pd` intrinsic; known as `__builtin_ia32_vpermil2pd` in GCC.
        #[link_name = "llvm.x86.xop.vpermil2pd"]
        pub fn xop_vpermil2pd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2, d: i8) -> ::simdty::f64x2;
        /// The `llvm.x86.xop.vpermil2pd.256` intrinsic; known as `__builtin_ia32_vpermil2pd256` in GCC.
        #[link_name = "llvm.x86.xop.vpermil2pd.256"]
        pub fn xop_vpermil2pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4, c: ::simdty::f64x4, d: i8) -> ::simdty::f64x4;
        /// The `llvm.x86.xop.vpermil2ps` intrinsic; known as `__builtin_ia32_vpermil2ps` in GCC.
        #[link_name = "llvm.x86.xop.vpermil2ps"]
        pub fn xop_vpermil2ps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4, d: i8) -> ::simdty::f32x4;
        /// The `llvm.x86.xop.vpermil2ps.256` intrinsic; known as `__builtin_ia32_vpermil2ps256` in GCC.
        #[link_name = "llvm.x86.xop.vpermil2ps.256"]
        pub fn xop_vpermil2ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: ::simdty::f32x8, d: i8) -> ::simdty::f32x8;
        /// The `llvm.x86.xop.vfrcz.pd` intrinsic; known as `__builtin_ia32_vfrczpd` in GCC.
        #[link_name = "llvm.x86.xop.vfrcz.pd"]
        pub fn xop_vfrcz_pd(a: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.xop.vfrcz.ps` intrinsic; known as `__builtin_ia32_vfrczps` in GCC.
        #[link_name = "llvm.x86.xop.vfrcz.ps"]
        pub fn xop_vfrcz_ps(a: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.xop.vfrcz.sd` intrinsic; known as `__builtin_ia32_vfrczsd` in GCC.
        #[link_name = "llvm.x86.xop.vfrcz.sd"]
        pub fn xop_vfrcz_sd(a: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.xop.vfrcz.ss` intrinsic; known as `__builtin_ia32_vfrczss` in GCC.
        #[link_name = "llvm.x86.xop.vfrcz.ss"]
        pub fn xop_vfrcz_ss(a: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.xop.vfrcz.pd.256` intrinsic; known as `__builtin_ia32_vfrczpd256` in GCC.
        #[link_name = "llvm.x86.xop.vfrcz.pd.256"]
        pub fn xop_vfrcz_pd_256(a: ::simdty::f64x4) -> ::simdty::f64x4;
        /// The `llvm.x86.xop.vfrcz.ps.256` intrinsic; known as `__builtin_ia32_vfrczps256` in GCC.
        #[link_name = "llvm.x86.xop.vfrcz.ps.256"]
        pub fn xop_vfrcz_ps_256(a: ::simdty::f32x8) -> ::simdty::f32x8;
        /// The `llvm.x86.xop.vpcmov` intrinsic; known as `__builtin_ia32_vpcmov` in GCC.
        #[link_name = "llvm.x86.xop.vpcmov"]
        pub fn xop_vpcmov(a: ::simdty::i64x2, b: ::simdty::i64x2, c: ::simdty::i64x2) -> ::simdty::i64x2;
        /// The `llvm.x86.xop.vpcmov.256` intrinsic; known as `__builtin_ia32_vpcmov_256` in GCC.
        #[link_name = "llvm.x86.xop.vpcmov.256"]
        pub fn xop_vpcmov_256(a: ::simdty::i64x4, b: ::simdty::i64x4, c: ::simdty::i64x4) -> ::simdty::i64x4;
        /// The `llvm.x86.xop.vpcomb` intrinsic; known as `__builtin_ia32_vpcomb` in GCC.
        #[link_name = "llvm.x86.xop.vpcomb"]
        pub fn xop_vpcomb(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i8) -> ::simdty::i8x16;
        /// The `llvm.x86.xop.vpcomw` intrinsic; known as `__builtin_ia32_vpcomw` in GCC.
        #[link_name = "llvm.x86.xop.vpcomw"]
        pub fn xop_vpcomw(a: ::simdty::i16x8, b: ::simdty::i16x8, c: i8) -> ::simdty::i16x8;
        /// The `llvm.x86.xop.vpcomd` intrinsic; known as `__builtin_ia32_vpcomd` in GCC.
        #[link_name = "llvm.x86.xop.vpcomd"]
        pub fn xop_vpcomd(a: ::simdty::i32x4, b: ::simdty::i32x4, c: i8) -> ::simdty::i32x4;
        /// The `llvm.x86.xop.vpcomq` intrinsic; known as `__builtin_ia32_vpcomq` in GCC.
        #[link_name = "llvm.x86.xop.vpcomq"]
        pub fn xop_vpcomq(a: ::simdty::i64x2, b: ::simdty::i64x2, c: i8) -> ::simdty::i64x2;
        /// The `llvm.x86.xop.vpcomub` intrinsic; known as `__builtin_ia32_vpcomub` in GCC.
        #[link_name = "llvm.x86.xop.vpcomub"]
        pub fn xop_vpcomub(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i8) -> ::simdty::i8x16;
        /// The `llvm.x86.xop.vpcomuw` intrinsic; known as `__builtin_ia32_vpcomuw` in GCC.
        #[link_name = "llvm.x86.xop.vpcomuw"]
        pub fn xop_vpcomuw(a: ::simdty::i16x8, b: ::simdty::i16x8, c: i8) -> ::simdty::i16x8;
        /// The `llvm.x86.xop.vpcomud` intrinsic; known as `__builtin_ia32_vpcomud` in GCC.
        #[link_name = "llvm.x86.xop.vpcomud"]
        pub fn xop_vpcomud(a: ::simdty::i32x4, b: ::simdty::i32x4, c: i8) -> ::simdty::i32x4;
        /// The `llvm.x86.xop.vpcomuq` intrinsic; known as `__builtin_ia32_vpcomuq` in GCC.
        #[link_name = "llvm.x86.xop.vpcomuq"]
        pub fn xop_vpcomuq(a: ::simdty::i64x2, b: ::simdty::i64x2, c: i8) -> ::simdty::i64x2;
        /// The `llvm.x86.xop.vphaddbd` intrinsic; known as `__builtin_ia32_vphaddbd` in GCC.
        #[link_name = "llvm.x86.xop.vphaddbd"]
        pub fn xop_vphaddbd(a: ::simdty::i8x16) -> ::simdty::i32x4;
        /// The `llvm.x86.xop.vphaddbq` intrinsic; known as `__builtin_ia32_vphaddbq` in GCC.
        #[link_name = "llvm.x86.xop.vphaddbq"]
        pub fn xop_vphaddbq(a: ::simdty::i8x16) -> ::simdty::i64x2;
        /// The `llvm.x86.xop.vphaddbw` intrinsic; known as `__builtin_ia32_vphaddbw` in GCC.
        #[link_name = "llvm.x86.xop.vphaddbw"]
        pub fn xop_vphaddbw(a: ::simdty::i8x16) -> ::simdty::i16x8;
        /// The `llvm.x86.xop.vphadddq` intrinsic; known as `__builtin_ia32_vphadddq` in GCC.
        #[link_name = "llvm.x86.xop.vphadddq"]
        pub fn xop_vphadddq(a: ::simdty::i32x4) -> ::simdty::i64x2;
        /// The `llvm.x86.xop.vphaddubd` intrinsic; known as `__builtin_ia32_vphaddubd` in GCC.
        #[link_name = "llvm.x86.xop.vphaddubd"]
        pub fn xop_vphaddubd(a: ::simdty::i8x16) -> ::simdty::i32x4;
        /// The `llvm.x86.xop.vphaddubq` intrinsic; known as `__builtin_ia32_vphaddubq` in GCC.
        #[link_name = "llvm.x86.xop.vphaddubq"]
        pub fn xop_vphaddubq(a: ::simdty::i8x16) -> ::simdty::i64x2;
        /// The `llvm.x86.xop.vphaddubw` intrinsic; known as `__builtin_ia32_vphaddubw` in GCC.
        #[link_name = "llvm.x86.xop.vphaddubw"]
        pub fn xop_vphaddubw(a: ::simdty::i8x16) -> ::simdty::i16x8;
        /// The `llvm.x86.xop.vphaddudq` intrinsic; known as `__builtin_ia32_vphaddudq` in GCC.
        #[link_name = "llvm.x86.xop.vphaddudq"]
        pub fn xop_vphaddudq(a: ::simdty::i32x4) -> ::simdty::i64x2;
        /// The `llvm.x86.xop.vphadduwd` intrinsic; known as `__builtin_ia32_vphadduwd` in GCC.
        #[link_name = "llvm.x86.xop.vphadduwd"]
        pub fn xop_vphadduwd(a: ::simdty::i16x8) -> ::simdty::i32x4;
        /// The `llvm.x86.xop.vphadduwq` intrinsic; known as `__builtin_ia32_vphadduwq` in GCC.
        #[link_name = "llvm.x86.xop.vphadduwq"]
        pub fn xop_vphadduwq(a: ::simdty::i16x8) -> ::simdty::i64x2;
        /// The `llvm.x86.xop.vphaddwd` intrinsic; known as `__builtin_ia32_vphaddwd` in GCC.
        #[link_name = "llvm.x86.xop.vphaddwd"]
        pub fn xop_vphaddwd(a: ::simdty::i16x8) -> ::simdty::i32x4;
        /// The `llvm.x86.xop.vphaddwq` intrinsic; known as `__builtin_ia32_vphaddwq` in GCC.
        #[link_name = "llvm.x86.xop.vphaddwq"]
        pub fn xop_vphaddwq(a: ::simdty::i16x8) -> ::simdty::i64x2;
        /// The `llvm.x86.xop.vphsubbw` intrinsic; known as `__builtin_ia32_vphsubbw` in GCC.
        #[link_name = "llvm.x86.xop.vphsubbw"]
        pub fn xop_vphsubbw(a: ::simdty::i8x16) -> ::simdty::i16x8;
        /// The `llvm.x86.xop.vphsubdq` intrinsic; known as `__builtin_ia32_vphsubdq` in GCC.
        #[link_name = "llvm.x86.xop.vphsubdq"]
        pub fn xop_vphsubdq(a: ::simdty::i32x4) -> ::simdty::i64x2;
        /// The `llvm.x86.xop.vphsubwd` intrinsic; known as `__builtin_ia32_vphsubwd` in GCC.
        #[link_name = "llvm.x86.xop.vphsubwd"]
        pub fn xop_vphsubwd(a: ::simdty::i16x8) -> ::simdty::i32x4;
        /// The `llvm.x86.xop.vpmacsdd` intrinsic; known as `__builtin_ia32_vpmacsdd` in GCC.
        #[link_name = "llvm.x86.xop.vpmacsdd"]
        pub fn xop_vpmacsdd(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.xop.vpmacsdqh` intrinsic; known as `__builtin_ia32_vpmacsdqh` in GCC.
        #[link_name = "llvm.x86.xop.vpmacsdqh"]
        pub fn xop_vpmacsdqh(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i64x2) -> ::simdty::i64x2;
        /// The `llvm.x86.xop.vpmacsdql` intrinsic; known as `__builtin_ia32_vpmacsdql` in GCC.
        #[link_name = "llvm.x86.xop.vpmacsdql"]
        pub fn xop_vpmacsdql(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i64x2) -> ::simdty::i64x2;
        /// The `llvm.x86.xop.vpmacssdd` intrinsic; known as `__builtin_ia32_vpmacssdd` in GCC.
        #[link_name = "llvm.x86.xop.vpmacssdd"]
        pub fn xop_vpmacssdd(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.xop.vpmacssdqh` intrinsic; known as `__builtin_ia32_vpmacssdqh` in GCC.
        #[link_name = "llvm.x86.xop.vpmacssdqh"]
        pub fn xop_vpmacssdqh(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i64x2) -> ::simdty::i64x2;
        /// The `llvm.x86.xop.vpmacssdql` intrinsic; known as `__builtin_ia32_vpmacssdql` in GCC.
        #[link_name = "llvm.x86.xop.vpmacssdql"]
        pub fn xop_vpmacssdql(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i64x2) -> ::simdty::i64x2;
        /// The `llvm.x86.xop.vpmacsswd` intrinsic; known as `__builtin_ia32_vpmacsswd` in GCC.
        #[link_name = "llvm.x86.xop.vpmacsswd"]
        pub fn xop_vpmacsswd(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.xop.vpmacssww` intrinsic; known as `__builtin_ia32_vpmacssww` in GCC.
        #[link_name = "llvm.x86.xop.vpmacssww"]
        pub fn xop_vpmacssww(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.xop.vpmacswd` intrinsic; known as `__builtin_ia32_vpmacswd` in GCC.
        #[link_name = "llvm.x86.xop.vpmacswd"]
        pub fn xop_vpmacswd(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.xop.vpmacsww` intrinsic; known as `__builtin_ia32_vpmacsww` in GCC.
        #[link_name = "llvm.x86.xop.vpmacsww"]
        pub fn xop_vpmacsww(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.xop.vpmadcsswd` intrinsic; known as `__builtin_ia32_vpmadcsswd` in GCC.
        #[link_name = "llvm.x86.xop.vpmadcsswd"]
        pub fn xop_vpmadcsswd(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.xop.vpmadcswd` intrinsic; known as `__builtin_ia32_vpmadcswd` in GCC.
        #[link_name = "llvm.x86.xop.vpmadcswd"]
        pub fn xop_vpmadcswd(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.xop.vpperm` intrinsic; known as `__builtin_ia32_vpperm` in GCC.
        #[link_name = "llvm.x86.xop.vpperm"]
        pub fn xop_vpperm(a: ::simdty::i8x16, b: ::simdty::i8x16, c: ::simdty::i8x16) -> ::simdty::i8x16;
        /// The `llvm.x86.xop.vprotb` intrinsic; known as `__builtin_ia32_vprotb` in GCC.
        #[link_name = "llvm.x86.xop.vprotb"]
        pub fn xop_vprotb(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        /// The `llvm.x86.xop.vprotd` intrinsic; known as `__builtin_ia32_vprotd` in GCC.
        #[link_name = "llvm.x86.xop.vprotd"]
        pub fn xop_vprotd(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.xop.vprotq` intrinsic; known as `__builtin_ia32_vprotq` in GCC.
        #[link_name = "llvm.x86.xop.vprotq"]
        pub fn xop_vprotq(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        /// The `llvm.x86.xop.vprotw` intrinsic; known as `__builtin_ia32_vprotw` in GCC.
        #[link_name = "llvm.x86.xop.vprotw"]
        pub fn xop_vprotw(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.xop.vprotbi` intrinsic; known as `__builtin_ia32_vprotbi` in GCC.
        #[link_name = "llvm.x86.xop.vprotbi"]
        pub fn xop_vprotbi(a: ::simdty::i8x16, b: i8) -> ::simdty::i8x16;
        /// The `llvm.x86.xop.vprotdi` intrinsic; known as `__builtin_ia32_vprotdi` in GCC.
        #[link_name = "llvm.x86.xop.vprotdi"]
        pub fn xop_vprotdi(a: ::simdty::i32x4, b: i8) -> ::simdty::i32x4;
        /// The `llvm.x86.xop.vprotqi` intrinsic; known as `__builtin_ia32_vprotqi` in GCC.
        #[link_name = "llvm.x86.xop.vprotqi"]
        pub fn xop_vprotqi(a: ::simdty::i64x2, b: i8) -> ::simdty::i64x2;
        /// The `llvm.x86.xop.vprotwi` intrinsic; known as `__builtin_ia32_vprotwi` in GCC.
        #[link_name = "llvm.x86.xop.vprotwi"]
        pub fn xop_vprotwi(a: ::simdty::i16x8, b: i8) -> ::simdty::i16x8;
        /// The `llvm.x86.xop.vpshab` intrinsic; known as `__builtin_ia32_vpshab` in GCC.
        #[link_name = "llvm.x86.xop.vpshab"]
        pub fn xop_vpshab(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        /// The `llvm.x86.xop.vpshad` intrinsic; known as `__builtin_ia32_vpshad` in GCC.
        #[link_name = "llvm.x86.xop.vpshad"]
        pub fn xop_vpshad(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.xop.vpshaq` intrinsic; known as `__builtin_ia32_vpshaq` in GCC.
        #[link_name = "llvm.x86.xop.vpshaq"]
        pub fn xop_vpshaq(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        /// The `llvm.x86.xop.vpshaw` intrinsic; known as `__builtin_ia32_vpshaw` in GCC.
        #[link_name = "llvm.x86.xop.vpshaw"]
        pub fn xop_vpshaw(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.xop.vpshlb` intrinsic; known as `__builtin_ia32_vpshlb` in GCC.
        #[link_name = "llvm.x86.xop.vpshlb"]
        pub fn xop_vpshlb(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        /// The `llvm.x86.xop.vpshld` intrinsic; known as `__builtin_ia32_vpshld` in GCC.
        #[link_name = "llvm.x86.xop.vpshld"]
        pub fn xop_vpshld(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.xop.vpshlq` intrinsic; known as `__builtin_ia32_vpshlq` in GCC.
        #[link_name = "llvm.x86.xop.vpshlq"]
        pub fn xop_vpshlq(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        /// The `llvm.x86.xop.vpshlw` intrinsic; known as `__builtin_ia32_vpshlw` in GCC.
        #[link_name = "llvm.x86.xop.vpshlw"]
        pub fn xop_vpshlw(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        /// The `llvm.x86.mmx.emms` intrinsic; known as `__builtin_ia32_emms` in GCC.
        #[link_name = "llvm.x86.mmx.emms"]
        pub fn mmx_emms() -> ();
        /// The `llvm.x86.mmx.femms` intrinsic; known as `__builtin_ia32_femms` in GCC.
        #[link_name = "llvm.x86.mmx.femms"]
        pub fn mmx_femms() -> ();
        /// The `llvm.x86.bmi.bextr.32` intrinsic; known as `__builtin_ia32_bextr_u32` in GCC.
        #[link_name = "llvm.x86.bmi.bextr.32"]
        pub fn bmi_bextr_32(a: i32, b: i32) -> i32;
        /// The `llvm.x86.bmi.bextr.64` intrinsic; known as `__builtin_ia32_bextr_u64` in GCC.
        #[link_name = "llvm.x86.bmi.bextr.64"]
        pub fn bmi_bextr_64(a: i64, b: i64) -> i64;
        /// The `llvm.x86.bmi.bzhi.32` intrinsic; known as `__builtin_ia32_bzhi_si` in GCC.
        #[link_name = "llvm.x86.bmi.bzhi.32"]
        pub fn bmi_bzhi_32(a: i32, b: i32) -> i32;
        /// The `llvm.x86.bmi.bzhi.64` intrinsic; known as `__builtin_ia32_bzhi_di` in GCC.
        #[link_name = "llvm.x86.bmi.bzhi.64"]
        pub fn bmi_bzhi_64(a: i64, b: i64) -> i64;
        /// The `llvm.x86.bmi.pdep.32` intrinsic; known as `__builtin_ia32_pdep_si` in GCC.
        #[link_name = "llvm.x86.bmi.pdep.32"]
        pub fn bmi_pdep_32(a: i32, b: i32) -> i32;
        /// The `llvm.x86.bmi.pdep.64` intrinsic; known as `__builtin_ia32_pdep_di` in GCC.
        #[link_name = "llvm.x86.bmi.pdep.64"]
        pub fn bmi_pdep_64(a: i64, b: i64) -> i64;
        /// The `llvm.x86.bmi.pext.32` intrinsic; known as `__builtin_ia32_pext_si` in GCC.
        #[link_name = "llvm.x86.bmi.pext.32"]
        pub fn bmi_pext_32(a: i32, b: i32) -> i32;
        /// The `llvm.x86.bmi.pext.64` intrinsic; known as `__builtin_ia32_pext_di` in GCC.
        #[link_name = "llvm.x86.bmi.pext.64"]
        pub fn bmi_pext_64(a: i64, b: i64) -> i64;
        /// The `llvm.x86.rdfsbase.32` intrinsic; known as `__builtin_ia32_rdfsbase32` in GCC.
        #[link_name = "llvm.x86.rdfsbase.32"]
        pub fn rdfsbase_32() -> i32;
        /// The `llvm.x86.rdgsbase.32` intrinsic; known as `__builtin_ia32_rdgsbase32` in GCC.
        #[link_name = "llvm.x86.rdgsbase.32"]
        pub fn rdgsbase_32() -> i32;
        /// The `llvm.x86.rdfsbase.64` intrinsic; known as `__builtin_ia32_rdfsbase64` in GCC.
        #[link_name = "llvm.x86.rdfsbase.64"]
        pub fn rdfsbase_64() -> i64;
        /// The `llvm.x86.rdgsbase.64` intrinsic; known as `__builtin_ia32_rdgsbase64` in GCC.
        #[link_name = "llvm.x86.rdgsbase.64"]
        pub fn rdgsbase_64() -> i64;
        /// The `llvm.x86.wrfsbase.32` intrinsic; known as `__builtin_ia32_wrfsbase32` in GCC.
        #[link_name = "llvm.x86.wrfsbase.32"]
        pub fn wrfsbase_32(a: i32) -> ();
        /// The `llvm.x86.wrgsbase.32` intrinsic; known as `__builtin_ia32_wrgsbase32` in GCC.
        #[link_name = "llvm.x86.wrgsbase.32"]
        pub fn wrgsbase_32(a: i32) -> ();
        /// The `llvm.x86.wrfsbase.64` intrinsic; known as `__builtin_ia32_wrfsbase64` in GCC.
        #[link_name = "llvm.x86.wrfsbase.64"]
        pub fn wrfsbase_64(a: i64) -> ();
        /// The `llvm.x86.wrgsbase.64` intrinsic; known as `__builtin_ia32_wrgsbase64` in GCC.
        #[link_name = "llvm.x86.wrgsbase.64"]
        pub fn wrgsbase_64(a: i64) -> ();
        /// The `llvm.x86.vcvtph2ps.128` intrinsic; known as `__builtin_ia32_vcvtph2ps` in GCC.
        #[link_name = "llvm.x86.vcvtph2ps.128"]
        pub fn vcvtph2ps_128(a: ::simdty::i16x8) -> ::simdty::f32x4;
        /// The `llvm.x86.vcvtph2ps.256` intrinsic; known as `__builtin_ia32_vcvtph2ps256` in GCC.
        #[link_name = "llvm.x86.vcvtph2ps.256"]
        pub fn vcvtph2ps_256(a: ::simdty::i16x8) -> ::simdty::f32x8;
        /// The `llvm.x86.vcvtps2ph.128` intrinsic; known as `__builtin_ia32_vcvtps2ph` in GCC.
        #[link_name = "llvm.x86.vcvtps2ph.128"]
        pub fn vcvtps2ph_128(a: ::simdty::f32x4, b: i32) -> ::simdty::i16x8;
        /// The `llvm.x86.vcvtps2ph.256` intrinsic; known as `__builtin_ia32_vcvtps2ph256` in GCC.
        #[link_name = "llvm.x86.vcvtps2ph.256"]
        pub fn vcvtps2ph_256(a: ::simdty::f32x8, b: i32) -> ::simdty::i16x8;
        /// The `llvm.x86.avx512.mask.vcvtph2ps.512` intrinsic; known as `__builtin_ia32_vcvtph2ps512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.vcvtph2ps.512"]
        pub fn avx512_mask_vcvtph2ps_512(a: ::simdty::i16x16, b: ::simdty::f32x16, c: i16, d: i32) -> ::simdty::f32x16;
        /// The `llvm.x86.avx512.mask.vcvtps2ph.512` intrinsic; known as `__builtin_ia32_vcvtps2ph512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.vcvtps2ph.512"]
        pub fn avx512_mask_vcvtps2ph_512(a: ::simdty::f32x16, b: i32, c: ::simdty::i16x16, d: i16) -> ::simdty::i16x16;
        /// The `llvm.x86.tbm.bextri.u32` intrinsic; known as `__builtin_ia32_bextri_u32` in GCC.
        #[link_name = "llvm.x86.tbm.bextri.u32"]
        pub fn tbm_bextri_u32(a: i32, b: i32) -> i32;
        /// The `llvm.x86.tbm.bextri.u64` intrinsic; known as `__builtin_ia32_bextri_u64` in GCC.
        #[link_name = "llvm.x86.tbm.bextri.u64"]
        pub fn tbm_bextri_u64(a: i64, b: i64) -> i64;
        /// The `llvm.x86.addcarryx.u32` intrinsic; known as `__builtin_ia32_addcarryx_u32` in GCC.
        #[link_name = "llvm.x86.addcarryx.u32"]
        pub fn addcarryx_u32(a: i8, b: i32, c: i32, d: *mut i8) -> i8;
        /// The `llvm.x86.addcarryx.u64` intrinsic; known as `__builtin_ia32_addcarryx_u64` in GCC.
        #[link_name = "llvm.x86.addcarryx.u64"]
        pub fn addcarryx_u64(a: i8, b: i64, c: i64, d: *mut i8) -> i8;
        /// The `llvm.x86.addcarry.u32` intrinsic; known as `__builtin_ia32_addcarry_u32` in GCC.
        #[link_name = "llvm.x86.addcarry.u32"]
        pub fn addcarry_u32(a: i8, b: i32, c: i32, d: *mut i8) -> i8;
        /// The `llvm.x86.addcarry.u64` intrinsic; known as `__builtin_ia32_addcarry_u64` in GCC.
        #[link_name = "llvm.x86.addcarry.u64"]
        pub fn addcarry_u64(a: i8, b: i64, c: i64, d: *mut i8) -> i8;
        /// The `llvm.x86.subborrow.u32` intrinsic; known as `__builtin_ia32_subborrow_u32` in GCC.
        #[link_name = "llvm.x86.subborrow.u32"]
        pub fn subborrow_u32(a: i8, b: i32, c: i32, d: *mut i8) -> i8;
        /// The `llvm.x86.subborrow.u64` intrinsic; known as `__builtin_ia32_subborrow_u64` in GCC.
        #[link_name = "llvm.x86.subborrow.u64"]
        pub fn subborrow_u64(a: i8, b: i64, c: i64, d: *mut i8) -> i8;
        /// The `llvm.x86.xbegin` intrinsic; known as `__builtin_ia32_xbegin` in GCC.
        #[link_name = "llvm.x86.xbegin"]
        pub fn xbegin() -> i32;
        /// The `llvm.x86.xend` intrinsic; known as `__builtin_ia32_xend` in GCC.
        #[link_name = "llvm.x86.xend"]
        pub fn xend() -> ();
        /// The `llvm.x86.xabort` intrinsic; known as `__builtin_ia32_xabort` in GCC.
        #[link_name = "llvm.x86.xabort"]
        pub fn xabort(a: i8) -> ();
        /// The `llvm.x86.xtest` intrinsic; known as `__builtin_ia32_xtest` in GCC.
        #[link_name = "llvm.x86.xtest"]
        pub fn xtest() -> i32;
        /// The `llvm.x86.avx512.kand.w` intrinsic; known as `__builtin_ia32_kandhi` in GCC.
        #[link_name = "llvm.x86.avx512.kand.w"]
        pub fn avx512_kand_w(a: i16, b: i16) -> i16;
        /// The `llvm.x86.avx512.kandn.w` intrinsic; known as `__builtin_ia32_kandnhi` in GCC.
        #[link_name = "llvm.x86.avx512.kandn.w"]
        pub fn avx512_kandn_w(a: i16, b: i16) -> i16;
        /// The `llvm.x86.avx512.knot.w` intrinsic; known as `__builtin_ia32_knothi` in GCC.
        #[link_name = "llvm.x86.avx512.knot.w"]
        pub fn avx512_knot_w(a: i16) -> i16;
        /// The `llvm.x86.avx512.kor.w` intrinsic; known as `__builtin_ia32_korhi` in GCC.
        #[link_name = "llvm.x86.avx512.kor.w"]
        pub fn avx512_kor_w(a: i16, b: i16) -> i16;
        /// The `llvm.x86.avx512.kxor.w` intrinsic; known as `__builtin_ia32_kxorhi` in GCC.
        #[link_name = "llvm.x86.avx512.kxor.w"]
        pub fn avx512_kxor_w(a: i16, b: i16) -> i16;
        /// The `llvm.x86.avx512.kxnor.w` intrinsic; known as `__builtin_ia32_kxnorhi` in GCC.
        #[link_name = "llvm.x86.avx512.kxnor.w"]
        pub fn avx512_kxnor_w(a: i16, b: i16) -> i16;
        /// The `llvm.x86.avx512.kunpck.bw` intrinsic; known as `__builtin_ia32_kunpckhi` in GCC.
        #[link_name = "llvm.x86.avx512.kunpck.bw"]
        pub fn avx512_kunpck_bw(a: i16, b: i16) -> i16;
        /// The `llvm.x86.avx512.kortestz.w` intrinsic; known as `__builtin_ia32_kortestzhi` in GCC.
        #[link_name = "llvm.x86.avx512.kortestz.w"]
        pub fn avx512_kortestz_w(a: i16, b: i16) -> i32;
        /// The `llvm.x86.avx512.kortestc.w` intrinsic; known as `__builtin_ia32_kortestchi` in GCC.
        #[link_name = "llvm.x86.avx512.kortestc.w"]
        pub fn avx512_kortestc_w(a: i16, b: i16) -> i32;
        /// The `llvm.x86.avx512.cvtss2usi` intrinsic; known as `__builtin_ia32_cvtss2usi` in GCC.
        #[link_name = "llvm.x86.avx512.cvtss2usi"]
        pub fn avx512_cvtss2usi(a: ::simdty::f32x4) -> i32;
        /// The `llvm.x86.avx512.cvtss2usi64` intrinsic; known as `__builtin_ia32_cvtss2usi64` in GCC.
        #[link_name = "llvm.x86.avx512.cvtss2usi64"]
        pub fn avx512_cvtss2usi64(a: ::simdty::f32x4) -> i64;
        /// The `llvm.x86.avx512.cvttss2usi` intrinsic; known as `__builtin_ia32_cvttss2usi` in GCC.
        #[link_name = "llvm.x86.avx512.cvttss2usi"]
        pub fn avx512_cvttss2usi(a: ::simdty::f32x4) -> i32;
        /// The `llvm.x86.avx512.cvttss2usi64` intrinsic; known as `__builtin_ia32_cvttss2usi64` in GCC.
        #[link_name = "llvm.x86.avx512.cvttss2usi64"]
        pub fn avx512_cvttss2usi64(a: ::simdty::f32x4) -> i64;
        /// The `llvm.x86.avx512.cvtusi2ss` intrinsic; known as `__builtin_ia32_cvtusi2ss` in GCC.
        #[link_name = "llvm.x86.avx512.cvtusi2ss"]
        pub fn avx512_cvtusi2ss(a: ::simdty::f32x4, b: i32) -> ::simdty::f32x4;
        /// The `llvm.x86.avx512.cvtusi642ss` intrinsic; known as `__builtin_ia32_cvtusi642ss` in GCC.
        #[link_name = "llvm.x86.avx512.cvtusi642ss"]
        pub fn avx512_cvtusi642ss(a: ::simdty::f32x4, b: i64) -> ::simdty::f32x4;
        /// The `llvm.x86.avx512.cvtsd2usi` intrinsic; known as `__builtin_ia32_cvtsd2usi` in GCC.
        #[link_name = "llvm.x86.avx512.cvtsd2usi"]
        pub fn avx512_cvtsd2usi(a: ::simdty::f64x2) -> i32;
        /// The `llvm.x86.avx512.cvtsd2usi64` intrinsic; known as `__builtin_ia32_cvtsd2usi64` in GCC.
        #[link_name = "llvm.x86.avx512.cvtsd2usi64"]
        pub fn avx512_cvtsd2usi64(a: ::simdty::f64x2) -> i64;
        /// The `llvm.x86.avx512.cvttsd2usi` intrinsic; known as `__builtin_ia32_cvttsd2usi` in GCC.
        #[link_name = "llvm.x86.avx512.cvttsd2usi"]
        pub fn avx512_cvttsd2usi(a: ::simdty::f64x2) -> i32;
        /// The `llvm.x86.avx512.cvttsd2usi64` intrinsic; known as `__builtin_ia32_cvttsd2usi64` in GCC.
        #[link_name = "llvm.x86.avx512.cvttsd2usi64"]
        pub fn avx512_cvttsd2usi64(a: ::simdty::f64x2) -> i64;
        /// The `llvm.x86.avx512.cvtusi2sd` intrinsic; known as `__builtin_ia32_cvtusi2sd` in GCC.
        #[link_name = "llvm.x86.avx512.cvtusi2sd"]
        pub fn avx512_cvtusi2sd(a: ::simdty::f64x2, b: i32) -> ::simdty::f64x2;
        /// The `llvm.x86.avx512.cvtusi642sd` intrinsic; known as `__builtin_ia32_cvtusi642sd` in GCC.
        #[link_name = "llvm.x86.avx512.cvtusi642sd"]
        pub fn avx512_cvtusi642sd(a: ::simdty::f64x2, b: i64) -> ::simdty::f64x2;
        /// The `llvm.x86.avx512.mask.cvttps2dq.512` intrinsic; known as `__builtin_ia32_cvttps2dq512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.cvttps2dq.512"]
        pub fn avx512_mask_cvttps2dq_512(a: ::simdty::f32x16, b: ::simdty::i32x16, c: i16, d: i32) -> ::simdty::i32x16;
        /// The `llvm.x86.avx512.mask.cvttps2udq.512` intrinsic; known as `__builtin_ia32_cvttps2udq512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.cvttps2udq.512"]
        pub fn avx512_mask_cvttps2udq_512(a: ::simdty::f32x16, b: ::simdty::i32x16, c: i16, d: i32) -> ::simdty::i32x16;
        /// The `llvm.x86.avx512.mask.cvttpd2dq.512` intrinsic; known as `__builtin_ia32_cvttpd2dq512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.cvttpd2dq.512"]
        pub fn avx512_mask_cvttpd2dq_512(a: ::simdty::f64x8, b: ::simdty::i32x8, c: i8, d: i32) -> ::simdty::i32x8;
        /// The `llvm.x86.avx512.mask.cvttpd2udq.512` intrinsic; known as `__builtin_ia32_cvttpd2udq512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.cvttpd2udq.512"]
        pub fn avx512_mask_cvttpd2udq_512(a: ::simdty::f64x8, b: ::simdty::i32x8, c: i8, d: i32) -> ::simdty::i32x8;
        /// The `llvm.x86.avx512.mask.rndscale.ps.512` intrinsic; known as `__builtin_ia32_rndscaleps_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.rndscale.ps.512"]
        pub fn avx512_mask_rndscale_ps_512(a: ::simdty::f32x16, b: i32, c: ::simdty::f32x16, d: i16, e: i32) -> ::simdty::f32x16;
        /// The `llvm.x86.avx512.mask.rndscale.pd.512` intrinsic; known as `__builtin_ia32_rndscalepd_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.rndscale.pd.512"]
        pub fn avx512_mask_rndscale_pd_512(a: ::simdty::f64x8, b: i32, c: ::simdty::f64x8, d: i8, e: i32) -> ::simdty::f64x8;
        /// The `llvm.x86.avx512.mask.cvtps2dq.512` intrinsic; known as `__builtin_ia32_cvtps2dq512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.cvtps2dq.512"]
        pub fn avx512_mask_cvtps2dq_512(a: ::simdty::f32x16, b: ::simdty::i32x16, c: i16, d: i32) -> ::simdty::i32x16;
        /// The `llvm.x86.avx512.mask.cvtpd2dq.512` intrinsic; known as `__builtin_ia32_cvtpd2dq512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.cvtpd2dq.512"]
        pub fn avx512_mask_cvtpd2dq_512(a: ::simdty::f64x8, b: ::simdty::i32x8, c: i8, d: i32) -> ::simdty::i32x8;
        /// The `llvm.x86.avx512.mask.cvtps2udq.512` intrinsic; known as `__builtin_ia32_cvtps2udq512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.cvtps2udq.512"]
        pub fn avx512_mask_cvtps2udq_512(a: ::simdty::f32x16, b: ::simdty::i32x16, c: i16, d: i32) -> ::simdty::i32x16;
        /// The `llvm.x86.avx512.mask.cvtpd2udq.512` intrinsic; known as `__builtin_ia32_cvtpd2udq512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.cvtpd2udq.512"]
        pub fn avx512_mask_cvtpd2udq_512(a: ::simdty::f64x8, b: ::simdty::i32x8, c: i8, d: i32) -> ::simdty::i32x8;
        /// The `llvm.x86.avx512.mask.cvtdq2ps.512` intrinsic; known as `__builtin_ia32_cvtdq2ps512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.cvtdq2ps.512"]
        pub fn avx512_mask_cvtdq2ps_512(a: ::simdty::i32x16, b: ::simdty::f32x16, c: i16, d: i32) -> ::simdty::f32x16;
        /// The `llvm.x86.avx512.mask.cvtdq2pd.512` intrinsic; known as `__builtin_ia32_cvtdq2pd512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.cvtdq2pd.512"]
        pub fn avx512_mask_cvtdq2pd_512(a: ::simdty::i32x8, b: ::simdty::f64x8, c: i8) -> ::simdty::f64x8;
        /// The `llvm.x86.avx512.mask.cvtudq2ps.512` intrinsic; known as `__builtin_ia32_cvtudq2ps512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.cvtudq2ps.512"]
        pub fn avx512_mask_cvtudq2ps_512(a: ::simdty::i32x16, b: ::simdty::f32x16, c: i16, d: i32) -> ::simdty::f32x16;
        /// The `llvm.x86.avx512.mask.cvtudq2pd.512` intrinsic; known as `__builtin_ia32_cvtudq2pd512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.cvtudq2pd.512"]
        pub fn avx512_mask_cvtudq2pd_512(a: ::simdty::i32x8, b: ::simdty::f64x8, c: i8) -> ::simdty::f64x8;
        /// The `llvm.x86.avx512.mask.cvtpd2ps.512` intrinsic; known as `__builtin_ia32_cvtpd2ps512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.cvtpd2ps.512"]
        pub fn avx512_mask_cvtpd2ps_512(a: ::simdty::f64x8, b: ::simdty::f32x8, c: i8, d: i32) -> ::simdty::f32x8;
        /// The `llvm.x86.avx512.vbroadcast.ss.512` intrinsic; known as `__builtin_ia32_vbroadcastss512` in GCC.
        #[link_name = "llvm.x86.avx512.vbroadcast.ss.512"]
        pub fn avx512_vbroadcast_ss_512(a: *mut i8) -> ::simdty::f32x16;
        /// The `llvm.x86.avx512.vbroadcast.ss.ps.512` intrinsic; known as `__builtin_ia32_vbroadcastss_ps512` in GCC.
        #[link_name = "llvm.x86.avx512.vbroadcast.ss.ps.512"]
        pub fn avx512_vbroadcast_ss_ps_512(a: ::simdty::f32x4) -> ::simdty::f32x16;
        /// The `llvm.x86.avx512.vbroadcast.sd.512` intrinsic; known as `__builtin_ia32_vbroadcastsd512` in GCC.
        #[link_name = "llvm.x86.avx512.vbroadcast.sd.512"]
        pub fn avx512_vbroadcast_sd_512(a: *mut i8) -> ::simdty::f64x8;
        /// The `llvm.x86.avx512.vbroadcast.sd.pd.512` intrinsic; known as `__builtin_ia32_vbroadcastsd_pd512` in GCC.
        #[link_name = "llvm.x86.avx512.vbroadcast.sd.pd.512"]
        pub fn avx512_vbroadcast_sd_pd_512(a: ::simdty::f64x2) -> ::simdty::f64x8;
        /// The `llvm.x86.avx512.pbroadcastd.512` intrinsic; known as `__builtin_ia32_pbroadcastd512` in GCC.
        #[link_name = "llvm.x86.avx512.pbroadcastd.512"]
        pub fn avx512_pbroadcastd_512(a: ::simdty::i32x4) -> ::simdty::i32x16;
        /// The `llvm.x86.avx512.pbroadcastd.i32.512` intrinsic.
        #[link_name = "llvm.x86.avx512.pbroadcastd.i32.512"]
        pub fn avx512_pbroadcastd_i32_512(a: i32) -> ::simdty::i32x16;
        /// The `llvm.x86.avx512.pbroadcastq.512` intrinsic; known as `__builtin_ia32_pbroadcastq512` in GCC.
        #[link_name = "llvm.x86.avx512.pbroadcastq.512"]
        pub fn avx512_pbroadcastq_512(a: ::simdty::i64x2) -> ::simdty::i64x8;
        /// The `llvm.x86.avx512.pbroadcastq.i64.512` intrinsic.
        #[link_name = "llvm.x86.avx512.pbroadcastq.i64.512"]
        pub fn avx512_pbroadcastq_i64_512(a: i64) -> ::simdty::i64x8;
        /// The `llvm.x86.avx512.pmovzxbq` intrinsic; known as `__builtin_ia32_pmovzxbq512` in GCC.
        #[link_name = "llvm.x86.avx512.pmovzxbq"]
        pub fn avx512_pmovzxbq(a: ::simdty::i8x16) -> ::simdty::i64x8;
        /// The `llvm.x86.avx512.pmovzxwd` intrinsic; known as `__builtin_ia32_pmovzxwd512` in GCC.
        #[link_name = "llvm.x86.avx512.pmovzxwd"]
        pub fn avx512_pmovzxwd(a: ::simdty::i16x16) -> ::simdty::i32x16;
        /// The `llvm.x86.avx512.pmovzxbd` intrinsic; known as `__builtin_ia32_pmovzxbd512` in GCC.
        #[link_name = "llvm.x86.avx512.pmovzxbd"]
        pub fn avx512_pmovzxbd(a: ::simdty::i8x16) -> ::simdty::i32x16;
        /// The `llvm.x86.avx512.pmovzxwq` intrinsic; known as `__builtin_ia32_pmovzxwq512` in GCC.
        #[link_name = "llvm.x86.avx512.pmovzxwq"]
        pub fn avx512_pmovzxwq(a: ::simdty::i16x8) -> ::simdty::i64x8;
        /// The `llvm.x86.avx512.pmovzxdq` intrinsic; known as `__builtin_ia32_pmovzxdq512` in GCC.
        #[link_name = "llvm.x86.avx512.pmovzxdq"]
        pub fn avx512_pmovzxdq(a: ::simdty::i32x8) -> ::simdty::i64x8;
        /// The `llvm.x86.avx512.mask.max.ps.512` intrinsic; known as `__builtin_ia32_maxps512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.max.ps.512"]
        pub fn avx512_mask_max_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: ::simdty::f32x16, d: i16, e: i32) -> ::simdty::f32x16;
        /// The `llvm.x86.avx512.mask.max.pd.512` intrinsic; known as `__builtin_ia32_maxpd512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.max.pd.512"]
        pub fn avx512_mask_max_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: ::simdty::f64x8, d: i8, e: i32) -> ::simdty::f64x8;
        /// The `llvm.x86.avx512.mask.min.ps.512` intrinsic; known as `__builtin_ia32_minps512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.min.ps.512"]
        pub fn avx512_mask_min_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: ::simdty::f32x16, d: i16, e: i32) -> ::simdty::f32x16;
        /// The `llvm.x86.avx512.mask.min.pd.512` intrinsic; known as `__builtin_ia32_minpd512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.min.pd.512"]
        pub fn avx512_mask_min_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: ::simdty::f64x8, d: i8, e: i32) -> ::simdty::f64x8;
        /// The `llvm.x86.avx512.rndscale.ss` intrinsic; known as `__builtin_ia32_rndscaless` in GCC.
        #[link_name = "llvm.x86.avx512.rndscale.ss"]
        pub fn avx512_rndscale_ss(a: ::simdty::f32x4, b: ::simdty::f32x4, c: i32) -> ::simdty::f32x4;
        /// The `llvm.x86.avx512.rndscale.sd` intrinsic; known as `__builtin_ia32_rndscalesd` in GCC.
        #[link_name = "llvm.x86.avx512.rndscale.sd"]
        pub fn avx512_rndscale_sd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: i32) -> ::simdty::f64x2;
        /// The `llvm.x86.avx512.sqrt.ss` intrinsic; known as `__builtin_ia32_sqrtrndss` in GCC.
        #[link_name = "llvm.x86.avx512.sqrt.ss"]
        pub fn avx512_sqrt_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        /// The `llvm.x86.avx512.sqrt.sd` intrinsic; known as `__builtin_ia32_sqrtrndsd` in GCC.
        #[link_name = "llvm.x86.avx512.sqrt.sd"]
        pub fn avx512_sqrt_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        /// The `llvm.x86.avx512.sqrt.pd.512` intrinsic; known as `__builtin_ia32_sqrtpd512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.sqrt.pd.512"]
        pub fn avx512_sqrt_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: i8, d: i32) -> ::simdty::f64x8;
        /// The `llvm.x86.avx512.sqrt.ps.512` intrinsic; known as `__builtin_ia32_sqrtps512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.sqrt.ps.512"]
        pub fn avx512_sqrt_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: i16, d: i32) -> ::simdty::f32x16;
        /// The `llvm.x86.avx512.rsqrt14.ss` intrinsic; known as `__builtin_ia32_rsqrt14ss_mask` in GCC.
        #[link_name = "llvm.x86.avx512.rsqrt14.ss"]
        pub fn avx512_rsqrt14_ss(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4, d: i8) -> ::simdty::f32x4;
        /// The `llvm.x86.avx512.rsqrt14.sd` intrinsic; known as `__builtin_ia32_rsqrt14sd_mask` in GCC.
        #[link_name = "llvm.x86.avx512.rsqrt14.sd"]
        pub fn avx512_rsqrt14_sd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2, d: i8) -> ::simdty::f64x2;
        /// The `llvm.x86.avx512.rsqrt14.pd.512` intrinsic; known as `__builtin_ia32_rsqrt14pd512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.rsqrt14.pd.512"]
        pub fn avx512_rsqrt14_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: i8) -> ::simdty::f64x8;
        /// The `llvm.x86.avx512.rsqrt14.ps.512` intrinsic; known as `__builtin_ia32_rsqrt14ps512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.rsqrt14.ps.512"]
        pub fn avx512_rsqrt14_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: i16) -> ::simdty::f32x16;
        /// The `llvm.x86.avx512.rcp14.ss` intrinsic; known as `__builtin_ia32_rcp14ss_mask` in GCC.
        #[link_name = "llvm.x86.avx512.rcp14.ss"]
        pub fn avx512_rcp14_ss(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4, d: i8) -> ::simdty::f32x4;
        /// The `llvm.x86.avx512.rcp14.sd` intrinsic; known as `__builtin_ia32_rcp14sd_mask` in GCC.
        #[link_name = "llvm.x86.avx512.rcp14.sd"]
        pub fn avx512_rcp14_sd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2, d: i8) -> ::simdty::f64x2;
        /// The `llvm.x86.avx512.rcp14.pd.512` intrinsic; known as `__builtin_ia32_rcp14pd512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.rcp14.pd.512"]
        pub fn avx512_rcp14_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: i8) -> ::simdty::f64x8;
        /// The `llvm.x86.avx512.rcp14.ps.512` intrinsic; known as `__builtin_ia32_rcp14ps512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.rcp14.ps.512"]
        pub fn avx512_rcp14_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: i16) -> ::simdty::f32x16;
        /// The `llvm.x86.avx512.rcp28.ps` intrinsic; known as `__builtin_ia32_rcp28ps_mask` in GCC.
        #[link_name = "llvm.x86.avx512.rcp28.ps"]
        pub fn avx512_rcp28_ps(a: ::simdty::f32x16, b: ::simdty::f32x16, c: i16, d: i32) -> ::simdty::f32x16;
        /// The `llvm.x86.avx512.rcp28.pd` intrinsic; known as `__builtin_ia32_rcp28pd_mask` in GCC.
        #[link_name = "llvm.x86.avx512.rcp28.pd"]
        pub fn avx512_rcp28_pd(a: ::simdty::f64x8, b: ::simdty::f64x8, c: i8, d: i32) -> ::simdty::f64x8;
        /// The `llvm.x86.avx512.rcp28.ss` intrinsic; known as `__builtin_ia32_rcp28ss_mask` in GCC.
        #[link_name = "llvm.x86.avx512.rcp28.ss"]
        pub fn avx512_rcp28_ss(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4, d: i8, e: i32) -> ::simdty::f32x4;
        /// The `llvm.x86.avx512.rcp28.sd` intrinsic; known as `__builtin_ia32_rcp28sd_mask` in GCC.
        #[link_name = "llvm.x86.avx512.rcp28.sd"]
        pub fn avx512_rcp28_sd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2, d: i8, e: i32) -> ::simdty::f64x2;
        /// The `llvm.x86.avx512.rsqrt28.ps` intrinsic; known as `__builtin_ia32_rsqrt28ps_mask` in GCC.
        #[link_name = "llvm.x86.avx512.rsqrt28.ps"]
        pub fn avx512_rsqrt28_ps(a: ::simdty::f32x16, b: ::simdty::f32x16, c: i16, d: i32) -> ::simdty::f32x16;
        /// The `llvm.x86.avx512.rsqrt28.pd` intrinsic; known as `__builtin_ia32_rsqrt28pd_mask` in GCC.
        #[link_name = "llvm.x86.avx512.rsqrt28.pd"]
        pub fn avx512_rsqrt28_pd(a: ::simdty::f64x8, b: ::simdty::f64x8, c: i8, d: i32) -> ::simdty::f64x8;
        /// The `llvm.x86.avx512.rsqrt28.ss` intrinsic; known as `__builtin_ia32_rsqrt28ss_mask` in GCC.
        #[link_name = "llvm.x86.avx512.rsqrt28.ss"]
        pub fn avx512_rsqrt28_ss(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4, d: i8, e: i32) -> ::simdty::f32x4;
        /// The `llvm.x86.avx512.rsqrt28.sd` intrinsic; known as `__builtin_ia32_rsqrt28sd_mask` in GCC.
        #[link_name = "llvm.x86.avx512.rsqrt28.sd"]
        pub fn avx512_rsqrt28_sd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2, d: i8, e: i32) -> ::simdty::f64x2;
        /// The `llvm.x86.avx512.psll.dq` intrinsic; known as `__builtin_ia32_pslldqi512` in GCC.
        #[link_name = "llvm.x86.avx512.psll.dq"]
        pub fn avx512_psll_dq(a: ::simdty::i64x8, b: i32) -> ::simdty::i64x8;
        /// The `llvm.x86.avx512.psrl.dq` intrinsic; known as `__builtin_ia32_psrldqi512` in GCC.
        #[link_name = "llvm.x86.avx512.psrl.dq"]
        pub fn avx512_psrl_dq(a: ::simdty::i64x8, b: i32) -> ::simdty::i64x8;
        /// The `llvm.x86.avx512.psll.dq.bs` intrinsic; known as `__builtin_ia32_pslldqi512_byteshift` in GCC.
        #[link_name = "llvm.x86.avx512.psll.dq.bs"]
        pub fn avx512_psll_dq_bs(a: ::simdty::i64x8, b: i32) -> ::simdty::i64x8;
        /// The `llvm.x86.avx512.psrl.dq.bs` intrinsic; known as `__builtin_ia32_psrldqi512_byteshift` in GCC.
        #[link_name = "llvm.x86.avx512.psrl.dq.bs"]
        pub fn avx512_psrl_dq_bs(a: ::simdty::i64x8, b: i32) -> ::simdty::i64x8;
        /// The `llvm.x86.avx512.gather.dpd.512` intrinsic; known as `__builtin_ia32_gathersiv8df` in GCC.
        #[link_name = "llvm.x86.avx512.gather.dpd.512"]
        pub fn avx512_gather_dpd_512(a: ::simdty::f64x8, b: *mut i8, c: ::simdty::i32x8, d: i8, e: i32) -> ::simdty::f64x8;
        /// The `llvm.x86.avx512.gather.dps.512` intrinsic; known as `__builtin_ia32_gathersiv16sf` in GCC.
        #[link_name = "llvm.x86.avx512.gather.dps.512"]
        pub fn avx512_gather_dps_512(a: ::simdty::f32x16, b: *mut i8, c: ::simdty::i32x16, d: i16, e: i32) -> ::simdty::f32x16;
        /// The `llvm.x86.avx512.gather.qpd.512` intrinsic; known as `__builtin_ia32_gatherdiv8df` in GCC.
        #[link_name = "llvm.x86.avx512.gather.qpd.512"]
        pub fn avx512_gather_qpd_512(a: ::simdty::f64x8, b: *mut i8, c: ::simdty::i64x8, d: i8, e: i32) -> ::simdty::f64x8;
        /// The `llvm.x86.avx512.gather.qps.512` intrinsic; known as `__builtin_ia32_gatherdiv16sf` in GCC.
        #[link_name = "llvm.x86.avx512.gather.qps.512"]
        pub fn avx512_gather_qps_512(a: ::simdty::f32x8, b: *mut i8, c: ::simdty::i64x8, d: i8, e: i32) -> ::simdty::f32x8;
        /// The `llvm.x86.avx512.gather.dpq.512` intrinsic; known as `__builtin_ia32_gathersiv8di` in GCC.
        #[link_name = "llvm.x86.avx512.gather.dpq.512"]
        pub fn avx512_gather_dpq_512(a: ::simdty::i64x8, b: *mut i8, c: ::simdty::i32x8, d: i8, e: i32) -> ::simdty::i64x8;
        /// The `llvm.x86.avx512.gather.dpi.512` intrinsic; known as `__builtin_ia32_gathersiv16si` in GCC.
        #[link_name = "llvm.x86.avx512.gather.dpi.512"]
        pub fn avx512_gather_dpi_512(a: ::simdty::i32x16, b: *mut i8, c: ::simdty::i32x16, d: i16, e: i32) -> ::simdty::i32x16;
        /// The `llvm.x86.avx512.gather.qpq.512` intrinsic; known as `__builtin_ia32_gatherdiv8di` in GCC.
        #[link_name = "llvm.x86.avx512.gather.qpq.512"]
        pub fn avx512_gather_qpq_512(a: ::simdty::i64x8, b: *mut i8, c: ::simdty::i64x8, d: i8, e: i32) -> ::simdty::i64x8;
        /// The `llvm.x86.avx512.gather.qpi.512` intrinsic; known as `__builtin_ia32_gatherdiv16si` in GCC.
        #[link_name = "llvm.x86.avx512.gather.qpi.512"]
        pub fn avx512_gather_qpi_512(a: ::simdty::i32x8, b: *mut i8, c: ::simdty::i64x8, d: i8, e: i32) -> ::simdty::i32x8;
        /// The `llvm.x86.avx512.scatter.dpd.512` intrinsic; known as `__builtin_ia32_scattersiv8df` in GCC.
        #[link_name = "llvm.x86.avx512.scatter.dpd.512"]
        pub fn avx512_scatter_dpd_512(a: *mut i8, b: i8, c: ::simdty::i32x8, d: ::simdty::f64x8, e: i32) -> ();
        /// The `llvm.x86.avx512.scatter.dps.512` intrinsic; known as `__builtin_ia32_scattersiv16sf` in GCC.
        #[link_name = "llvm.x86.avx512.scatter.dps.512"]
        pub fn avx512_scatter_dps_512(a: *mut i8, b: i16, c: ::simdty::i32x16, d: ::simdty::f32x16, e: i32) -> ();
        /// The `llvm.x86.avx512.scatter.qpd.512` intrinsic; known as `__builtin_ia32_scatterdiv8df` in GCC.
        #[link_name = "llvm.x86.avx512.scatter.qpd.512"]
        pub fn avx512_scatter_qpd_512(a: *mut i8, b: i8, c: ::simdty::i64x8, d: ::simdty::f64x8, e: i32) -> ();
        /// The `llvm.x86.avx512.scatter.qps.512` intrinsic; known as `__builtin_ia32_scatterdiv16sf` in GCC.
        #[link_name = "llvm.x86.avx512.scatter.qps.512"]
        pub fn avx512_scatter_qps_512(a: *mut i8, b: i8, c: ::simdty::i64x8, d: ::simdty::f32x8, e: i32) -> ();
        /// The `llvm.x86.avx512.scatter.dpq.512` intrinsic; known as `__builtin_ia32_scattersiv8di` in GCC.
        #[link_name = "llvm.x86.avx512.scatter.dpq.512"]
        pub fn avx512_scatter_dpq_512(a: *mut i8, b: i8, c: ::simdty::i32x8, d: ::simdty::i64x8, e: i32) -> ();
        /// The `llvm.x86.avx512.scatter.dpi.512` intrinsic; known as `__builtin_ia32_scattersiv16si` in GCC.
        #[link_name = "llvm.x86.avx512.scatter.dpi.512"]
        pub fn avx512_scatter_dpi_512(a: *mut i8, b: i16, c: ::simdty::i32x16, d: ::simdty::i32x16, e: i32) -> ();
        /// The `llvm.x86.avx512.scatter.qpq.512` intrinsic; known as `__builtin_ia32_scatterdiv8di` in GCC.
        #[link_name = "llvm.x86.avx512.scatter.qpq.512"]
        pub fn avx512_scatter_qpq_512(a: *mut i8, b: i8, c: ::simdty::i64x8, d: ::simdty::i64x8, e: i32) -> ();
        /// The `llvm.x86.avx512.scatter.qpi.512` intrinsic; known as `__builtin_ia32_scatterdiv16si` in GCC.
        #[link_name = "llvm.x86.avx512.scatter.qpi.512"]
        pub fn avx512_scatter_qpi_512(a: *mut i8, b: i8, c: ::simdty::i64x8, d: ::simdty::i32x8, e: i32) -> ();
        /// The `llvm.x86.avx512.gatherpf.dpd.512` intrinsic; known as `__builtin_ia32_gatherpfdpd` in GCC.
        #[link_name = "llvm.x86.avx512.gatherpf.dpd.512"]
        pub fn avx512_gatherpf_dpd_512(a: i8, b: ::simdty::i32x8, c: *mut i8, d: i32, e: i32) -> ();
        /// The `llvm.x86.avx512.gatherpf.dps.512` intrinsic; known as `__builtin_ia32_gatherpfdps` in GCC.
        #[link_name = "llvm.x86.avx512.gatherpf.dps.512"]
        pub fn avx512_gatherpf_dps_512(a: i16, b: ::simdty::i32x16, c: *mut i8, d: i32, e: i32) -> ();
        /// The `llvm.x86.avx512.gatherpf.qpd.512` intrinsic; known as `__builtin_ia32_gatherpfqpd` in GCC.
        #[link_name = "llvm.x86.avx512.gatherpf.qpd.512"]
        pub fn avx512_gatherpf_qpd_512(a: i8, b: ::simdty::i64x8, c: *mut i8, d: i32, e: i32) -> ();
        /// The `llvm.x86.avx512.gatherpf.qps.512` intrinsic; known as `__builtin_ia32_gatherpfqps` in GCC.
        #[link_name = "llvm.x86.avx512.gatherpf.qps.512"]
        pub fn avx512_gatherpf_qps_512(a: i8, b: ::simdty::i64x8, c: *mut i8, d: i32, e: i32) -> ();
        /// The `llvm.x86.avx512.scatterpf.dpd.512` intrinsic; known as `__builtin_ia32_scatterpfdpd` in GCC.
        #[link_name = "llvm.x86.avx512.scatterpf.dpd.512"]
        pub fn avx512_scatterpf_dpd_512(a: i8, b: ::simdty::i32x8, c: *mut i8, d: i32, e: i32) -> ();
        /// The `llvm.x86.avx512.scatterpf.dps.512` intrinsic; known as `__builtin_ia32_scatterpfdps` in GCC.
        #[link_name = "llvm.x86.avx512.scatterpf.dps.512"]
        pub fn avx512_scatterpf_dps_512(a: i16, b: ::simdty::i32x16, c: *mut i8, d: i32, e: i32) -> ();
        /// The `llvm.x86.avx512.scatterpf.qpd.512` intrinsic; known as `__builtin_ia32_scatterpfqpd` in GCC.
        #[link_name = "llvm.x86.avx512.scatterpf.qpd.512"]
        pub fn avx512_scatterpf_qpd_512(a: i8, b: ::simdty::i64x8, c: *mut i8, d: i32, e: i32) -> ();
        /// The `llvm.x86.avx512.scatterpf.qps.512` intrinsic; known as `__builtin_ia32_scatterpfqps` in GCC.
        #[link_name = "llvm.x86.avx512.scatterpf.qps.512"]
        pub fn avx512_scatterpf_qps_512(a: i8, b: ::simdty::i64x8, c: *mut i8, d: i32, e: i32) -> ();
        /// The `llvm.x86.avx512.mask.conflict.d.512` intrinsic; known as `__builtin_ia32_vpconflictsi_512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.conflict.d.512"]
        pub fn avx512_mask_conflict_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: i16) -> ::simdty::i32x16;
        /// The `llvm.x86.avx512.mask.conflict.q.512` intrinsic; known as `__builtin_ia32_vpconflictdi_512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.conflict.q.512"]
        pub fn avx512_mask_conflict_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: i8) -> ::simdty::i64x8;
        /// The `llvm.x86.avx512.mask.lzcnt.d.512` intrinsic; known as `__builtin_ia32_vplzcntd_512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.lzcnt.d.512"]
        pub fn avx512_mask_lzcnt_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: i16) -> ::simdty::i32x16;
        /// The `llvm.x86.avx512.mask.lzcnt.q.512` intrinsic; known as `__builtin_ia32_vplzcntq_512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.lzcnt.q.512"]
        pub fn avx512_mask_lzcnt_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: i8) -> ::simdty::i64x8;
        /// The `llvm.x86.avx512.mask.blend.ps.512` intrinsic; known as `__builtin_ia32_blendmps_512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.blend.ps.512"]
        pub fn avx512_mask_blend_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: i16) -> ::simdty::f32x16;
        /// The `llvm.x86.avx512.mask.blend.pd.512` intrinsic; known as `__builtin_ia32_blendmpd_512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.blend.pd.512"]
        pub fn avx512_mask_blend_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: i8) -> ::simdty::f64x8;
        /// The `llvm.x86.avx512.mask.blend.d.512` intrinsic; known as `__builtin_ia32_blendmd_512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.blend.d.512"]
        pub fn avx512_mask_blend_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: i16) -> ::simdty::i32x16;
        /// The `llvm.x86.avx512.mask.blend.q.512` intrinsic; known as `__builtin_ia32_blendmq_512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.blend.q.512"]
        pub fn avx512_mask_blend_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: i8) -> ::simdty::i64x8;
        /// The `llvm.x86.avx512.mask.valign.q.512` intrinsic; known as `__builtin_ia32_alignq512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.valign.q.512"]
        pub fn avx512_mask_valign_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: i8, d: ::simdty::i64x8, e: i8) -> ::simdty::i64x8;
        /// The `llvm.x86.avx512.mask.valign.d.512` intrinsic; known as `__builtin_ia32_alignd512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.valign.d.512"]
        pub fn avx512_mask_valign_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: i8, d: ::simdty::i32x16, e: i16) -> ::simdty::i32x16;
        /// The `llvm.x86.avx512.mask.pcmpeq.b.512` intrinsic; known as `__builtin_ia32_pcmpeqb512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.b.512"]
        pub fn avx512_mask_pcmpeq_b_512(a: ::simdty::i8x64, b: ::simdty::i8x64, c: i64) -> i64;
        /// The `llvm.x86.avx512.mask.pcmpeq.w.512` intrinsic; known as `__builtin_ia32_pcmpeqw512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.w.512"]
        pub fn avx512_mask_pcmpeq_w_512(a: ::simdty::i16x32, b: ::simdty::i16x32, c: i32) -> i32;
        /// The `llvm.x86.avx512.mask.pcmpeq.d.512` intrinsic; known as `__builtin_ia32_pcmpeqd512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.d.512"]
        pub fn avx512_mask_pcmpeq_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: i16) -> i16;
        /// The `llvm.x86.avx512.mask.pcmpeq.q.512` intrinsic; known as `__builtin_ia32_pcmpeqq512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.q.512"]
        pub fn avx512_mask_pcmpeq_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: i8) -> i8;
        /// The `llvm.x86.avx512.mask.pcmpgt.b.512` intrinsic; known as `__builtin_ia32_pcmpgtb512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.b.512"]
        pub fn avx512_mask_pcmpgt_b_512(a: ::simdty::i8x64, b: ::simdty::i8x64, c: i64) -> i64;
        /// The `llvm.x86.avx512.mask.pcmpgt.w.512` intrinsic; known as `__builtin_ia32_pcmpgtw512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.w.512"]
        pub fn avx512_mask_pcmpgt_w_512(a: ::simdty::i16x32, b: ::simdty::i16x32, c: i32) -> i32;
        /// The `llvm.x86.avx512.mask.pcmpgt.d.512` intrinsic; known as `__builtin_ia32_pcmpgtd512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.d.512"]
        pub fn avx512_mask_pcmpgt_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: i16) -> i16;
        /// The `llvm.x86.avx512.mask.pcmpgt.q.512` intrinsic; known as `__builtin_ia32_pcmpgtq512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.q.512"]
        pub fn avx512_mask_pcmpgt_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: i8) -> i8;
        /// The `llvm.x86.avx512.mask.pcmpeq.b.256` intrinsic; known as `__builtin_ia32_pcmpeqb256_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.b.256"]
        pub fn avx512_mask_pcmpeq_b_256(a: ::simdty::i8x32, b: ::simdty::i8x32, c: i32) -> i32;
        /// The `llvm.x86.avx512.mask.pcmpeq.w.256` intrinsic; known as `__builtin_ia32_pcmpeqw256_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.w.256"]
        pub fn avx512_mask_pcmpeq_w_256(a: ::simdty::i16x16, b: ::simdty::i16x16, c: i16) -> i16;
        /// The `llvm.x86.avx512.mask.pcmpeq.d.256` intrinsic; known as `__builtin_ia32_pcmpeqd256_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.d.256"]
        pub fn avx512_mask_pcmpeq_d_256(a: ::simdty::i32x8, b: ::simdty::i32x8, c: i8) -> i8;
        /// The `llvm.x86.avx512.mask.pcmpeq.q.256` intrinsic; known as `__builtin_ia32_pcmpeqq256_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.q.256"]
        pub fn avx512_mask_pcmpeq_q_256(a: ::simdty::i64x4, b: ::simdty::i64x4, c: i8) -> i8;
        /// The `llvm.x86.avx512.mask.pcmpgt.b.256` intrinsic; known as `__builtin_ia32_pcmpgtb256_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.b.256"]
        pub fn avx512_mask_pcmpgt_b_256(a: ::simdty::i8x32, b: ::simdty::i8x32, c: i32) -> i32;
        /// The `llvm.x86.avx512.mask.pcmpgt.w.256` intrinsic; known as `__builtin_ia32_pcmpgtw256_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.w.256"]
        pub fn avx512_mask_pcmpgt_w_256(a: ::simdty::i16x16, b: ::simdty::i16x16, c: i16) -> i16;
        /// The `llvm.x86.avx512.mask.pcmpgt.d.256` intrinsic; known as `__builtin_ia32_pcmpgtd256_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.d.256"]
        pub fn avx512_mask_pcmpgt_d_256(a: ::simdty::i32x8, b: ::simdty::i32x8, c: i8) -> i8;
        /// The `llvm.x86.avx512.mask.pcmpgt.q.256` intrinsic; known as `__builtin_ia32_pcmpgtq256_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.q.256"]
        pub fn avx512_mask_pcmpgt_q_256(a: ::simdty::i64x4, b: ::simdty::i64x4, c: i8) -> i8;
        /// The `llvm.x86.avx512.mask.pcmpeq.b.128` intrinsic; known as `__builtin_ia32_pcmpeqb128_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.b.128"]
        pub fn avx512_mask_pcmpeq_b_128(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i16) -> i16;
        /// The `llvm.x86.avx512.mask.pcmpeq.w.128` intrinsic; known as `__builtin_ia32_pcmpeqw128_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.w.128"]
        pub fn avx512_mask_pcmpeq_w_128(a: ::simdty::i16x8, b: ::simdty::i16x8, c: i8) -> i8;
        /// The `llvm.x86.avx512.mask.pcmpeq.d.128` intrinsic; known as `__builtin_ia32_pcmpeqd128_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.d.128"]
        pub fn avx512_mask_pcmpeq_d_128(a: ::simdty::i32x4, b: ::simdty::i32x4, c: i8) -> i8;
        /// The `llvm.x86.avx512.mask.pcmpeq.q.128` intrinsic; known as `__builtin_ia32_pcmpeqq128_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.q.128"]
        pub fn avx512_mask_pcmpeq_q_128(a: ::simdty::i64x2, b: ::simdty::i64x2, c: i8) -> i8;
        /// The `llvm.x86.avx512.mask.pcmpgt.b.128` intrinsic; known as `__builtin_ia32_pcmpgtb128_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.b.128"]
        pub fn avx512_mask_pcmpgt_b_128(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i16) -> i16;
        /// The `llvm.x86.avx512.mask.pcmpgt.w.128` intrinsic; known as `__builtin_ia32_pcmpgtw128_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.w.128"]
        pub fn avx512_mask_pcmpgt_w_128(a: ::simdty::i16x8, b: ::simdty::i16x8, c: i8) -> i8;
        /// The `llvm.x86.avx512.mask.pcmpgt.d.128` intrinsic; known as `__builtin_ia32_pcmpgtd128_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.d.128"]
        pub fn avx512_mask_pcmpgt_d_128(a: ::simdty::i32x4, b: ::simdty::i32x4, c: i8) -> i8;
        /// The `llvm.x86.avx512.mask.pcmpgt.q.128` intrinsic; known as `__builtin_ia32_pcmpgtq128_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.q.128"]
        pub fn avx512_mask_pcmpgt_q_128(a: ::simdty::i64x2, b: ::simdty::i64x2, c: i8) -> i8;
        /// The `llvm.x86.avx512.mask.cmp.ps.512` intrinsic; known as `__builtin_ia32_cmpps512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.cmp.ps.512"]
        pub fn avx512_mask_cmp_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: i32, d: i16, e: i32) -> i16;
        /// The `llvm.x86.avx512.mask.cmp.pd.512` intrinsic; known as `__builtin_ia32_cmppd512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.cmp.pd.512"]
        pub fn avx512_mask_cmp_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: i32, d: i8, e: i32) -> i8;
        /// The `llvm.x86.avx512.mask.pand.d.512` intrinsic; known as `__builtin_ia32_pandd512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pand.d.512"]
        pub fn avx512_mask_pand_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: ::simdty::i32x16, d: i16) -> ::simdty::i32x16;
        /// The `llvm.x86.avx512.mask.pand.q.512` intrinsic; known as `__builtin_ia32_pandq512_mask` in GCC.
        #[link_name = "llvm.x86.avx512.mask.pand.q.512"]
        pub fn avx512_mask_pand_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: ::simdty::i64x8, d: i8) -> ::simdty::i64x8;
        /// The `llvm.x86.avx512.movntdqa` intrinsic; known as `__builtin_ia32_movntdqa512` in GCC.
        #[link_name = "llvm.x86.avx512.movntdqa"]
        pub fn avx512_movntdqa(a: *mut i8) -> ::simdty::i64x8;
        /// The `llvm.x86.sha1rnds4` intrinsic; known as `__builtin_ia32_sha1rnds4` in GCC.
        #[link_name = "llvm.x86.sha1rnds4"]
        pub fn sha1rnds4(a: ::simdty::i32x4, b: ::simdty::i32x4, c: i8) -> ::simdty::i32x4;
        /// The `llvm.x86.sha1nexte` intrinsic; known as `__builtin_ia32_sha1nexte` in GCC.
        #[link_name = "llvm.x86.sha1nexte"]
        pub fn sha1nexte(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.sha1msg1` intrinsic; known as `__builtin_ia32_sha1msg1` in GCC.
        #[link_name = "llvm.x86.sha1msg1"]
        pub fn sha1msg1(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.sha1msg2` intrinsic; known as `__builtin_ia32_sha1msg2` in GCC.
        #[link_name = "llvm.x86.sha1msg2"]
        pub fn sha1msg2(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.sha256rnds2` intrinsic; known as `__builtin_ia32_sha256rnds2` in GCC.
        #[link_name = "llvm.x86.sha256rnds2"]
        pub fn sha256rnds2(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.sha256msg1` intrinsic; known as `__builtin_ia32_sha256msg1` in GCC.
        #[link_name = "llvm.x86.sha256msg1"]
        pub fn sha256msg1(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        /// The `llvm.x86.sha256msg2` intrinsic; known as `__builtin_ia32_sha256msg2` in GCC.
        #[link_name = "llvm.x86.sha256msg2"]
        pub fn sha256msg2(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
    }
}