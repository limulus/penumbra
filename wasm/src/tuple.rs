use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::fuzzy::fuzzy_eq_f32x4;
use wide::f32x4;

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
use core::arch::wasm32;

#[derive(Clone, Copy, Debug)]
pub struct Tuple {
    data: f32x4,
}

impl Tuple {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple {
        Tuple {
            data: f32x4::from([x, y, z, w]),
        }
    }

    /// Create a Tuple from an f32x4 SIMD vector
    #[inline]
    pub fn from_f32x4(data: f32x4) -> Tuple {
        Tuple { data }
    }

    #[inline]
    pub fn point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple::new(x, y, z, 1.0)
    }

    #[inline]
    pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple::new(x, y, z, 0.0)
    }

    #[inline]
    pub fn color(red: f32, green: f32, blue: f32, alpha: f32) -> Tuple {
        Tuple::new(red, green, blue, alpha)
    }

    /// Convert this Tuple to an f32x4 SIMD vector
    #[inline]
    pub fn f32x4(&self) -> f32x4 {
        self.data
    }

    #[inline]
    pub fn is_point(self) -> bool {
        self.w() == 1.0
    }

    #[inline]
    pub fn is_vector(self) -> bool {
        self.w() == 0.0
    }

    #[inline]
    pub fn as_array(&self) -> &[f32; 4] {
        self.data.as_array()
    }

    #[inline]
    pub fn get(self, index: usize) -> f32 {
        self.data.as_array()[index]
    }

    #[inline]
    pub fn x(self) -> f32 {
        self.data.as_array()[0]
    }

    #[inline]
    pub fn y(self) -> f32 {
        self.data.as_array()[1]
    }

    #[inline]
    pub fn z(self) -> f32 {
        self.data.as_array()[2]
    }

    #[inline]
    pub fn w(self) -> f32 {
        self.data.as_array()[3]
    }

    #[inline]
    fn yzx(self) -> Tuple {
        // Swizzle: [x, y, z, w] -> [y, z, x, w]
        let values = self.data.as_array();
        Tuple {
            data: f32x4::from([values[1], values[2], values[0], values[3]]),
        }
    }

    #[inline]
    fn zxy(self) -> Tuple {
        // Swizzle: [x, y, z, w] -> [z, x, y, w]
        let values = self.data.as_array();
        Tuple {
            data: f32x4::from([values[2], values[0], values[1], values[3]]),
        }
    }

    #[inline]
    pub fn cross(self, other: Tuple) -> Tuple {
        self.yzx() * other.zxy() - self.zxy() * other.yzx()
    }

    #[inline]
    pub fn dot(self, other: Tuple) -> f32 {
        (self * other).sum()
    }

    #[inline]
    pub fn magnitude(self) -> f32 {
        (self * self).sum().sqrt()
    }

    #[inline]
    pub fn normalize(self) -> Tuple {
        self / self.magnitude()
    }

    fn sum(self) -> f32 {
        #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
        {
            self.data.reduce_add()
        }

        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        {
            let v: wasm32::v128 = unsafe { std::mem::transmute(self.data) };
            let sum1 = wasm32::f32x4_add(v, wasm32::i32x4_shuffle::<2, 3, 0, 1>(v, v));
            let sum2 =
                wasm32::f32x4_add(sum1, wasm32::i32x4_shuffle::<1, 0, 2, 3>(sum1, sum1));
            wasm32::f32x4_extract_lane::<0>(sum2)
        }
    }

    #[inline]
    pub fn to_rgba(self) -> Tuple {
        Tuple {
            data: (self.data * f32x4::splat(255.0)).round(),
        }
    }

    pub fn repair_vector_after_translation(self) -> Tuple {
        Tuple::vector(self.x(), self.y(), self.z())
    }

    pub fn reflect(self, normal: Tuple) -> Tuple {
        self - normal * 2.0 * self.dot(normal)
    }
}

impl Add<Tuple> for Tuple {
    type Output = Tuple;

    #[inline]
    fn add(self, other: Tuple) -> Tuple {
        Tuple {
            data: self.data + other.data,
        }
    }
}

impl Div<f32> for Tuple {
    type Output = Tuple;

