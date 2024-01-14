use std::arch::wasm32::*;
use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::fuzzy::EPSILON;

#[derive(Clone, Copy, Debug)]
pub struct Tuple(v128);

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple {
        Tuple(f32x4(x, y, z, w))
    }

    pub fn point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple::new(x, y, z, 1.0)
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple::new(x, y, z, 0.0)
    }

    pub fn color(red: f32, green: f32, blue: f32) -> Tuple {
        Tuple::new(red, green, blue, 0.0)
    }

    pub fn is_point(self) -> bool {
        self.w() == 1.0
    }

    pub fn is_vector(self) -> bool {
        self.w() == 0.0
    }

    pub fn x(self) -> f32 {
        f32x4_extract_lane::<0>(self.0)
    }

    pub fn y(self) -> f32 {
        f32x4_extract_lane::<1>(self.0)
    }

    pub fn z(self) -> f32 {
        f32x4_extract_lane::<2>(self.0)
    }

    pub fn w(self) -> f32 {
        f32x4_extract_lane::<3>(self.0)
    }

    fn yzx(self) -> Tuple {
        Tuple(u8x16_swizzle(
            self.0,
            u8x16(
                4, 5, 6, 7,
                8, 9, 10, 11,
                0, 1, 2, 3,
                12, 13, 14, 15
            )
        ))
    }

    fn zxy(self) -> Tuple {
        Tuple(u8x16_swizzle(
            self.0,
            u8x16(
                8, 9, 10, 11,
                0, 1, 2, 3,
                4, 5, 6, 7,
                12, 13, 14, 15
            )
        ))
    }

    pub fn cross(self, other: Tuple) -> Tuple {
        self.yzx() * other.zxy() - self.zxy() * other.yzx()
    }

    pub fn dot(self, other: Tuple) -> f32 {
        (self * other).sum()
    }

    pub fn magnitude(self) -> f32 {
        (self * self).sum().sqrt()
    }

    pub fn normalize(self) -> Tuple {
        self / self.magnitude()
    }

    fn sum(self) -> f32 {
        self.x() + self.y() + self.z() + self.w()
    }
}

impl Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        Tuple(f32x4_add(self.0, other.0))
    }
}

impl Div<f32> for Tuple {
    type Output = Tuple;

    fn div(self, other: f32) -> Tuple {
        Tuple(f32x4_div(self.0, f32x4_splat(other)))
    }
}

impl Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, other: f32) -> Tuple {
        Tuple(f32x4_mul(self.0, f32x4_splat(other)))
    }
}

impl Mul<Tuple> for Tuple {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        Tuple(f32x4_mul(self.0, other.0))
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple(f32x4_neg(self.0))
    }
}

impl PartialEq<Tuple> for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        let diff = f32x4_abs(f32x4_sub(self.0, other.0));
        let ge_epsilon = f32x4_ge(diff, f32x4_splat(EPSILON));
        !v128_any_true(ge_epsilon)
    }
}

