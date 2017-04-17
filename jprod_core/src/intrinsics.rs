extern "platform-intrinsic" {
    pub fn simd_eq<T, U>(x: T, y: T) -> U;
    pub fn simd_ne<T, U>(x: T, y: T) -> U;
    pub fn simd_lt<T, U>(x: T, y: T) -> U;
    pub fn simd_le<T, U>(x: T, y: T) -> U;
    pub fn simd_gt<T, U>(x: T, y: T) -> U;
    pub fn simd_ge<T, U>(x: T, y: T) -> U;

    pub fn simd_shuffle2<T, U>(v: T, w: T, idx: [u32; 2]) -> U;
    pub fn simd_shuffle4<T, U>(v: T, w: T, idx: [u32; 4]) -> U;
    pub fn simd_shuffle8<T, U>(v: T, w: T, idx: [u32; 8]) -> U;
    pub fn simd_shuffle16<T, U>(v: T, w: T, idx: [u32; 16]) -> U;

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
    pub fn debugtrap() -> !;
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