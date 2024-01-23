use crate::ray::*;
use crate::tuple::*;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {}

impl Sphere {
  pub fn new() -> Sphere {
    Sphere {}
  }

  pub fn intersect(&self, ray: &Ray) -> Vec<f32> {
    let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);

    // Determine the discriminant.
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * ray.direction.dot(sphere_to_ray);
    let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
      vec![]
    } else {
      vec![
        (-b - discriminant.sqrt()) / (2.0 * a),
        (-b + discriminant.sqrt()) / (2.0 * a),
      ]
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use wasm_bindgen_test::*;

  #[wasm_bindgen_test]
  pub fn a_ray_intersects_a_sphere_at_two_points() {
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();

    let xs = s.intersect(&r);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], 4.0);
    assert_eq!(xs[1], 6.0);
  }

  #[wasm_bindgen_test]
  pub fn a_ray_intersects_a_sphere_at_a_tangent() {
    let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();

    let xs = s.intersect(&r);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], 5.0);
    assert_eq!(xs[1], 5.0);
  }

  #[wasm_bindgen_test]
  pub fn a_ray_misses_a_sphere() {
    let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();

    let xs = s.intersect(&r);

    assert_eq!(xs.len(), 0);
  }

  #[wasm_bindgen_test]
  pub fn a_ray_originates_inside_a_sphere() {
    let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();

    let xs = s.intersect(&r);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], -1.0);
    assert_eq!(xs[1], 1.0);
  }

  #[wasm_bindgen_test]
  pub fn a_sphere_is_behind_a_ray() {
    let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();

    let xs = s.intersect(&r);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], -6.0);
    assert_eq!(xs[1], -4.0);
  }
}