    #[inline]
    fn div(self, other: f32) -> Tuple {
        Tuple {
            data: self.data / f32x4::splat(other),
        }
    }
}

impl Div<Tuple> for Tuple {
    type Output = Tuple;

    #[inline]
    fn div(self, other: Tuple) -> Tuple {
        Tuple {
            data: self.data / other.data,
        }
    }
}

impl Mul<f32> for Tuple {
    type Output = Tuple;

    #[inline]
    fn mul(self, other: f32) -> Tuple {
        Tuple {
            data: self.data * f32x4::splat(other),
        }
    }
}

impl Mul<Tuple> for Tuple {
    type Output = Tuple;

    #[inline]
    fn mul(self, other: Tuple) -> Tuple {
        Tuple {
            data: self.data * other.data,
        }
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    #[inline]
    fn neg(self) -> Self::Output {
        Tuple { data: -self.data }
    }
}

impl PartialEq<Tuple> for Tuple {
    #[inline]
    fn eq(&self, other: &Tuple) -> bool {
        fuzzy_eq_f32x4(self.data, other.data)
    }
}

impl Sub<Tuple> for Tuple {
    type Output = Tuple;

    #[inline]
    fn sub(self, other: Tuple) -> Tuple {
        Tuple::from_f32x4(self.data - other.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fuzzy::fuzzy_eq_f32;
    use crate::matrix::Matrix4;
    use wasm_bindgen_test::*;

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
    fn eq_when_difference_is_less_than_epsilon() {
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
        assert!(fuzzy_eq_f32(v.normalize().magnitude(), 1.0));
    }

    #[wasm_bindgen_test]
    fn dot_returns_the_dot_product_of_two_tuples() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert!(fuzzy_eq_f32(a.dot(b), 20.0));
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
        let c1 = Tuple::color(0.9, 0.6, 0.75, 0.2);
        let c2 = Tuple::color(0.7, 0.1, 0.25, 0.3);
        assert_eq!(c1 + c2, Tuple::color(1.6, 0.7, 1.0, 0.5));
    }

    #[wasm_bindgen_test]
    fn subtracting_colors() {
        let c1 = Tuple::color(0.9, 0.6, 0.75, 0.2);
        let c2 = Tuple::color(0.7, 0.1, 0.25, 0.2);
        assert_eq!(c1 - c2, Tuple::color(0.2, 0.5, 0.5, 0.0));
    }

    #[wasm_bindgen_test]
    fn multiplying_a_color_by_a_scalar() {
        let c = Tuple::color(0.2, 0.3, 0.4, 0.5);
        assert_eq!(c * 2.0, Tuple::color(0.4, 0.6, 0.8, 1.0));
    }

    #[wasm_bindgen_test]
    fn multiplying_colors() {
        let c1 = Tuple::color(1.0, 0.2, 0.4, 0.5);
        let c2 = Tuple::color(0.9, 1.0, 0.1, 0.2);
        assert_eq!(c1 * c2, Tuple::color(0.9, 0.2, 0.04, 0.1));
    }

    #[wasm_bindgen_test]
    fn repairing_a_translated_vector() {
        let v = Tuple::vector(-2.0, 1.0, 4.0);
        let m = Matrix4::translation(5.0, -3.0, 2.0).transpose();
        let v2 = m * v;
        assert_ne!(v2.w(), 0.0);
        let repaired = v2.repair_vector_after_translation();
        assert_eq!(repaired.w(), 0.0);
    }

    #[wasm_bindgen_test]
    fn reflecting_a_vector_approaching_at_45_degrees() {
        let v = Tuple::vector(1.0, -1.0, 0.0);
        let n = Tuple::vector(0.0, 1.0, 0.0);
        let r = v.reflect(n);
        assert_eq!(r, Tuple::vector(1.0, 1.0, 0.0));
    }

    #[wasm_bindgen_test]
    fn reflecting_a_vector_off_a_slanted_surface() {
        let v = Tuple::vector(0.0, -1.0, 0.0);
        let n = Tuple::vector(2f32.sqrt() / 2.0, 2f32.sqrt() / 2.0, 0.0);
        let r = v.reflect(n);
        print!("{:?}", v);
        assert_eq!(r, Tuple::vector(1.0, 0.0, 0.0));
    }
}
