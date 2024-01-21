use std::arch::wasm32::*;
use std::cmp::PartialEq;
use std::ops::Mul;

use crate::fuzzy::{fuzzy_eq_f32x4, fuzzy_eq_f32};
use crate::tuple::Tuple;

const IDENTITY: [f32; 16] = [
  1.0, 0.0, 0.0, 0.0,
  0.0, 1.0, 0.0, 0.0,
  0.0, 0.0, 1.0, 0.0,
  0.0, 0.0, 0.0, 1.0,
];

#[derive(Clone, Copy, Debug)]
#[repr(C, align(16))]
pub struct Matrix4 {
  data: [f32; 16]
}

impl Matrix4 {
  pub fn new(
    m00: f32, m01: f32, m02: f32, m03: f32,
    m10: f32, m11: f32, m12: f32, m13: f32,
    m20: f32, m21: f32, m22: f32, m23: f32,
    m30: f32, m31: f32, m32: f32, m33: f32,
  ) -> Matrix4 {
    Matrix4 {
      data: [
        m00, m10, m20, m30,
        m01, m11, m21, m31,
        m02, m12, m22, m32,
        m03, m13, m23, m33,
      ]
    }
  }

  pub fn identity() -> Matrix4 {
    Matrix4 { data: IDENTITY }
  }

  pub fn get(&self, row: usize, col: usize) -> f32 {
    self.data[col * 4 + row]
  }

  #[inline]
  pub fn col_v128(&self, col: usize) -> v128 {
    assert!(col < 4);
    unsafe {
      *(self.data.as_ptr().add(col * 4) as *const v128)
    }
  }

  pub fn cofactor(&self, row: usize, col: usize) -> f32 {
    let minor = self.minor(row, col);
    if (row + col) % 2 == 0 {
      minor
    } else {
      -minor
    }
  }

  pub fn determinant(&self) -> f32 {
    self.get(0, 0) * self.cofactor(0, 0) +
    self.get(0, 1) * self.cofactor(0, 1) +
    self.get(0, 2) * self.cofactor(0, 2) +
    self.get(0, 3) * self.cofactor(0, 3)
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
      data[0], data[1], data[2],
      data[3], data[4], data[5],
      data[6], data[7], data[8],
    )
  }

  pub fn transpose(&self) -> Matrix4 {
    Self::new(
      self.get(0, 0), self.get(1, 0), self.get(2, 0), self.get(3, 0),
      self.get(0, 1), self.get(1, 1), self.get(2, 1), self.get(3, 1),
      self.get(0, 2), self.get(1, 2), self.get(2, 2), self.get(3, 2),
      self.get(0, 3), self.get(1, 3), self.get(2, 3), self.get(3, 3),
    )
  }
}

impl PartialEq for Matrix4 {
  fn eq(&self, other: &Self) -> bool {
    for i in 0..4 {
      if !fuzzy_eq_f32x4(self.col_v128(i), other.col_v128(i)) {
        return false;
      }
    }
    true
  }
}

impl Mul<Matrix4> for Matrix4 {
  type Output = Self;

  fn mul(self, other: Self) -> Self {
    let data: [f32; 16] = [0.0; 16];

    for i in 0..4 {
      let mut sum = f32x4_splat(0.0);
      for j in 0..4 {
        sum = f32x4_add(
          sum,
          f32x4_mul(
            f32x4_splat(other.get(j, i)),
            self.col_v128(j),
          )
        );
      }
      unsafe { v128_store((data.as_ptr().add(i * 4)) as *mut v128, sum) }
    }

    Matrix4 { data }
  }
}

impl Mul<Tuple> for Matrix4 {
  type Output = Tuple;

  fn mul(self, other: Tuple) -> Tuple {
    let mut sum = f32x4_splat(0.0);
    for i in 0..4 {
      sum = f32x4_add(
        sum,
        f32x4_mul(
          f32x4_splat(other.get(i)),
          self.col_v128(i),
        )
      );
    }
    Tuple::from_v128(sum)
  }
}

#[derive(Clone, Copy, Debug)]
struct Matrix2 {
  data: [f32; 4]
}

