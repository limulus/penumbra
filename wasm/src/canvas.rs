use web_sys::ImageData;

use crate::tuple::Tuple;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Tuple>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let pixels = vec![Tuple::color(0.0, 0.0, 0.0, 1.0); width * height];
        Canvas { width, height, pixels }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Tuple) {
        self.pixels[y * self.width + x] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Tuple {
        self.pixels[y * self.width + x]
    }

    pub fn to_image_data(&self) -> ImageData {
        let mut data: Vec<u8> = Vec::with_capacity(self.width * self.height * 4);
        for pixel in &self.pixels {
            let rgba = pixel.to_rgba();
            data.extend_from_slice(&[rgba.x() as u8, rgba.y() as u8, rgba.z() as u8, rgba.w() as u8]);
        }
        ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(&mut data),
            self.width as u32,
            self.height as u32,
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn create_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for pixel in c.pixels {
            assert_eq!(pixel, Tuple::color(0.0, 0.0, 0.0, 1.0));
        }
    }

    #[wasm_bindgen_test]
    fn write_pixel_to_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Tuple::color(1.0, 0.0, 0.0, 1.0);
        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }
}
