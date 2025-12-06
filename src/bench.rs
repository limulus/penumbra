//! Benchmarking utilities for WASM performance measurement
//!
//! These functions are designed to be called from Node.js via wasm-bindgen
//! and measured using performance.now() for accurate WASM performance profiling.

use crate::light::Light;
use crate::material::Material;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::transform::Transform;
use crate::tuple::Tuple;
use crate::world::World;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Default)]
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
            -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 0.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0,
            1.0,
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
            -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 0.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0,
            1.0,
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

    /// Benchmark world color_at calculation (core rendering operation)
    ///
    /// Creates a small test world with 3 spheres and calculates the color at a ray's intersection.
    /// This benchmarks the complete rendering pipeline: ray-sphere intersection, hit determination,
    /// and Phong lighting calculation.
    /// Returns the final result to prevent compiler optimization.
    #[wasm_bindgen]
    pub fn world_color_at_bench(&self, iterations: u32) -> f32 {
        // Create a world with 3 spheres and a light
        let mut world = World::new();
        world.light = Some(Light::new(
            Tuple::point(-10.0, 10.0, -10.0),
            Tuple::color(1.0, 1.0, 1.0),
        ));

        // Sphere 1: Default material, scaled
        let mut s1 = Sphere::new();
        s1.set_transform(Transform::new().scale(2.0, 2.0, 2.0))
            .unwrap();
        world.objects.push(s1);

        // Sphere 2: Custom material, translated
        let mut s2 = Sphere::new();
        s2.set_transform(Transform::new().translate(1.0, 0.0, 0.0))
            .unwrap();
        s2.material = Material {
            color: Tuple::color(0.8, 0.1, 0.1),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        };
        world.objects.push(s2);

        // Sphere 3: Another custom material, different position
        let mut s3 = Sphere::new();
        s3.set_transform(
            Transform::new()
                .translate(-1.0, 0.5, 0.5)
                .scale(0.5, 0.5, 0.5),
        )
        .unwrap();
        s3.material = Material {
            color: Tuple::color(0.1, 0.1, 0.8),
            ambient: 0.2,
            diffuse: 0.8,
            specular: 0.5,
            shininess: 100.0,
        };
        world.objects.push(s3);

        // Create a ray that will intersect the world
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let mut result = 0.0;
        for _ in 0..iterations {
            let color = world.color_at(ray);
            result = color.get(0);
        }
        result
    }
}
