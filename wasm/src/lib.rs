use wasm_bindgen::prelude::*;
use web_sys::ImageData;

pub mod fuzzy;
pub mod tuple;
pub mod canvas;
pub mod matrix;
pub mod ray;
pub mod sphere;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    Ok(())
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello from wasm-land!");
}

#[wasm_bindgen]
pub fn pretty_gradient(width: usize, height: usize) -> ImageData {
    let mut c = canvas::Canvas::new(width, height);

    let magic = tuple::Tuple::new(c.width as f32 * 1.3333, c.height as f32 * 1.3333, 1.0, 1.0);
    for y in 0..c.height {
        for x in 0..c.width {
            c.write_pixel(x, y, tuple::Tuple::new(x as f32, y as f32, 0.6666, 1.0) / magic);
        }
    }

    c.to_image_data()
}