impl Matrix2 {
  pub fn new(
    m00: f32, m01: f32,
    m10: f32, m11: f32,
  ) -> Matrix2 {
    Matrix2 {
      data: [
        m00, m10,
        m01, m11,
      ]
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
  data: [f32; 9]
}

impl Matrix3 {
  pub fn new(
    m00: f32, m01: f32, m02: f32,
    m10: f32, m11: f32, m12: f32,
    m20: f32, m21: f32, m22: f32,
  ) -> Matrix3 {
    Matrix3 {
      data: [
        m00, m10, m20,
        m01, m11, m21,
        m02, m12, m22,
      ]
    }
  }

  pub fn get(&self, row: usize, col: usize) -> f32 {
    self.data[col * 3 + row]
  }

  pub fn cofactor(&self, row: usize, col: usize) -> f32 {
    let minor = self.minor(row, col);
    if (row + col) % 2 == 0 {
      minor
    } else {
      -minor
    }
  }

  pub fn determinant(&self) -> f32 {
    self.get(0, 0) * self.cofactor(0, 0) +
    self.get(0, 1) * self.cofactor(0, 1) +
    self.get(0, 2) * self.cofactor(0, 2)
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
    Matrix2::new(
      data[0], data[1],
      data[2], data[3],
    )
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
      1.0, 2.0, 3.0, 4.0,
      5.5, 6.5, 7.5, 8.5,
      9.0, 10.0, 11.0, 12.0,
      13.5, 14.5, 15.5, 16.5,
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
    let m = Matrix2::new(
      -3.0, 5.0,
      1.0, -2.0,
    );

    assert_eq!(m.get(0, 0), -3.0);
    assert_eq!(m.get(0, 1), 5.0);
    assert_eq!(m.get(1, 0), 1.0);
    assert_eq!(m.get(1, 1), -2.0);
  }

  #[wasm_bindgen_test]
  pub fn a_3x3_matrix_ought_to_be_representable() {
    let m = Matrix3 {
      data: [
        -3.0, 5.0, 0.0,
        1.0, -2.0, -7.0,
        0.0, 1.0, 1.0,
      ]
    };

    assert_eq!(m.get(0, 0), -3.0);
    assert_eq!(m.get(1, 1), -2.0);
    assert_eq!(m.get(2, 2), 1.0);
  }

  #[wasm_bindgen_test]
  pub fn matrix_equality_with_identical_matrices() {
    let a = Matrix4::new(
      1.0, 2.0, 3.0, 4.0,
      5.0, 6.0, 7.0, 8.0,
      9.0, 8.0, 7.0, 6.0,
      5.0, 4.0, 3.0, 2.0,
    );
    let b = Matrix4::new(
      1.0, 2.0, 3.0, 4.0,
      5.0, 6.0, 7.0, 8.0,
      9.0, 8.0, 7.0, 6.0,
      5.0, 4.0, 3.0, 2.0,
    );

    assert_eq!(a, b);
  }

  #[wasm_bindgen_test]
  pub fn matrix_equality_with_different_matrices() {
    let a = Matrix4::new(
      1.0, 2.0, 3.0, 4.0,
      5.0, 6.0, 7.0, 8.0,
      9.0, 8.0, 7.0, 6.0,
      5.0, 4.0, 3.0, 2.0,
    );
    let b = Matrix4::new(
      2.0, 3.0, 4.0, 5.0,
      6.0, 7.0, 8.0, 9.0,
      8.0, 7.0, 6.0, 5.0,
      4.0, 3.0, 2.0, 1.0,
    );

    assert_ne!(a, b);
  }

  #[wasm_bindgen_test]
  pub fn multiplying_two_matrices() {
    let a = Matrix4::new(
      1.0, 2.0, 3.0, 4.0,
      5.0, 6.0, 7.0, 8.0,
      9.0, 8.0, 7.0, 6.0,
      5.0, 4.0, 3.0, 2.0,
    );
    let b = Matrix4::new(
      -2.0, 1.0, 2.0, 3.0,
      3.0, 2.0, 1.0, -1.0,
      4.0, 3.0, 6.0, 5.0,
      1.0, 2.0, 7.0, 8.0,
    );
    let expected = Matrix4::new(
      20.0, 22.0, 50.0, 48.0,
      44.0, 54.0, 114.0, 108.0,
      40.0, 58.0, 110.0, 102.0,
      16.0, 26.0, 46.0, 42.0,
    );

    assert_eq!(a * b, expected);
  }

  #[wasm_bindgen_test]
  pub fn a_matrix_multiplied_by_a_tuple() {
    let a = Matrix4::new(
      1.0, 2.0, 3.0, 4.0,
      2.0, 4.0, 4.0, 2.0,
      8.0, 6.0, 4.0, 1.0,
      0.0, 0.0, 0.0, 1.0,
    );
    let b = Tuple::new(1.0, 2.0, 3.0, 1.0);

    assert_eq!(a * b, Tuple::new(18.0, 24.0, 33.0, 1.0));
  }

  #[wasm_bindgen_test]
  pub fn multiplying_a_matrix_by_the_identity_matrix() {
    let a = Matrix4::new(
      0.0, 1.0, 2.0, 4.0,
      1.0, 2.0, 4.0, 8.0,
      2.0, 4.0, 8.0, 16.0,
      4.0, 8.0, 16.0, 32.0,
    );
    let expected = Matrix4::new(
      0.0, 1.0, 2.0, 4.0,
      1.0, 2.0, 4.0, 8.0,
      2.0, 4.0, 8.0, 16.0,
      4.0, 8.0, 16.0, 32.0,
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
      0.0, 9.0, 3.0, 0.0,
      9.0, 8.0, 0.0, 8.0,
      1.0, 8.0, 5.0, 3.0,
      0.0, 0.0, 5.0, 8.0,
    );
    let expected = Matrix4::new(
      0.0, 9.0, 1.0, 0.0,
      9.0, 8.0, 8.0, 0.0,
      3.0, 0.0, 5.0, 5.0,
      0.0, 8.0, 3.0, 8.0,
    );

    assert_eq!(a.transpose(), expected);
  }

  #[wasm_bindgen_test]
  pub fn transposing_the_identity_matrix() {
    assert_eq!(Matrix4::identity().transpose(), Matrix4::identity());
  }

  #[wasm_bindgen_test]
  pub fn calculating_the_determinant_of_a_2x2_matrix() {
    let a = Matrix2::new(
      1.0, 5.0,
      -3.0, 2.0,
    );

    assert_eq!(a.determinant(), 17.0);
  }

  #[wasm_bindgen_test]
  pub fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
    let a = Matrix3::new(
      1.0, 5.0, 0.0,
      -3.0, 2.0, 7.0,
      0.0, 6.0, -3.0,
    );
    let expected = Matrix2::new(
      -3.0, 2.0,
      0.0, 6.0,
    );

    assert_eq!(a.submatrix(0, 2), expected);
  }

  #[wasm_bindgen_test]
  pub fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
    let a = Matrix4::new(
      -6.0, 1.0, 1.0, 6.0,
      -8.0, 5.0, 8.0, 6.0,
      -1.0, 0.0, 8.0, 2.0,
      -7.0, 1.0, -1.0, 1.0,
    );
    let expected = Matrix3::new(
      -6.0, 1.0, 6.0,
      -8.0, 8.0, 6.0,
      -7.0, -1.0, 1.0,
    );

    assert_eq!(a.submatrix(2, 1), expected);
  }

