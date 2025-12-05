use crate::matrix::Matrix4;
use crate::tuple::Tuple;

#[derive(Clone, Debug)]
pub struct Transform {
    operations: Vec<Matrix4>,
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            operations: Vec::new(),
        }
    }

    pub fn translate(self, x: f32, y: f32, z: f32) -> Self {
        let mut operations = self.operations;
        operations.push(Matrix4::translation(x, y, z));
        Self { operations }
    }

    pub fn scale(self, x: f32, y: f32, z: f32) -> Self {
        let mut operations = self.operations;
        operations.push(Matrix4::scaling(x, y, z));
        Self { operations }
    }

    pub fn rotate_x(self, r: f32) -> Self {
        let mut operations = self.operations;
        operations.push(Matrix4::rotation_x(r));
        Self { operations }
    }

    pub fn rotate_y(self, r: f32) -> Self {
        let mut operations = self.operations;
        operations.push(Matrix4::rotation_y(r));
        Self { operations }
    }

    pub fn rotate_z(self, r: f32) -> Self {
        let mut operations = self.operations;
        operations.push(Matrix4::rotation_z(r));
        Self { operations }
    }

    pub fn shear(self, xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Self {
        let mut operations = self.operations;
        operations.push(Matrix4::shearing(xy, xz, yx, yz, zx, zy));
        Self { operations }
    }

    pub fn build(self) -> Matrix4 {
        if self.operations.is_empty() {
            Matrix4::identity()
        } else {
            self.operations
                .iter()
                .cloned()
                .rev()
                .reduce(|a, b| a * b)
                .expect("Transform should have at least one operation")
        }
    }
}

pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix4 {
    let forward = (to - from).normalize();
    let left = forward.cross(up.normalize());
    let true_up = left.cross(forward);
    let orientation =
        Matrix4::from_rows(left, true_up, -forward, Tuple::new(0.0, 0.0, 0.0, 1.0));
    orientation * Matrix4::translation(-from.x(), -from.y(), -from.z())
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    pub fn the_transformation_matrix_for_the_default_orientation() {
        let from = Tuple::point(0.0, 0.0, 0.0);
        let to = Tuple::point(0.0, 0.0, -1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        assert_eq!(t, Matrix4::identity());
    }

    #[wasm_bindgen_test]
    pub fn a_view_transformation_matrix_looking_in_positive_z_direction() {
        let from = Tuple::point(0.0, 0.0, 0.0);
        let to = Tuple::point(0.0, 0.0, 1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        assert_eq!(t, Matrix4::scaling(-1.0, 1.0, -1.0));
    }

    #[wasm_bindgen_test]
    pub fn the_view_transformation_moves_the_world() {
        let from = Tuple::point(0.0, 0.0, 8.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        assert_eq!(t, Matrix4::translation(0.0, 0.0, -8.0));
    }

    #[wasm_bindgen_test]
    pub fn an_arbitrary_view_transformation() {
        let from = Tuple::point(1.0, 3.0, 2.0);
        let to = Tuple::point(4.0, -2.0, 8.0);
        let up = Tuple::vector(1.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        let expected = Matrix4::new(
            -0.50709, 0.50709, 0.67612, -2.36643, 0.76772, 0.60609, 0.12122, -2.82843,
            -0.35857, 0.59761, -0.71714, 0.00000, 0.00000, 0.00000, 0.00000, 1.00000,
        );
        assert_eq!(t, expected);
    }

    #[wasm_bindgen_test]
    pub fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let t = Transform::new()
            .rotate_x(std::f32::consts::PI / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0)
            .build();
        assert_eq!(t * p, Tuple::point(15.0, 0.0, 7.0));
    }
}
