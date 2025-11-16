use wide::{f32x4, CmpGe};

pub const EPSILON: f32 = 0.00001;

pub fn fuzzy_eq_f32(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

pub fn fuzzy_eq_f32x4(a: f32x4, b: f32x4) -> bool {
    let diff = (a - b).abs();
    let epsilon = f32x4::splat(EPSILON);
    let ge_epsilon = diff.cmp_ge(epsilon);
    !ge_epsilon.any()
}