impl Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple(f32x4_sub(self.0, other.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use crate::fuzzy::fuzzy_eq;

    #[wasm_bindgen_test]
    fn getters() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x(), 4.3);
        assert_eq!(a.y(), -4.2);
        assert_eq!(a.z(), 3.1);
        assert_eq!(a.w(), 1.0);
    }

    #[wasm_bindgen_test]
    fn is_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[wasm_bindgen_test]
    fn is_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[wasm_bindgen_test]
    fn point_creates_tuple_with_w_1() {
        let a = Tuple::point(4.0, -4.0, 3.0);
        assert_eq!(a, Tuple::new(4.0, -4.0, 3.0, 1.0));
    }

    #[wasm_bindgen_test]
    fn vector_creates_tuple_with_w_0() {
        let a = Tuple::vector(4.0, -4.0, 3.0);
        assert_eq!(a, Tuple::new(4.0, -4.0, 3.0, 0.0));
    }

    #[wasm_bindgen_test]
    fn eq_with_all_values_the_same() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        let b = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a, b);
    }

    #[wasm_bindgen_test]
    fn eq_with_one_value_differing() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        let b = Tuple::new(4.3, -4.2, 10.1, 1.0);
        assert_ne!(a, b);
    }

    #[wasm_bindgen_test]
    fn eq_with_all_values_differing() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        let b = Tuple::new(10.3, -10.2, 10.1, 0.0);
        assert_ne!(a, b);
    }

    #[wasm_bindgen_test]
    fn eq_when_difference_is_less_than_epsilon(){
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        let b = Tuple::new(4.300009, -4.200009, 3.100009, 1.0);
        assert_eq!(a, b);
    }

    #[wasm_bindgen_test]
    fn eq_when_difference_is_greater_than_epsilon() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        let b = Tuple::new(4.30001, -4.20001, 3.10001, 1.0);
        assert_ne!(a, b);
    }

    #[wasm_bindgen_test]
    fn add_returns_the_sum_of_two_tuples() {
        let a = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let b = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(a + b, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }

    #[wasm_bindgen_test]
    fn sub_returns_the_difference_of_two_points_as_vec() {
        let a = Tuple::point(3.0, 2.0, 1.0);
        let b = Tuple::point(5.0, 6.0, 7.0);
        assert_eq!(a - b, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[wasm_bindgen_test]
    fn sub_returns_the_difference_of_a_vector_and_a_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);
        assert_eq!(p - v, Tuple::point(-2.0, -4.0, -6.0));
    }

    #[wasm_bindgen_test]
    fn sub_returns_the_difference_of_two_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[wasm_bindgen_test]
    fn sub_returns_the_difference_of_a_vector_and_the_zero_vector() {
        let v1 = Tuple::vector(1.0, -2.0, 3.0);
        let v2 = Tuple::vector(0.0, 0.0, 0.0);
        assert_eq!(v1 - v2, Tuple::vector(1.0, -2.0, 3.0));
    }

    #[wasm_bindgen_test]
    fn neg_returns_the_negation_of_a_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[wasm_bindgen_test]
    fn mul_by_scalar_returns_the_product_of_a_tuple_and_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[wasm_bindgen_test]
    fn mul_by_fraction_returns_the_product_of_a_tuple_and_a_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[wasm_bindgen_test]
    fn div_by_scalar_returns_the_quotient_of_a_tuple_and_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[wasm_bindgen_test]
    fn magnitude_returns_the_magnitude_of_a_vector() {
        let v = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
        let v = Tuple::vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
        let v = Tuple::vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), 14.0_f32.sqrt());
        let v = Tuple::vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), 14.0_f32.sqrt());
    }

    #[wasm_bindgen_test]
    fn normalize_returns_a_vector_with_magnitude_1() {
        let v = Tuple::vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), Tuple::vector(1.0, 0.0, 0.0));
        let v = Tuple::vector(1.0, 2.0, 3.0);
        // Tuple::vector(1 / √14, 2 / √14, 3 / √14)
        assert_eq!(v.normalize(), Tuple::vector(0.26726, 0.53452, 0.80178));
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert!(fuzzy_eq(v.normalize().magnitude(), 1.0));
    }

    #[wasm_bindgen_test]
    fn dot_returns_the_dot_product_of_two_tuples() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert!(fuzzy_eq(a.dot(b), 20.0));
    }

    #[wasm_bindgen_test]
    fn cross_returns_the_cross_product_of_two_vectors() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(a.cross(b), Tuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(a), Tuple::vector(1.0, -2.0, 1.0));
    }

    #[wasm_bindgen_test]
    fn adding_colors() {
        let c1 = Tuple::color(0.9, 0.6, 0.75);
        let c2 = Tuple::color(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Tuple::color(1.6, 0.7, 1.0));
    }

    #[wasm_bindgen_test]
    fn subtracting_colors() {
        let c1 = Tuple::color(0.9, 0.6, 0.75);
        let c2 = Tuple::color(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, Tuple::color(0.2, 0.5, 0.5));
    }

    #[wasm_bindgen_test]
    fn multiplying_a_color_by_a_scalar() {
        let c = Tuple::color(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, Tuple::color(0.4, 0.6, 0.8));
    }

    #[wasm_bindgen_test]
    fn multiplying_colors() {
        let c1 = Tuple::color(1.0, 0.2, 0.4);
        let c2 = Tuple::color(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, Tuple::color(0.9, 0.2, 0.04));
    }
}
