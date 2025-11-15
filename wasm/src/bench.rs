//! Benchmarking utilities for WASM performance measurement
//!
//! These functions are designed to be called from Node.js via wasm-bindgen
//! and measured using performance.now() for accurate WASM performance profiling.

use crate::matrix::Matrix4;
use crate::tuple::Tuple;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BenchOps;

#[wasm_bindgen]
impl BenchOps {
    /// Create a new benchmark suite
    #[wasm_bindgen(constructor)]
    pub fn new() -> BenchOps {
        BenchOps
    }

    /// Benchmark 4x4 matrix multiplication
    ///
    /// Performs `iterations` matrix multiplications with fixed test matrices.
    /// Returns the final result to prevent compiler optimization.
    #[wasm_bindgen]
    pub fn matrix_multiply_bench(&self, iterations: u32) -> f32 {
        let a = Matrix4::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        );
        let b = Matrix4::new(
            -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 0.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0,
        );

        let mut result = 0.0;
        for _ in 0..iterations {
            result = (a * b).get(0, 0);
        }
        result
    }

    /// Benchmark matrix multiplication chain (4 multiplications)
    ///
    /// Useful for measuring accumulated overhead in transformation pipelines.
    /// Returns the final result to prevent compiler optimization.
    #[wasm_bindgen]
    pub fn matrix_chain_multiply_bench(&self, iterations: u32) -> f32 {
        let a = Matrix4::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        );
        let b = Matrix4::new(
            -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 0.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0,
        );

        let mut result = 0.0;
        for _ in 0..iterations {
            result = (a * b * a * b).get(0, 0);
        }
        result
    }

    /// Benchmark matrix-point multiplication
    ///
    /// Point transformations are common in ray tracing.
    /// Returns the final result to prevent compiler optimization.
    #[wasm_bindgen]
    pub fn matrix_point_multiply_bench(&self, iterations: u32) -> f32 {
        let matrix = Matrix4::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        );
        let point = Tuple::point(1.0, 2.0, 3.0);

        let mut result = 0.0;
        for _ in 0..iterations {
            result = (matrix * point).get(0);
        }
        result
    }

    /// Benchmark matrix-vector multiplication
    ///
    /// Vector transformations are used for direction/normal transforms.
    /// Returns the final result to prevent compiler optimization.
    #[wasm_bindgen]
    pub fn matrix_vector_multiply_bench(&self, iterations: u32) -> f32 {
        let matrix = Matrix4::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        );
        let vector = Tuple::vector(2.0, 3.0, 4.0);

        let mut result = 0.0;
        for _ in 0..iterations {
            result = (matrix * vector).get(0);
        }
        result
    }

    /// Benchmark tuple addition
    /// Returns the final result to prevent compiler optimization.
    #[wasm_bindgen]
    pub fn tuple_add_bench(&self, iterations: u32) -> f32 {
        let t1 = Tuple::vector(1.0, 2.0, 3.0);
        let t2 = Tuple::vector(2.0, 3.0, 4.0);

        let mut result = 0.0;
        for _ in 0..iterations {
            result = (t1 + t2).get(0);
        }
        result
    }

    /// Benchmark tuple subtraction
    /// Returns the final result to prevent compiler optimization.
    #[wasm_bindgen]
    pub fn tuple_subtract_bench(&self, iterations: u32) -> f32 {
        let t1 = Tuple::vector(1.0, 2.0, 3.0);
        let t2 = Tuple::vector(2.0, 3.0, 4.0);

        let mut result = 0.0;
        for _ in 0..iterations {
            result = (t1 - t2).get(0);
        }
        result
    }

    /// Benchmark dot product
    ///
    /// Used heavily in ray-sphere intersection calculations.
    /// Returns the final result to prevent compiler optimization.
    #[wasm_bindgen]
    pub fn tuple_dot_product_bench(&self, iterations: u32) -> f32 {
        let t1 = Tuple::vector(1.0, 2.0, 3.0);
        let t2 = Tuple::vector(2.0, 3.0, 4.0);

        let mut result = 0.0;
        for _ in 0..iterations {
            result = t1.dot(t2);
        }
        result
    }

    /// Benchmark cross product
    ///
    /// Used in calculating surface normals.
    /// Returns the final result to prevent compiler optimization.
    #[wasm_bindgen]
    pub fn tuple_cross_product_bench(&self, iterations: u32) -> f32 {
        let t1 = Tuple::vector(1.0, 2.0, 3.0);
        let t2 = Tuple::vector(2.0, 3.0, 4.0);

        let mut result = 0.0;
        for _ in 0..iterations {
            result = t1.cross(t2).get(0);
        }
        result
    }

    /// Benchmark tuple scalar multiplication
    /// Returns the final result to prevent compiler optimization.
    #[wasm_bindgen]
    pub fn tuple_scalar_multiply_bench(&self, iterations: u32) -> f32 {
        let t = Tuple::vector(1.0, 2.0, 3.0);
        let scalar = 2.5;

        let mut result = 0.0;
        for _ in 0..iterations {
            result = (t * scalar).get(0);
        }
        result
    }

    /// Benchmark matrix transpose
    /// Returns the final result to prevent compiler optimization.
    #[wasm_bindgen]
    pub fn matrix_transpose_bench(&self, iterations: u32) -> f32 {
        let matrix = Matrix4::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        );

        let mut result = 0.0;
        for _ in 0..iterations {
            result = matrix.transpose().get(0, 0);
        }
        result
    }

    /// Benchmark matrix determinant calculation
    /// Returns the final result to prevent compiler optimization.
    #[wasm_bindgen]
    pub fn matrix_determinant_bench(&self, iterations: u32) -> f32 {
        let matrix = Matrix4::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        );

        let mut result = 0.0;
        for _ in 0..iterations {
            result = matrix.determinant();
        }
        result
    }
}
