use wasm_bindgen::prelude::*;

pub mod fuzzy;
pub mod tuple;
pub mod canvas;
pub mod matrix;
pub mod ray;
pub mod sphere;
pub mod intersection;

mod demo;
pub use demo::sphere_shadow::*;

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
