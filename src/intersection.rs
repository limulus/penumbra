use std::ops::Index;

use crate::ray::*;
use crate::sphere::*;
use crate::tuple::*;

const SHADOW_SURFACE_OFFSET: f32 = 0.005;

#[derive(Clone, Copy, Debug)]
pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, object: &'a Sphere) -> Intersection<'a> {
        Intersection { t, object }
    }

    pub fn prepare_computations(&self, r: Ray) -> IntersectionComputations {
        let eyev = -r.direction;
        let mut normalv = (*self.object).normal_at(r.position(self.t));

        let inside = normalv.dot(eyev) < 0.0;
        if inside {
            normalv = -normalv;
        }

        let point = r.position(self.t);
        let over_point = point + normalv * SHADOW_SURFACE_OFFSET;

        IntersectionComputations {
            t: self.t,
            object: *self.object,
            point,
            over_point,
            eyev,
            normalv,
            inside,
        }
    }
}

pub struct IntersectionComputations {
    pub t: f32,
    pub object: Sphere,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

#[derive(Debug)]
pub struct IntersectionCollection<'a> {
    pub xs: Vec<Intersection<'a>>,
}

impl<'a> IntersectionCollection<'a> {
    pub fn new(mut xs: Vec<Intersection<'a>>) -> IntersectionCollection<'a> {
        xs.sort_unstable_by(|a, b| {
            a.t.partial_cmp(&b.t)
                .expect("intersection t values must not be NaN")
        });
        IntersectionCollection { xs }
    }

    pub fn len(&self) -> usize {
        self.xs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.xs.is_empty()
    }

    pub fn hit(&self) -> Option<&Intersection<'a>> {
        self.xs.iter().find(|i| i.t >= 0.0)
    }
}

impl<'a> Index<usize> for IntersectionCollection<'a> {
    type Output = Intersection<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.xs[index]
    }
}

impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.object.same_object(*other.object)
            || (self.t == other.t && self.object == other.object)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transform::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    pub fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &s);
    }

    #[wasm_bindgen_test]
    pub fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);

        let xs = IntersectionCollection::new(vec![i1, i2]);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[wasm_bindgen_test]
    pub fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, &s);
        assert_eq!(xs[1].object, &s);
    }

    #[wasm_bindgen_test]
    pub fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = IntersectionCollection::new(vec![i1, i2]);

        let i = xs.hit();

        assert_eq!(i, Some(&i1));
    }

    #[wasm_bindgen_test]
    pub fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = IntersectionCollection::new(vec![i1, i2]);

        let i = xs.hit();

        assert_eq!(i, Some(&i2));
    }

    #[wasm_bindgen_test]
    pub fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = IntersectionCollection::new(vec![i1, i2]);

        let i = xs.hit();

        assert_eq!(i, None);
    }

    #[wasm_bindgen_test]
    pub fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = IntersectionCollection::new(vec![i1, i2, i3, i4]);

        let i = xs.hit();

        assert_eq!(i, Some(&i4));
    }

    #[wasm_bindgen_test]
    pub fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection::new(4.0, &shape);

        let comps = i.prepare_computations(r);

        assert_eq!(comps.t, i.t);
        assert!(comps.object.same_object(*i.object));
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[wasm_bindgen_test]
    pub fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection::new(4.0, &shape);

        let comps = i.prepare_computations(r);

        assert_eq!(comps.inside, false);
    }

    #[wasm_bindgen_test]
    pub fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection::new(1.0, &shape);

        let comps = i.prepare_computations(r);

        assert_eq!(comps.point, Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[wasm_bindgen_test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut shape = Sphere::default();
        shape
            .set_transform(Transform::new().translate(0.0, 0.0, 1.0))
            .unwrap();
        let i = Intersection::new(5.0, &shape);

        let comps = i.prepare_computations(r);

        assert!(comps.over_point.z() < SHADOW_SURFACE_OFFSET / 2.0);
        assert!(comps.point.z() > comps.over_point.z());
    }
}
