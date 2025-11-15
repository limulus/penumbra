use crate::intersection::*;
use crate::matrix::*;
use crate::ray::*;
use crate::tuple::*;

#[derive(Debug, Clone)]
pub struct Sphere {
    transform: Matrix4,
    transform_inverse: Matrix4,
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transform: Matrix4::identity(),
            transform_inverse: Matrix4::identity(),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> IntersectionCollection<'_> {
        let ray = ray.transform(&self.transform_inverse);
        let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);

        // Determine the discriminant.
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            IntersectionCollection::new(vec![])
        } else {
            IntersectionCollection::new(vec![
                Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), self),
                Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), self),
            ])
        }
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform.build();
        self.transform_inverse = self.transform.inverse().unwrap();
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    pub fn two_different_spheres_are_not_equal() {
        let s1 = Sphere::new();
        let s2 = Sphere::new();

        assert_ne!(s1, s2);
    }

    #[wasm_bindgen_test]
    pub fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[wasm_bindgen_test]
    pub fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
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
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[wasm_bindgen_test]
    pub fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[wasm_bindgen_test]
    pub fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[wasm_bindgen_test]
    pub fn a_spheres_default_transformation() {
        let s = Sphere::new();

        assert_eq!(s.transform, Matrix4::identity());
    }

    #[wasm_bindgen_test]
    pub fn changing_a_spheres_transformation() {
        let mut s = Sphere::new();
        let t = Matrix4::translation(2.0, 3.0, 4.0);

        s.set_transform(Transform::new().translate(2.0, 3.0, 4.0));

        assert_eq!(s.transform, t);
    }

    #[wasm_bindgen_test]
    pub fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();

        s.set_transform(Transform::new().scale(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[wasm_bindgen_test]
    pub fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();

        s.set_transform(Transform::new().translate(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 0);
    }
}
