use crate::intersection::*;
use crate::light::*;
use crate::material::*;
use crate::ray::*;
use crate::sphere::*;
use crate::transform::*;
use crate::tuple::*;

#[derive(Debug, Clone)]
pub struct World {
    pub objects: Vec<Sphere>,
    pub light: Option<Light>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
            light: None,
        }
    }

    pub fn intersect(&self, ray: Ray) -> IntersectionCollection<'_> {
        let mut intersections: Vec<Intersection> = Vec::new();
        for object in &self.objects {
            let mut object_intersections = object.intersect(&ray).xs;
            intersections.append(&mut object_intersections);
        }
        IntersectionCollection::new(intersections)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Sphere> {
        self.objects.iter()
    }

    pub fn shade_hit(&self, comps: &IntersectionComputations) -> Tuple {
        let material = &comps.object.material;
        material.lighting(
            self.light.expect("world must have a light to shade hit"),
            comps.point,
            comps.eyev,
            comps.normalv,
            self.is_shadowed(comps.over_point),
        )
    }

    pub fn color_at(&self, r: Ray) -> Tuple {
        let intersections = self.intersect(r);
        let hit = intersections.hit();
        match hit {
            Some(i) => {
                let comps = i.prepare_computations(r);
                self.shade_hit(&comps)
            }
            None => Tuple::color(0.0, 0.0, 0.0),
        }
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool {
        let light = self.light.expect("world must have a light to have shadows");
        let vector = light.position - point;
        let distance = vector.magnitude();
        let ray = Ray::new(point, vector.normalize());
        let intersections = self.intersect(ray);
        match intersections.hit() {
            None => false,
            Some(hit) => hit.t < distance,
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let light = Light::new(
            Tuple::point(-10.0, 10.0, -10.0),
            Tuple::color(1.0, 1.0, 1.0),
        );
        let mut s1 = Sphere::default();
        let material = Material {
            color: Tuple::color(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        };
        s1.material = material;
        let mut s2 = Sphere::new();
        s2.set_transform(Transform::new().scale(0.5, 0.5, 0.5))
            .unwrap();
        World {
            objects: vec![s1, s2],
            light: Some(light),
        }
    }
}

impl<'a> IntoIterator for &'a World {
    type Item = &'a Sphere;
    type IntoIter = std::slice::Iter<'a, Sphere>;

    fn into_iter(self) -> Self::IntoIter {
        self.objects.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn creating_a_world() {
        let w = World::new();
        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.light.is_none(), true);
    }

    #[wasm_bindgen_test]
    fn default_world() {
        let light = Light::new(
            Tuple::point(-10.0, 10.0, -10.0),
            Tuple::color(1.0, 1.0, 1.0),
        );
        let mut s1 = Sphere::default();
        let mut material = Material::default();
        material.color = Tuple::color(0.8, 1.0, 0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;
        s1.material = material;
        let mut s2 = Sphere::new();
        s2.set_transform(Transform::new().scale(0.5, 0.5, 0.5))
            .unwrap();
        let w = World::default();
        assert_eq!(w.light.unwrap(), light);
        assert!(w.iter().any(|obj| obj == &s1));
        assert!(w.iter().any(|obj| obj == &s2));
    }

    #[wasm_bindgen_test]
    fn intersect_world_with_ray() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let intersections = w.intersect(r);
        assert_eq!(intersections.len(), 4);
        assert_eq!(intersections[0].t, 4.0);
        assert_eq!(intersections[1].t, 4.5);
        assert_eq!(intersections[2].t, 5.5);
        assert_eq!(intersections[3].t, 6.0);
    }

    #[wasm_bindgen_test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let i = Intersection::new(4.0, shape);

        let comps = i.prepare_computations(r);
        let c = w.shade_hit(&comps);

        assert!(c.rgb_eq(Tuple::color(0.38066, 0.47583, 0.2855)));
    }

    #[wasm_bindgen_test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = World::default();
        w.light = Some(Light::new(
            Tuple::point(0.0, 0.25, 0.0),
            Tuple::color(1.0, 1.0, 1.0),
        ));
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, shape);

        let comps = i.prepare_computations(r);
        let c = w.shade_hit(&comps);
        assert!(c.rgb_eq(Tuple::color(0.90498, 0.90498, 0.90498)));
    }

    #[wasm_bindgen_test]
    fn the_color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
        let c = w.color_at(r);
        let black = Tuple::color(0.0, 0.0, 0.0);
        assert!(c.rgb_eq(black));
    }

    #[wasm_bindgen_test]
    fn the_color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let c = w.color_at(r);
        assert!(c.rgb_eq(Tuple::color(0.38066, 0.47583, 0.2855)));
    }

    #[wasm_bindgen_test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = World::default();

        let outer = &mut w.objects[0];
        outer.material.ambient = 1.0;

        let inner_material_color = {
            let inner = &mut w.objects[1];
            inner.material.ambient = 1.0;
            inner.material.color
        };

        let r = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));
        let c = w.color_at(r);
        assert!(c.rgb_eq(inner_material_color));
    }

    #[wasm_bindgen_test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();
        let p = Tuple::point(0.0, 10.0, 0.0);
        assert_eq!(w.is_shadowed(p), false);
    }

    #[wasm_bindgen_test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World::default();
        let p = Tuple::point(10.0, -10.0, 10.0);
        assert_eq!(w.is_shadowed(p), true);
    }

    #[wasm_bindgen_test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default();
        let p = Tuple::point(-20.0, 20.0, -20.0);
        assert_eq!(w.is_shadowed(p), false);
    }

    #[wasm_bindgen_test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = World::default();
        let p = Tuple::point(-2.0, 2.0, -2.0);
        assert_eq!(w.is_shadowed(p), false);
    }

    #[wasm_bindgen_test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = World::new();
        w.light = Some(Light::new(
            Tuple::point(0.0, 0.0, -10.0),
            Tuple::color(1.0, 1.0, 1.0),
        ));
        let s1 = Sphere::default();
        w.objects.push(s1);
        let mut s2 = Sphere::new();
        s2.set_transform(Transform::new().translate(0.0, 0.0, 10.0))
            .unwrap();
        w.objects.push(s2);
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, &s2);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(&comps);
        assert!(c.rgb_eq(Tuple::color(0.1, 0.1, 0.1)));
    }
}
