use crate::canvas::*;
use crate::matrix::*;
use crate::ray::*;
use crate::tuple::*;
use crate::world::*;

pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f32,
    transform: Matrix4,
    transform_inverse: Matrix4,
    origin: Tuple,
    pixel_size: f32,
    half_width: f32,
    half_height: f32,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Camera {
        let aspect_ratio = hsize as f32 / vsize as f32;
        let half_view = (field_of_view / 2.0).tan();
        let (half_width, half_height) = if aspect_ratio >= 1.0 {
            (half_view, half_view / aspect_ratio)
        } else {
            (half_view * aspect_ratio, half_view)
        };
        let pixel_size = half_width * 2.0 / hsize as f32;

        let transform = Matrix4::identity();
        let transform_inverse = transform
            .inverse()
            .expect("transform of identity matrix should not panic");
        let origin = transform_inverse * Tuple::point(0.0, 0.0, 0.0);

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform,
            transform_inverse,
            origin,
            half_width,
            half_height,
            pixel_size,
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.hsize, self.vsize)
    }

    pub fn field_of_view(&self) -> f32 {
        self.field_of_view
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        // Offset from the edge of the canvas to the pixel’s center
        let pixel_edge = Tuple::vector(x as f32, y as f32, 0.0);
        let half_pixel_offset: Tuple = Tuple::vector(0.5, 0.5, 0.0);
        let offset = (pixel_edge + half_pixel_offset) * self.pixel_size;

        // Untransformed coords of the pixel in world space (the pixel is on the canvas at z
        // = -1)
        let world_coords = Tuple::point(self.half_width, self.half_height, -1.0) - offset;

        // Location of the pixel in world space
        let pixel = self.transform_inverse * world_coords;
        let direction = (pixel - self.origin).normalize();
        Ray::new(self.origin, direction)
    }

    pub fn set_transform(&mut self, transform: Matrix4) -> Result<(), &'static str> {
        self.transform = transform;
        self.transform_inverse = transform
            .inverse()
            .ok_or("transform matrix must be invertible")?;
        self.origin = self.transform_inverse * Tuple::point(0.0, 0.0, 0.0);
        Ok(())
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for x in 0..self.hsize {
            for y in 0..self.vsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray);
                image.write_pixel(x, y, color);
            }
        }

        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fuzzy::*;
    use crate::transform::{view_transform, Transform};
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = std::f32::consts::FRAC_PI_2;
        let c = Camera::new(hsize, vsize, field_of_view);
        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, std::f32::consts::FRAC_PI_2);
        assert!(c.transform == Matrix4::identity());
    }

    #[wasm_bindgen_test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, std::f32::consts::FRAC_PI_2);
        assert!(fuzzy_eq_f32(c.pixel_size, 0.01));
    }

    #[wasm_bindgen_test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, std::f32::consts::FRAC_PI_2);
        assert!(fuzzy_eq_f32(c.pixel_size, 0.01));
    }

    #[wasm_bindgen_test]
    fn ray_through_center_of_canvas() {
        let c = Camera::new(201, 101, std::f32::consts::FRAC_PI_2);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[wasm_bindgen_test]
    fn ray_through_corner_of_canvas() {
        let c = Camera::new(201, 101, std::f32::consts::FRAC_PI_2);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::vector(0.66519, 0.33259, -0.66851));
    }

    #[wasm_bindgen_test]
    fn ray_when_camera_is_transformed() {
        let mut c = Camera::new(201, 101, std::f32::consts::FRAC_PI_2);
        c.set_transform(
            Transform::new()
                .translate(0.0, -2.0, 5.0)
                .rotate_y(std::f32::consts::FRAC_PI_4)
                .build(),
        )
        .unwrap();
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, Tuple::point(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            Tuple::vector(2f32.sqrt() / 2.0, 0.0, -2f32.sqrt() / 2.0)
        );
    }

    #[wasm_bindgen_test]
    fn rendering_a_world_with_a_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, std::f32::consts::FRAC_PI_2);
        c.set_transform(view_transform(
            Tuple::point(0.0, 0.0, -5.0),
            Tuple::point(0.0, 0.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
        ))
        .unwrap();
        let image = c.render(&w);
        let expected_color = Tuple::color(0.38066, 0.47583, 0.2855);
        let pixel_color = image.pixel_at(5, 5);
        assert!(pixel_color.rgb_eq(expected_color));
    }
}
