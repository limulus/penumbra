//! SIMD type conversion utilities
//!
//! This module provides safe wrappers for converting between v128 and [f32; 4].
//! The conversions use `transmute` which is justified because v128 and [f32; 4]
//! have identical memory layout (16 bytes, 4-byte alignment).

use std::arch::wasm32::v128;
use std::mem::transmute;

/// Convert v128 to [f32; 4] via transmute
///
/// Safe to use because both types have identical memory layout.
#[inline]
pub fn v128_to_f32x4(v: v128) -> [f32; 4] {
    unsafe { transmute(v) }
}

/// Convert [f32; 4] to v128 via transmute
///
/// Safe to use because both types have identical memory layout.
#[inline]
pub fn f32x4_to_v128(arr: [f32; 4]) -> v128 {
    unsafe { transmute(arr) }
}
