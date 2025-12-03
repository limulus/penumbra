use wasm_bindgen::prelude::*;
use web_sys::ImageData;

use crate::canvas::*;
use crate::matrix::*;
use crate::ray::*;
use crate::sphere::*;
use crate::tuple::*;

#[wasm_bindgen]
pub struct SphereShadowRenderer {
    background: Canvas,
    canvas: Canvas,
    light_pos: Tuple,
    light_transform: Matrix4,
    sensor_points: Vec<Tuple>,
    sensor_size: f32,
    sphere: Sphere,
}

#[wasm_bindgen]
impl SphereShadowRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize, sensor_size: f32) -> SphereShadowRenderer {
        let sensor_transform: Matrix4 = Transform::new()
            .translate(-sensor_size / 2.0, -sensor_size / 2.0, 0.0) // Center the sensor
            .rotate_x(std::f32::consts::PI) // Rotate 180 degrees around the x-axis to account for the canvas being upside-down
            .rotate_y(std::f32::consts::PI / 2.0) // Rotate 90 degrees around the y-axis so it faces the light
            .build();

        let mut sphere = Sphere::new();
        sphere
            .set_transform(Transform::new().scale(2.0, 2.0, 2.0))
            .expect("Hardcoded sphere transform should always be invertible");

        SphereShadowRenderer {
            canvas: Canvas::new(width, height),
            background: precompute_background_gradient(width, height),
            light_pos: Tuple::point(-5.0, 0.0, 0.0),
            light_transform: Matrix4::identity(),
            sensor_points: precompute_sensor_points(
                sensor_size,
                sensor_transform,
                width,
                height,
            ),
            sensor_size,
            sphere,
        }
    }

    #[wasm_bindgen]
    pub fn translate_light_relative_to_canvas_pos(&mut self, x: f32, y: f32) {
        self.light_transform = Transform::new()
            .translate(0.0, y, x)
            .scale(
                1.0,
                self.sensor_size / self.canvas.height as f32,
                self.sensor_size / self.canvas.width as f32,
            )
            .translate(0.0, -self.sensor_size / 2.0, -self.sensor_size / 2.0)
            .scale(1.0, 1.5, 1.5)
            .rotate_z(std::f32::consts::PI)
            .rotate_y(std::f32::consts::PI)
            .build();
    }

    #[wasm_bindgen]
    pub fn render(&mut self) {
        let light_pos = self.light_transform * self.light_pos;
        for i in 0..self.canvas.width {
            for j in 0..self.canvas.height {
                let sensor_point = self.sensor_points[i * self.canvas.height + j];
                let ray = Ray::new(light_pos, (sensor_point - light_pos).normalize());
                let intersections = self.sphere.intersect(&ray);
                if !intersections.is_empty() {
                    self.canvas.write_pixel(i, j, Tuple::color(0.0, 0.0, 0.0));
                } else {
                    self.canvas
                        .write_pixel(i, j, self.background.pixel_at(i, j));
                }
            }
        }
    }

    #[wasm_bindgen]
    pub fn to_image_data(&self) -> ImageData {
        self.canvas.to_image_data()
    }
}

fn precompute_background_gradient(width: usize, height: usize) -> Canvas {
    let mut canvas = Canvas::new(width, height);
    for y in 0..height {
        let y_ratio = (height - y) as f32 / height as f32 / 1.3333;
        for x in 0..width {
            let x_ratio = (width - x) as f32 / width as f32 / 1.3333;
            canvas.write_pixel(x, y, Tuple::color(x_ratio, y_ratio, 0.6666));
        }
    }
    canvas
}

fn precompute_sensor_points(
    sensor_size: f32,
    sensor_transform: Matrix4,
    width: usize,
    height: usize,
) -> Vec<Tuple> {
    let mut sensor_points = Vec::new();
    for i in 0..width {
        for j in 0..height {
            let x = (i as f32 / width as f32) * sensor_size;
            let y = (j as f32 / height as f32) * sensor_size;
            let sensor_point = sensor_transform * Tuple::point(x, y, 0.0);
            sensor_points.push(sensor_point);
        }
    }
    sensor_points
}
