use crate::tuple::*;

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Tuple,
}

impl Light {
    pub fn new(position: Tuple, intensity: Tuple) -> Light {
        Light {
            position,
            intensity,
        }
    }
}

impl PartialEq<Light> for Light {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.intensity.rgb_eq(other.intensity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    pub fn a_point_light_has_a_position_and_intensity() {
        let intensity = Tuple::color(1.0, 1.0, 1.0);
        let position = Tuple::point(0.0, 0.0, 0.0);
        let light = Light::new(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.position, position);
    }

    #[wasm_bindgen_test]
    pub fn lights_are_equal_if_they_have_the_same_position_and_intensity() {
        let intensity = Tuple::color(1.0, 1.0, 1.0);
        let position = Tuple::point(0.0, 0.0, 0.0);
        let light1 = Light::new(position, intensity);
        let light2 = Light::new(position, intensity);
        assert_eq!(light1, light2);
    }
}
