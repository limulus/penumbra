use wasm_bindgen::prelude::*;

pub mod bench;
pub mod camera;
pub mod canvas;
pub mod fuzzy;
pub mod intersection;
pub mod light;
pub mod material;
pub mod matrix;
pub mod ray;
pub mod scene;
pub mod sphere;
pub mod transform;
pub mod tuple;
pub mod world;

mod demo;
pub use demo::phong_sphere::*;
pub use demo::sphere_shadow::*;
pub use scene::*;
pub use transform::Transform;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    Ok(())
}
