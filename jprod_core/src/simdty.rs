#![allow(non_camel_case_types)]

#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 2 values of type i8 in a single SIMD vector.
pub struct i8x2(pub i8, pub i8);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 2 values of type i16 in a single SIMD vector.
pub struct i16x2(pub i16, pub i16);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 2 values of type i32 in a single SIMD vector.
pub struct i32x2(pub i32, pub i32);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 2 values of type i64 in a single SIMD vector.
pub struct i64x2(pub i64, pub i64);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 2 values of type u8 in a single SIMD vector.
pub struct u8x2(pub u8, pub u8);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 2 values of type u16 in a single SIMD vector.
pub struct u16x2(pub u16, pub u16);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 2 values of type u32 in a single SIMD vector.
pub struct u32x2(pub u32, pub u32);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 2 values of type u64 in a single SIMD vector.
pub struct u64x2(pub u64, pub u64);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 2 values of type f32 in a single SIMD vector.
pub struct f32x2(pub f32, pub f32);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 2 values of type f64 in a single SIMD vector.
pub struct f64x2(pub f64, pub f64);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 4 values of type i8 in a single SIMD vector.
pub struct i8x4(pub i8, pub i8, pub i8, pub i8);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 4 values of type i16 in a single SIMD vector.
pub struct i16x4(pub i16, pub i16, pub i16, pub i16);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 4 values of type i32 in a single SIMD vector.
pub struct i32x4(pub i32, pub i32, pub i32, pub i32);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 4 values of type i64 in a single SIMD vector.
pub struct i64x4(pub i64, pub i64, pub i64, pub i64);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 4 values of type u8 in a single SIMD vector.
pub struct u8x4(pub u8, pub u8, pub u8, pub u8);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 4 values of type u16 in a single SIMD vector.
pub struct u16x4(pub u16, pub u16, pub u16, pub u16);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 4 values of type u32 in a single SIMD vector.
pub struct u32x4(pub u32, pub u32, pub u32, pub u32);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 4 values of type u64 in a single SIMD vector.
pub struct u64x4(pub u64, pub u64, pub u64, pub u64);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 4 values of type f32 in a single SIMD vector.
pub struct f32x4(pub f32, pub f32, pub f32, pub f32);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 4 values of type f64 in a single SIMD vector.
pub struct f64x4(pub f64, pub f64, pub f64, pub f64);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 8 values of type i8 in a single SIMD vector.
pub struct i8x8(pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 8 values of type i16 in a single SIMD vector.
pub struct i16x8(pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 8 values of type i32 in a single SIMD vector.
pub struct i32x8(pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 8 values of type i64 in a single SIMD vector.
pub struct i64x8(pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 8 values of type u8 in a single SIMD vector.
pub struct u8x8(pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 8 values of type u16 in a single SIMD vector.
pub struct u16x8(pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 8 values of type u32 in a single SIMD vector.
pub struct u32x8(pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 8 values of type u64 in a single SIMD vector.
pub struct u64x8(pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 8 values of type f32 in a single SIMD vector.
pub struct f32x8(pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 8 values of type f64 in a single SIMD vector.
pub struct f64x8(pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 16 values of type i8 in a single SIMD vector.
pub struct i8x16(pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 16 values of type i16 in a single SIMD vector.
pub struct i16x16(pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 16 values of type i32 in a single SIMD vector.
pub struct i32x16(pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 16 values of type i64 in a single SIMD vector.
pub struct i64x16(pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 16 values of type u8 in a single SIMD vector.
pub struct u8x16(pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 16 values of type u16 in a single SIMD vector.
pub struct u16x16(pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 16 values of type u32 in a single SIMD vector.
pub struct u32x16(pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 16 values of type u64 in a single SIMD vector.
pub struct u64x16(pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 16 values of type f32 in a single SIMD vector.
pub struct f32x16(pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 16 values of type f64 in a single SIMD vector.
pub struct f64x16(pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 32 values of type i8 in a single SIMD vector.
pub struct i8x32(pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 32 values of type i16 in a single SIMD vector.
pub struct i16x32(pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 32 values of type i32 in a single SIMD vector.
pub struct i32x32(pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 32 values of type i64 in a single SIMD vector.
pub struct i64x32(pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 32 values of type u8 in a single SIMD vector.
pub struct u8x32(pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 32 values of type u16 in a single SIMD vector.
pub struct u16x32(pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 32 values of type u32 in a single SIMD vector.
pub struct u32x32(pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 32 values of type u64 in a single SIMD vector.
pub struct u64x32(pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 32 values of type f32 in a single SIMD vector.
pub struct f32x32(pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 32 values of type f64 in a single SIMD vector.
pub struct f64x32(pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 64 values of type i8 in a single SIMD vector.
pub struct i8x64(pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8, pub i8);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 64 values of type i16 in a single SIMD vector.
pub struct i16x64(pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16, pub i16);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 64 values of type i32 in a single SIMD vector.
pub struct i32x64(pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 64 values of type i64 in a single SIMD vector.
pub struct i64x64(pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64, pub i64);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 64 values of type u8 in a single SIMD vector.
pub struct u8x64(pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 64 values of type u16 in a single SIMD vector.
pub struct u16x64(pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16, pub u16);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 64 values of type u32 in a single SIMD vector.
pub struct u32x64(pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 64 values of type u64 in a single SIMD vector.
pub struct u64x64(pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 64 values of type f32 in a single SIMD vector.
pub struct f32x64(pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32);
#[repr(simd)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// 64 values of type f64 in a single SIMD vector.
pub struct f64x64(pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub f64);
