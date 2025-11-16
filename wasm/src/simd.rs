//! SIMD type conversion utilities
//!
//! This module provides wrappers for the `wide` crate's f32x4 type.
//! The `wide` crate provides portable SIMD operations that work across
//! different architectures, including both SIMD and non-SIMD WASM targets.

pub use wide::f32x4;
