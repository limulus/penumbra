use crate::matrix::*;
use crate::tuple::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
  pub origin: Tuple,
  pub direction: Tuple,
}

impl Ray {
  pub fn new(origin: Tuple, direction: Tuple) -> Ray {
    Ray {
      origin,
      direction,
    }
  }

  pub fn position(&self, t: f32) -> Tuple {
    self.origin + self.direction * t
  }

  pub fn transform(&self, m: &Matrix4) -> Self {
    Ray::new(m * self.origin, m * self.direction)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use wasm_bindgen_test::*;

  #[wasm_bindgen_test]
  pub fn creating_and_querying_a_ray() {
    let origin = Tuple::point(1.0, 2.0, 3.0);
    let direction = Tuple::vector(4.0, 5.0, 6.0);
    let r = Ray::new(origin, direction);

    assert_eq!(r.origin, origin);
    assert_eq!(r.direction, direction);
  }

  #[wasm_bindgen_test]
  pub fn computing_a_point_from_a_distance() {
    let r = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0));

    assert_eq!(r.position(0.0), Tuple::point(2.0, 3.0, 4.0));
    assert_eq!(r.position(1.0), Tuple::point(3.0, 3.0, 4.0));
    assert_eq!(r.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
    assert_eq!(r.position(2.5), Tuple::point(4.5, 3.0, 4.0));
  }

  #[wasm_bindgen_test]
  pub fn translating_a_ray() {
    let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
    let m = Matrix4::translation(3.0, 4.0, 5.0);

    let r2 = r.transform(&m);

    assert_eq!(r2.origin, Tuple::point(4.0, 6.0, 8.0));
    assert_eq!(r2.direction, Tuple::vector(0.0, 1.0, 0.0));
  }

  #[wasm_bindgen_test]
  pub fn scaling_a_ray() {
    let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
    let m = Matrix4::scaling(2.0, 3.0, 4.0);

    let r2 = r.transform(&m);

    assert_eq!(r2.origin, Tuple::point(2.0, 6.0, 12.0));
    assert_eq!(r2.direction, Tuple::vector(0.0, 3.0, 0.0));
  }
}
