use std::arch::wasm32::*;
use std::cmp::PartialEq;
use std::ops::Mul;

use crate::fuzzy::fuzzy_eq_f32x4;
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
}
