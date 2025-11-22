use crate::tuple::*;

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Tuple,
}

impl Light {
    pub fn new(position: Tuple, intensity: Tuple) -> Light {
        Light {
            position: position,
            intensity: intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    pub fn a_point_light_has_a_position_and_intensity() {
        let intensity = Tuple::color(1.0, 1.0, 1.0, 1.0);
        let position = Tuple::point(0.0, 0.0, 0.0);
        let light = Light::new(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.position, position);
    }
}