  #[wasm_bindgen_test]
  pub fn calculating_a_minor_of_a_3x3_matrix() {
    let a = Matrix3::new(
      3.0, 5.0, 0.0,
      2.0, -1.0, -7.0,
      6.0, -1.0, 5.0,
    );
    let b = a.submatrix(1, 0);

    assert_eq!(b.determinant(), 25.0);
    assert_eq!(a.minor(1, 0), 25.0);
  }

  #[wasm_bindgen_test]
  pub fn calculating_a_cofactor_of_a_3x3_matrix() {
    let a = Matrix3::new(
      3.0, 5.0, 0.0,
      2.0, -1.0, -7.0,
      6.0, -1.0, 5.0,
    );

    assert_eq!(a.minor(0, 0), -12.0);
    assert_eq!(a.cofactor(0, 0), -12.0);
    assert_eq!(a.minor(1, 0), 25.0);
    assert_eq!(a.cofactor(1, 0), -25.0);
  }

  #[wasm_bindgen_test]
  pub fn calculating_the_determinant_of_a_3x3_matrix() {
    let a = Matrix3::new(
      1.0, 2.0, 6.0,
      -5.0, 8.0, -4.0,
      2.0, 6.0, 4.0,
    );

    assert_eq!(a.cofactor(0, 0), 56.0);
    assert_eq!(a.cofactor(0, 1), 12.0);
    assert_eq!(a.cofactor(0, 2), -46.0);
    assert_eq!(a.determinant(), -196.0);
  }

