use std::cmp::PartialEq;
use std::ops::Mul;

use crate::fuzzy::{fuzzy_eq_f32, fuzzy_eq_f32x4};
use crate::simd::f32x4;
use crate::tuple::Tuple;

const IDENTITY: [f32; 16] = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
];

#[derive(Clone, Copy, Debug)]
#[repr(C, align(16))]
pub struct Matrix4 {
    data: [f32; 16],
}

impl Matrix4 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        m00: f32,
        m01: f32,
        m02: f32,
        m03: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m13: f32,
        m20: f32,
        m21: f32,
        m22: f32,
        m23: f32,
        m30: f32,
        m31: f32,
        m32: f32,
        m33: f32,
    ) -> Matrix4 {
        Matrix4 {
            data: [
                m00, m10, m20, m30, m01, m11, m21, m31, m02, m12, m22, m32, m03, m13, m23,
                m33,
            ],
        }
    }

    pub fn identity() -> Matrix4 {
        Matrix4 { data: IDENTITY }
    }

    #[allow(clippy::identity_op, clippy::erasing_op)]
    pub fn rotation_x(r: f32) -> Matrix4 {
        let mut data: [f32; 16] = IDENTITY;
        data[1 * 4 + 1] = r.cos();
        data[2 * 4 + 1] = -r.sin();
        data[1 * 4 + 2] = r.sin();
        data[2 * 4 + 2] = r.cos();
        Matrix4 { data }
    }

    #[allow(clippy::identity_op, clippy::erasing_op)]
    pub fn rotation_y(r: f32) -> Matrix4 {
        let mut data: [f32; 16] = IDENTITY;
        data[0 * 4 + 0] = r.cos();
        data[2 * 4 + 0] = r.sin();
        data[0 * 4 + 2] = -r.sin();
        data[2 * 4 + 2] = r.cos();
        Matrix4 { data }
    }

    #[allow(clippy::identity_op, clippy::erasing_op)]
    pub fn rotation_z(r: f32) -> Matrix4 {
        let mut data: [f32; 16] = IDENTITY;
        data[0 * 4 + 0] = r.cos();
        data[1 * 4 + 0] = -r.sin();
        data[0 * 4 + 1] = r.sin();
        data[1 * 4 + 1] = r.cos();
        Matrix4 { data }
    }

    #[allow(clippy::identity_op, clippy::erasing_op)]
    pub fn scaling(x: f32, y: f32, z: f32) -> Matrix4 {
        let mut data: [f32; 16] = [0.0; 16];
        data[0 * 4 + 0] = x;
        data[1 * 4 + 1] = y;
        data[2 * 4 + 2] = z;
        data[3 * 4 + 3] = 1.0;
        Matrix4 { data }
    }

    #[allow(clippy::identity_op, clippy::erasing_op)]
    pub fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Matrix4 {
        let mut data: [f32; 16] = IDENTITY;
        data[1 * 4 + 0] = xy;
        data[2 * 4 + 0] = xz;
        data[0 * 4 + 1] = yx;
        data[2 * 4 + 1] = yz;
        data[0 * 4 + 2] = zx;
        data[1 * 4 + 2] = zy;
        Matrix4 { data }
    }

    #[allow(clippy::identity_op, clippy::erasing_op)]
    pub fn translation(x: f32, y: f32, z: f32) -> Matrix4 {
        let mut data: [f32; 16] = IDENTITY;
        data[3 * 4 + 0] = x;
        data[3 * 4 + 1] = y;
        data[3 * 4 + 2] = z;
        Matrix4 { data }
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.data[col * 4 + row]
    }

    #[inline]
    pub fn col_f32x4(&self, col: usize) -> f32x4 {
        assert!(col < 4);
        f32x4::new([
            self.data[col * 4],
            self.data[col * 4 + 1],
            self.data[col * 4 + 2],
            self.data[col * 4 + 3],
        ])
    }

    /// Helper to safely store an f32x4 column into the matrix data
    #[inline]
    fn store_column(data: &mut [f32; 16], column: usize, value: f32x4) {
        let values = value.to_array();
        let base = column * 4;
        data[base] = values[0];
        data[base + 1] = values[1];
        data[base + 2] = values[2];
        data[base + 3] = values[3];
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        let minor = self.minor(row, col);
        if (row + col).is_multiple_of(2) {
            minor
        } else {
            -minor
        }
    }

    pub fn determinant(&self) -> f32 {
        self.get(0, 0) * self.cofactor(0, 0)
            + self.get(0, 1) * self.cofactor(0, 1)
            + self.get(0, 2) * self.cofactor(0, 2)
            + self.get(0, 3) * self.cofactor(0, 3)
    }

    pub fn inverse(&self) -> Option<Matrix4> {
        if !self.is_invertible() {
            return None;
        }

        let mut data: [f32; 16] = [0.0; 16];
        let determinant = self.determinant();

        for row in 0..4 {
            for col in 0..4 {
                let c = self.cofactor(row, col);
                data[row * 4 + col] = c / determinant;
            }
        }

        Some(Matrix4 { data })
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn minor(&self, row: usize, col: usize) -> f32 {
        self.submatrix(row, col).determinant()
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix3 {
        let mut data: [f32; 9] = [0.0; 9];
        let mut i = 0;
        for r in 0..4 {
            for c in 0..4 {
                if r != row && c != col {
                    data[i] = self.get(r, c);
                    i += 1;
                }
            }
        }
        Matrix3::new(
            data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8],
        )
    }

    pub fn transpose(&self) -> Matrix4 {
        Self::new(
            self.get(0, 0),
            self.get(1, 0),
            self.get(2, 0),
            self.get(3, 0),
            self.get(0, 1),
            self.get(1, 1),
            self.get(2, 1),
            self.get(3, 1),
            self.get(0, 2),
            self.get(1, 2),
            self.get(2, 2),
            self.get(3, 2),
            self.get(0, 3),
            self.get(1, 3),
            self.get(2, 3),
            self.get(3, 3),
        )
    }
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..4 {
            if !fuzzy_eq_f32x4(self.col_f32x4(i), other.col_f32x4(i)) {
                return false;
            }
        }
        true
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut data: [f32; 16] = [0.0; 16];

        for i in 0..4 {
            let mut sum = f32x4::splat(0.0);
            for j in 0..4 {
                sum += f32x4::splat(other.get(j, i)) * self.col_f32x4(j);
            }
            Self::store_column(&mut data, i, sum);
        }

        Matrix4 { data }
    }
}

