use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

use crate::intersection::*;
use crate::material::*;
use crate::matrix::*;
use crate::ray::*;
use crate::transform::*;
use crate::tuple::*;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    id: u64,
    pub material: Material,
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
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);

        Sphere {
            id: NEXT_ID.fetch_add(1, Relaxed),
            material: Material::default(),
            transform: Matrix4::identity(),
            transform_inverse: Matrix4::identity(),
        }
    }

    #[must_use]
    pub fn same_object(&self, other: Sphere) -> bool {
        self.id == other.id
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

    pub fn set_transform(&mut self, transform: Transform) -> Result<(), &'static str> {
        self.transform = transform.build();
        self.transform_inverse =
            self.transform.inverse().ok_or("Matrix is not invertible")?;
        Ok(())
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = self.transform_inverse * world_point;
        let object_normal = object_point - Tuple::point(0.0, 0.0, 0.0);
        let world_normal = Matrix4::transpose(&self.transform_inverse) * object_normal;
        world_normal.repair_vector().normalize()
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.material == other.material
            && self.transform == other.transform
            && self.transform_inverse == other.transform_inverse
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    pub fn two_equal_spheres_have_different_ids() {
        let s1 = Sphere::new();
        let s2 = Sphere::new();

        assert_eq!(s1, s2);
        assert_ne!(s1.id, s2.id);
    }

    #[wasm_bindgen_test]
    pub fn same_object_checks_id() {
        let s1 = Sphere::new();
        let s2 = Sphere::new();

        assert_eq!(s1.same_object(s1), true);
        assert_eq!(s1.same_object(s2), false);
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

        s.set_transform(Transform::new().translate(2.0, 3.0, 4.0))
            .unwrap();

        assert_eq!(s.transform, t);
    }

    #[wasm_bindgen_test]
    pub fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();

        s.set_transform(Transform::new().scale(2.0, 2.0, 2.0))
            .unwrap();
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[wasm_bindgen_test]
    pub fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();

        s.set_transform(Transform::new().translate(5.0, 0.0, 0.0))
            .unwrap();
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    #[wasm_bindgen_test]
    pub fn normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(1.0, 0.0, 0.0));
        assert_eq!(n, Tuple::vector(1.0, 0.0, 0.0));
    }

    #[wasm_bindgen_test]
    pub fn normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(0.0, 1.0, 0.0));
        assert_eq!(n, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[wasm_bindgen_test]
    pub fn normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(n, Tuple::vector(0.0, 0.0, 1.0));
    }

    #[wasm_bindgen_test]
    pub fn normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(
            3f32.sqrt() / 3.0,
            3f32.sqrt() / 3.0,
            3f32.sqrt() / 3.0,
        ));
        assert_eq!(
            n,
            Tuple::vector(3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0)
        );
    }

    #[wasm_bindgen_test]
    pub fn the_normal_is_a_normalized_vector() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(
            3f32.sqrt() / 3.0,
            3f32.sqrt() / 3.0,
            3f32.sqrt() / 3.0,
        ));
        assert_eq!(n, n.normalize());
    }

    #[wasm_bindgen_test]
    pub fn computing_the_normal_on_a_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transform(Transform::new().translate(0.0, 1.0, 0.0))
            .unwrap();
        let n = s.normal_at(Tuple::point(0.0, 1.70711, -0.70711));
        assert_eq!(n, Tuple::vector(0.0, 0.70711, -0.70711));
    }

    #[wasm_bindgen_test]
    pub fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = Sphere::new();
        s.set_transform(
            Transform::new()
                .rotate_z(std::f32::consts::PI / 5.0)
                .scale(1.0, 0.5, 1.0),
        )
        .unwrap();
        let n = s.normal_at(Tuple::point(0.0, 2f32.sqrt() / 2.0, -(2f32.sqrt()) / 2.0));
        assert_eq!(n, Tuple::vector(0.0, 0.97014, -0.24254));
    }

    #[wasm_bindgen_test]
    pub fn sphere_has_a_default_material() {
        let s = Sphere::new();
        let m = Material::default();
        assert_eq!(s.material, m);
    }

    #[wasm_bindgen_test]
    pub fn sphere_may_be_assigned_a_material() {
        let mut s = Sphere::new();
        let mut m = Material::default();
        m.ambient = 1.0;
        s.material = m;
        assert_eq!(s.material, m);
    }
}