  #[wasm_bindgen_test]
  pub fn calculating_the_determinant_of_a_4x4_matrix() {
    let a = Matrix4::new(
      -2.0, -8.0, 3.0, 5.0,
      -3.0, 1.0, 7.0, 3.0,
      1.0, 2.0, -9.0, 6.0,
      -6.0, 7.0, 7.0, -9.0,
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
      6.0, 4.0, 4.0, 4.0,
      5.0, 5.0, 7.0, 6.0,
      4.0, -9.0, 3.0, -7.0,
      9.0, 1.0, 7.0, -6.0,
    );

    assert_eq!(a.determinant(), -2120.0);
    assert!(a.is_invertible());
  }

  #[wasm_bindgen_test]
  pub fn testing_a_noninvertible_matrix_for_invertibility() {
    let a = Matrix4::new(
      -4.0, 2.0, -2.0, -3.0,
      9.0, 6.0, 2.0, 6.0,
      0.0, -5.0, 1.0, -5.0,
      0.0, 0.0, 0.0, 0.0,
    );

    assert_eq!(a.determinant(), 0.0);
    assert!(!a.is_invertible());
  }

  #[wasm_bindgen_test]
  pub fn calculating_the_inverse_of_a_matrix() {
    let a = Matrix4::new(
      -5.0, 2.0, 6.0, -8.0,
      1.0, -5.0, 1.0, 8.0,
      7.0, 7.0, -6.0, -7.0,
      1.0, -3.0, 7.0, 4.0,
    );
    let b = a.inverse().unwrap();
    let expected = Matrix4::new(
      0.21805, 0.45113, 0.24060, -0.04511,
      -0.80827, -1.45677, -0.44361, 0.52068,
      -0.07895, -0.22368, -0.05263, 0.19737,
      -0.52256, -0.81391, -0.30075, 0.30639,
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
      8.0, -5.0, 9.0, 2.0,
      7.0, 5.0, 6.0, 1.0,
      -6.0, 0.0, 9.0, 6.0,
      -3.0, 0.0, -9.0, -4.0,
    );
    let b = a.inverse().unwrap();
    let expected = Matrix4::new(
      -0.15385, -0.15385, -0.28205, -0.53846,
      -0.07692, 0.12308, 0.02564, 0.03077,
      0.35897, 0.35897, 0.43590, 0.92308,
      -0.69231, -0.69231, -0.76923, -1.92308,
    );

    assert_eq!(b, expected);
  }

  #[wasm_bindgen_test]
  pub fn calculating_the_inverse_of_a_third_matrix() {
    let a = Matrix4::new(
      9.0, 3.0, 0.0, 9.0,
      -5.0, -2.0, -6.0, -3.0,
      -4.0, 9.0, 6.0, 4.0,
      -7.0, 6.0, 6.0, 2.0,
    );
    let b = a.inverse().unwrap();
    let expected = Matrix4::new(
      -0.04074, -0.07778, 0.14444, -0.22222,
      -0.07778, 0.03333, 0.36667, -0.33333,
      -0.02901, -0.14630, -0.10926, 0.12963,
      0.17778, 0.06667, -0.26667, 0.33333,
    );

    assert_eq!(b, expected);
  }

  #[wasm_bindgen_test]
  pub fn multiplying_a_product_by_its_inverse() {
    let a = Matrix4::new(
      3.0, -9.0, 7.0, 3.0,
      3.0, -8.0, 2.0, -9.0,
      -4.0, 4.0, 4.0, 1.0,
      -6.0, 5.0, -1.0, 1.0,
    );
    let b = Matrix4::new(
      8.0, 2.0, 2.0, 2.0,
      3.0, -1.0, 7.0, 0.0,
      7.0, 0.0, 5.0, 4.0,
      6.0, -2.0, 0.0, 5.0,
    );
    let c = a * b;

    assert_eq!(c * b.inverse().unwrap(), a);
  }
}