impl Mul<Tuple> for Matrix4 {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        &self * other
    }
}

impl Mul<Tuple> for &Matrix4 {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        let mut sum = f32x4::splat(0.0);
        for i in 0..4 {
            sum += f32x4::splat(other.get(i)) * self.col_f32x4(i);
        }
        Tuple::from_f32x4(sum)
    }
}

#[derive(Clone, Debug)]
pub struct Transform {
    operations: Vec<Matrix4>,
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            operations: Vec::new(),
        }
    }

    pub fn translate(self, x: f32, y: f32, z: f32) -> Self {
        let mut operations = self.operations;
        operations.push(Matrix4::translation(x, y, z));
        Self { operations }
    }

    pub fn scale(self, x: f32, y: f32, z: f32) -> Self {
        let mut operations = self.operations;
        operations.push(Matrix4::scaling(x, y, z));
        Self { operations }
    }

    pub fn rotate_x(self, r: f32) -> Self {
        let mut operations = self.operations;
        operations.push(Matrix4::rotation_x(r));
        Self { operations }
    }

    pub fn rotate_y(self, r: f32) -> Self {
        let mut operations = self.operations;
        operations.push(Matrix4::rotation_y(r));
        Self { operations }
    }

    pub fn rotate_z(self, r: f32) -> Self {
        let mut operations = self.operations;
        operations.push(Matrix4::rotation_z(r));
        Self { operations }
    }

    pub fn shear(self, xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Self {
        let mut operations = self.operations;
        operations.push(Matrix4::shearing(xy, xz, yx, yz, zx, zy));
        Self { operations }
    }

    pub fn build(self) -> Matrix4 {
        if self.operations.is_empty() {
            Matrix4::identity()
        } else {
            self.operations
                .iter()
                .cloned()
                .rev()
                .reduce(|a, b| a * b)
                .expect("Transform should have at least one operation")
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Matrix2 {
    data: [f32; 4],
}

impl Matrix2 {
    pub fn new(m00: f32, m01: f32, m10: f32, m11: f32) -> Matrix2 {
        Matrix2 {
            data: [m00, m10, m01, m11],
        }
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.data[col * 2 + row]
    }

    pub fn determinant(&self) -> f32 {
        self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0)
    }
}

impl PartialEq for Matrix2 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..2 {
            for j in 0..2 {
                if !fuzzy_eq_f32(self.get(i, j), other.get(i, j)) {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(Clone, Copy, Debug)]
struct Matrix3 {
    data: [f32; 9],
}

impl Matrix3 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        m00: f32,
        m01: f32,
        m02: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m20: f32,
        m21: f32,
        m22: f32,
    ) -> Matrix3 {
        Matrix3 {
            data: [m00, m10, m20, m01, m11, m21, m02, m12, m22],
        }
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.data[col * 3 + row]
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        let minor = self.minor(row, col);
        if (row + col).is_multiple_of(2) {
            minor
        } else {
            -minor
        }
    }

    pub fn determinant(&self) -> f32 {
        self.get(0, 0) * self.cofactor(0, 0)
            + self.get(0, 1) * self.cofactor(0, 1)
            + self.get(0, 2) * self.cofactor(0, 2)
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2 {
        let mut data: [f32; 4] = [0.0; 4];
        let mut i = 0;
        for r in 0..3 {
            for c in 0..3 {
                if r != row && c != col {
                    data[i] = self.get(r, c);
                    i += 1;
                }
            }
        }
        Matrix2::new(data[0], data[1], data[2], data[3])
    }

    pub fn minor(&self, row: usize, col: usize) -> f32 {
        self.submatrix(row, col).determinant()
    }
}

impl PartialEq for Matrix3 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if !fuzzy_eq_f32(self.get(i, j), other.get(i, j)) {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    pub fn constructing_and_inspecting_a_4x4_matrix() {
        let m = Matrix4::new(
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5,
            15.5, 16.5,
        );

        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(0, 3), 4.0);
        assert_eq!(m.get(1, 0), 5.5);
        assert_eq!(m.get(1, 2), 7.5);
        assert_eq!(m.get(2, 2), 11.0);
        assert_eq!(m.get(3, 0), 13.5);
        assert_eq!(m.get(3, 2), 15.5);
    }

    #[wasm_bindgen_test]
    pub fn a_2x2_matrix_ought_to_be_representable() {
        let m = Matrix2::new(-3.0, 5.0, 1.0, -2.0);

        assert_eq!(m.get(0, 0), -3.0);
        assert_eq!(m.get(0, 1), 5.0);
        assert_eq!(m.get(1, 0), 1.0);
        assert_eq!(m.get(1, 1), -2.0);
    }

    #[wasm_bindgen_test]
    pub fn a_3x3_matrix_ought_to_be_representable() {
        let m = Matrix3 {
            data: [-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0],
        };

        assert_eq!(m.get(0, 0), -3.0);
        assert_eq!(m.get(1, 1), -2.0);
        assert_eq!(m.get(2, 2), 1.0);
    }

    #[wasm_bindgen_test]
    pub fn matrix_equality_with_identical_matrices() {
        let a = Matrix4::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        );
        let b = Matrix4::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        );

        assert_eq!(a, b);
    }

    #[wasm_bindgen_test]
    pub fn matrix_equality_with_different_matrices() {
        let a = Matrix4::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        );
        let b = Matrix4::new(
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        );

        assert_ne!(a, b);
    }

    #[wasm_bindgen_test]
    pub fn multiplying_two_matrices() {
        let a = Matrix4::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        );
        let b = Matrix4::new(
            -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0,
            8.0,
        );
        let expected = Matrix4::new(
            20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0,
            16.0, 26.0, 46.0, 42.0,
        );

        assert_eq!(a * b, expected);
    }

    #[wasm_bindgen_test]
    pub fn a_matrix_multiplied_by_a_tuple() {
        let a = Matrix4::new(
            1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        );
        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);

        assert_eq!(a * b, Tuple::new(18.0, 24.0, 33.0, 1.0));
    }

    #[wasm_bindgen_test]
    pub fn multiplying_a_matrix_by_the_identity_matrix() {
        let a = Matrix4::new(
            0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0,
            32.0,
        );
        let expected = Matrix4::new(
            0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0,
            32.0,
        );

        assert_eq!(a * Matrix4::identity(), expected);
    }

    #[wasm_bindgen_test]
    pub fn multiplying_the_identity_matrix_by_a_tuple() {
        let a = Tuple::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(Matrix4::identity() * a, a);
    }

    #[wasm_bindgen_test]
    pub fn transposing_a_matrix() {
        let a = Matrix4::new(
            0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
        );
        let expected = Matrix4::new(
            0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0,
        );

        assert_eq!(a.transpose(), expected);
    }

    #[wasm_bindgen_test]
    pub fn transposing_the_identity_matrix() {
        assert_eq!(Matrix4::identity().transpose(), Matrix4::identity());
    }

    #[wasm_bindgen_test]
    pub fn calculating_the_determinant_of_a_2x2_matrix() {
        let a = Matrix2::new(1.0, 5.0, -3.0, 2.0);

        assert_eq!(a.determinant(), 17.0);
    }

    #[wasm_bindgen_test]
    pub fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        let a = Matrix3::new(1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0);
        let expected = Matrix2::new(-3.0, 2.0, 0.0, 6.0);

        assert_eq!(a.submatrix(0, 2), expected);
    }

    #[wasm_bindgen_test]
    pub fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
        let a = Matrix4::new(
            -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0,
            1.0,
        );
        let expected = Matrix3::new(-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0);

        assert_eq!(a.submatrix(2, 1), expected);
    }

    #[wasm_bindgen_test]
    pub fn calculating_a_minor_of_a_3x3_matrix() {
        let a = Matrix3::new(3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0);
        let b = a.submatrix(1, 0);

        assert_eq!(b.determinant(), 25.0);
        assert_eq!(a.minor(1, 0), 25.0);
    }

    #[wasm_bindgen_test]
    pub fn calculating_a_cofactor_of_a_3x3_matrix() {
        let a = Matrix3::new(3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0);

        assert_eq!(a.minor(0, 0), -12.0);
        assert_eq!(a.cofactor(0, 0), -12.0);
        assert_eq!(a.minor(1, 0), 25.0);
        assert_eq!(a.cofactor(1, 0), -25.0);
    }

    #[wasm_bindgen_test]
    pub fn calculating_the_determinant_of_a_3x3_matrix() {
        let a = Matrix3::new(1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0);

        assert_eq!(a.cofactor(0, 0), 56.0);
        assert_eq!(a.cofactor(0, 1), 12.0);
        assert_eq!(a.cofactor(0, 2), -46.0);
        assert_eq!(a.determinant(), -196.0);
    }

    #[wasm_bindgen_test]
    pub fn calculating_the_determinant_of_a_4x4_matrix() {
        let a = Matrix4::new(
            -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0,
            -9.0,
        );

        assert_eq!(a.cofactor(0, 0), 690.0);
        assert_eq!(a.cofactor(0, 1), 447.0);
        assert_eq!(a.cofactor(0, 2), 210.0);
        assert_eq!(a.cofactor(0, 3), 51.0);
        assert_eq!(a.determinant(), -4071.0);
    }

    #[wasm_bindgen_test]
    pub fn testing_an_invertible_matrix_for_invertibility() {
        let a = Matrix4::new(
            6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0,
            -6.0,
        );

        assert_eq!(a.determinant(), -2120.0);
        assert!(a.is_invertible());
    }

    #[wasm_bindgen_test]
    pub fn testing_a_noninvertible_matrix_for_invertibility() {
        let a = Matrix4::new(
            -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0,
            0.0,
        );

        assert_eq!(a.determinant(), 0.0);
        assert!(!a.is_invertible());
    }

    #[wasm_bindgen_test]
    pub fn calculating_the_inverse_of_a_matrix() {
        let a = Matrix4::new(
            -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0,
            7.0, 4.0,
        );
        let b = a.inverse().unwrap();
        let expected = Matrix4::new(
            0.21805, 0.45113, 0.24060, -0.04511, -0.80827, -1.45677, -0.44361, 0.52068,
            -0.07895, -0.22368, -0.05263, 0.19737, -0.52256, -0.81391, -0.30075, 0.30639,
        );

        assert_eq!(a.determinant(), 532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        assert_eq!(b.get(3, 2), -160.0 / 532.0);
        assert_eq!(a.cofactor(3, 2), 105.0);
        assert_eq!(b.get(2, 3), 105.0 / 532.0);
        assert_eq!(b, expected);
    }

    #[wasm_bindgen_test]
    pub fn calculating_the_inverse_of_another_matrix() {
        let a = Matrix4::new(
            8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0,
            -4.0,
        );
        let b = a.inverse().unwrap();
        let expected = Matrix4::new(
            -0.15385, -0.15385, -0.28205, -0.53846, -0.07692, 0.12308, 0.02564, 0.03077,
            0.35897, 0.35897, 0.43590, 0.92308, -0.69231, -0.69231, -0.76923, -1.92308,
        );

        assert_eq!(b, expected);
    }

    #[wasm_bindgen_test]
    pub fn calculating_the_inverse_of_a_third_matrix() {
        let a = Matrix4::new(
            9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, -4.0, 9.0, 6.0, 4.0, -7.0, 6.0,
            6.0, 2.0,
        );
        let b = a.inverse().unwrap();
        let expected = Matrix4::new(
            -0.04074, -0.07778, 0.14444, -0.22222, -0.07778, 0.03333, 0.36667, -0.33333,
            -0.02901, -0.14630, -0.10926, 0.12963, 0.17778, 0.06667, -0.26667, 0.33333,
        );

        assert_eq!(b, expected);
    }

    #[wasm_bindgen_test]
    pub fn multiplying_a_product_by_its_inverse() {
        let a = Matrix4::new(
            3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0,
            -1.0, 1.0,
        );
        let b = Matrix4::new(
            8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0,
            5.0,
        );
        let c = a * b;

        assert_eq!(c * b.inverse().unwrap(), a);
    }

    #[wasm_bindgen_test]
    pub fn multiplying_by_a_translation_matrix() {
        let transform = Matrix4::translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(transform * p, Tuple::point(2.0, 1.0, 7.0));
    }

    #[wasm_bindgen_test]
    pub fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = Matrix4::translation(5.0, -3.0, 2.0);
        let inv = transform.inverse().unwrap();
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(inv * p, Tuple::point(-8.0, 7.0, 3.0));
    }

    #[wasm_bindgen_test]
    pub fn translation_does_not_affect_vectors() {
        let transform = Matrix4::translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);

        assert_eq!(transform * v, v);
    }

    #[wasm_bindgen_test]
    pub fn a_scaling_matrix_applied_to_a_point() {
        let transform = Matrix4::scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);

        assert_eq!(transform * p, Tuple::point(-8.0, 18.0, 32.0));
    }

    #[wasm_bindgen_test]
    pub fn a_scaling_matrix_applied_to_a_vector() {
        let transform = Matrix4::scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(transform * v, Tuple::vector(-8.0, 18.0, 32.0));
    }

    #[wasm_bindgen_test]
    pub fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = Matrix4::scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse().unwrap();
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(inv * v, Tuple::vector(-2.0, 2.0, 2.0));
    }

    #[wasm_bindgen_test]
    pub fn reflection_is_scaling_by_a_negative_value() {
        let transform = Matrix4::scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(-2.0, 3.0, 4.0));
    }

    #[wasm_bindgen_test]
    pub fn rotating_a_point_around_the_x_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix4::rotation_x(std::f32::consts::PI / 4.0);
        let full_quarter = Matrix4::rotation_x(std::f32::consts::PI / 2.0);

        assert_eq!(
            half_quarter * p,
            Tuple::point(0.0, 2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, Tuple::point(0.0, 0.0, 1.0));
    }

    #[wasm_bindgen_test]
    pub fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix4::rotation_x(std::f32::consts::PI / 4.0);
        let inv = half_quarter.inverse().unwrap();

        assert_eq!(
            inv * p,
            Tuple::point(0.0, 2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0)
        );
    }

    #[wasm_bindgen_test]
    pub fn rotating_a_point_around_the_y_axis() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = Matrix4::rotation_y(std::f32::consts::PI / 4.0);
        let full_quarter = Matrix4::rotation_y(std::f32::consts::PI / 2.0);

        assert_eq!(
            half_quarter * p,
            Tuple::point(2.0_f32.sqrt() / 2.0, 0.0, 2.0_f32.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, Tuple::point(1.0, 0.0, 0.0));
    }

    #[wasm_bindgen_test]
    pub fn rotating_a_point_around_the_z_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix4::rotation_z(std::f32::consts::PI / 4.0);
        let full_quarter = Matrix4::rotation_z(std::f32::consts::PI / 2.0);

        assert_eq!(
            half_quarter * p,
            Tuple::point(-2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0, 0.0)
        );
        assert_eq!(full_quarter * p, Tuple::point(-1.0, 0.0, 0.0));
    }

    #[wasm_bindgen_test]
    pub fn a_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Matrix4::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(5.0, 3.0, 4.0));
    }

    #[wasm_bindgen_test]
    pub fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Matrix4::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(6.0, 3.0, 4.0));
    }

    #[wasm_bindgen_test]
    pub fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Matrix4::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(2.0, 5.0, 4.0));
    }

    #[wasm_bindgen_test]
    pub fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Matrix4::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(2.0, 7.0, 4.0));
    }

    #[wasm_bindgen_test]
    pub fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Matrix4::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(2.0, 3.0, 6.0));
    }

    #[wasm_bindgen_test]
    pub fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Matrix4::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(2.0, 3.0, 7.0));
    }

    #[wasm_bindgen_test]
    pub fn individual_transformations_are_applied_in_sequence() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = Matrix4::rotation_x(std::f32::consts::PI / 2.0);
        let b = Matrix4::scaling(5.0, 5.0, 5.0);
        let c = Matrix4::translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_eq!(p2, Tuple::point(1.0, -1.0, 0.0));

        let p3 = b * p2;
        assert_eq!(p3, Tuple::point(5.0, -5.0, 0.0));

        let p4 = c * p3;
        assert_eq!(p4, Tuple::point(15.0, 0.0, 7.0));
    }

    #[wasm_bindgen_test]
    pub fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let t = Transform::new()
            .rotate_x(std::f32::consts::PI / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0)
            .build();
        assert_eq!(t * p, Tuple::point(15.0, 0.0, 7.0));
    }
}
