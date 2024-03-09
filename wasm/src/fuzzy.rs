use std::arch::wasm32::*;
pub const EPSILON: f32 = 0.00001;

pub fn fuzzy_eq_f32(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

pub fn fuzzy_eq_f32x4(a: v128, b: v128) -> bool {
    let diff = f32x4_abs(f32x4_sub(a, b));
    let ge_epsilon = f32x4_ge(diff, f32x4_splat(EPSILON));
    !v128_any_true(ge_epsilon)
}
