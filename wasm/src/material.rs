use crate::fuzzy::*;
use crate::light::*;
use crate::tuple::*;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub color: Tuple,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Tuple::color(1.0, 1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl PartialEq<Material> for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && fuzzy_eq_f32(self.ambient, other.ambient)
            && fuzzy_eq_f32(self.diffuse, other.diffuse)
            && fuzzy_eq_f32(self.specular, other.specular)
            && fuzzy_eq_f32(self.shininess, other.shininess)
    }
}

impl Material {
    pub fn lighting(
        &self,
        light: Light,
        point: Tuple,
        eyev: Tuple,
        normalv: Tuple,
    ) -> Tuple {
        // Combined color of this material and the light’s color/intensity
        let effective_color = self.color * light.intensity;

        // Direction to the light source
        let lightv = (light.position - point).normalize();

        // Ambient light contribution
        let ambient = effective_color * self.ambient;

        let cos_between_lightv_and_normalv = lightv.dot(normalv);
        let light_is_behind_surface = cos_between_lightv_and_normalv < 0.0;
        let (diffuse, specular) = if light_is_behind_surface {
            (
                Tuple::color(0.0, 0.0, 0.0, 1.0),
                Tuple::color(0.0, 0.0, 0.0, 1.0),
            )
        } else {
            // Diffuse contribution
            let diffuse = effective_color * self.diffuse * cos_between_lightv_and_normalv;
            let reflectv = (-lightv).reflect(normalv);
            let cos_between_reflectv_and_eyev = reflectv.dot(eyev);
            let light_reflects_away_from_eye = cos_between_lightv_and_normalv <= 0.0;

            let specular = if light_reflects_away_from_eye {
                Tuple::color(0.0, 0.0, 0.0, 1.0)
            } else {
                let factor = cos_between_reflectv_and_eyev.powf(self.shininess);
                light.intensity * self.specular * factor
            };

            (diffuse, specular)
        };

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    pub fn default_material() {
        let m = Material::default();
        assert_eq!(m.color, Tuple::color(1.0, 1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[wasm_bindgen_test]
    pub fn materials_are_equal() {
        let m1 = Material::default();
        let m2 = Material::default();
        assert_eq!(m1, m2);
    }

    #[wasm_bindgen_test]
    pub fn materials_are_not_equal() {
        let m1 = Material::default();
        let mut m2 = Material::default();
        m2.ambient = 0.2;
        assert_ne!(m1, m2);
    }

    #[wasm_bindgen_test]
    pub fn materials_with_fuzzy_equal_properties_are_equal() {
        let m1 = Material::default();
        let mut m2 = Material::default();
        m2.shininess = 200.0 + 1e-6;
        assert_eq!(m1, m2);
    }

    fn background() -> (Material, Tuple) {
        (Material::default(), Tuple::point(0.0, 0.0, 0.0))
    }

    #[wasm_bindgen_test]
    pub fn lighting_with_the_eye_between_light_and_surface() {
        let (m, position) = background();
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(
            Tuple::point(0.0, 0.0, -10.0),
            Tuple::color(1.0, 1.0, 1.0, 1.0),
        );
        let result = m.lighting(light, position, eyev, normalv);
        assert_eq!(result, Tuple::color(1.9, 1.9, 1.9, 1.9));
    }

    #[wasm_bindgen_test]
    pub fn lighting_with_eye_offset_45_degrees() {
        let (m, position) = background();
        let eyev = Tuple::vector(0.0, 2f32.sqrt() / 2.0, -2f32.sqrt() / 2.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(
            Tuple::point(0.0, 0.0, -10.0),
            Tuple::color(1.0, 1.0, 1.0, 1.0),
        );
        let result = m.lighting(light, position, eyev, normalv);
        assert_eq!(result, Tuple::color(1.0, 1.0, 1.0, 1.0));
    }

    #[wasm_bindgen_test]
    pub fn lighting_with_light_offset_45_degrees() {
        let (m, position) = background();
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(
            Tuple::point(0.0, 10.0, -10.0),
            Tuple::color(1.0, 1.0, 1.0, 1.0),
        );
        let result = m.lighting(light, position, eyev, normalv);
        assert_eq!(result, Tuple::color(0.7364, 0.7364, 0.7364, 0.7364));
    }

    #[wasm_bindgen_test]
    pub fn lighting_with_eye_in_path_of_reflection_vector() {
        let (m, position) = background();
        let eyev = Tuple::vector(0.0, -2f32.sqrt() / 2.0, -2f32.sqrt() / 2.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(
            Tuple::point(0.0, 10.0, -10.0),
            Tuple::color(1.0, 1.0, 1.0, 1.0),
        );
        let result = m.lighting(light, position, eyev, normalv);
        assert_eq!(result, Tuple::color(1.63638, 1.63638, 1.63638, 1.63638));
    }

    #[wasm_bindgen_test]
    pub fn lighting_with_light_behind_surface() {
        let (m, position) = background();
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(
            Tuple::point(0.0, 0.0, 10.0),
            Tuple::color(1.0, 1.0, 1.0, 1.0),
        );
        let result = m.lighting(light, position, eyev, normalv);
        assert_eq!(result, Tuple::color(0.1, 0.1, 0.1, 2.1));
    }
}
