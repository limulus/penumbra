use wasm_bindgen::prelude::*;

pub mod bench;
pub mod canvas;
pub mod fuzzy;
pub mod intersection;
pub mod light;
pub mod material;
pub mod matrix;
pub mod ray;
pub mod sphere;
pub mod tuple;

mod demo;
pub use demo::phong_sphere::*;
pub use demo::sphere_shadow::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    Ok(())
}
