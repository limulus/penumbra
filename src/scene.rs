use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use web_sys::ImageData;

use crate::camera::Camera;
use crate::light::Light;
use crate::sphere::Sphere;
use crate::transform::{view_transform, Transform};
use crate::tuple::Tuple;
use crate::world::World;

/// A high-level wrapper for ray tracing scenes, designed for JavaScript interop.
///
/// Scene manages a World (containing spheres and light) and a Camera, providing
/// a simplified API for building and rendering scenes from JavaScript.
#[wasm_bindgen]
pub struct Scene {
    world: World,
    camera: Camera,
    /// Maps sphere ID to index in world.objects for O(1) lookup
    sphere_index: HashMap<u64, usize>,
}

#[wasm_bindgen]
impl Scene {
    /// Creates a new Scene with the specified canvas dimensions and field of view.
    ///
    /// # Arguments
    /// * `width` - Canvas width in pixels
    /// * `height` - Canvas height in pixels
    /// * `fov` - Field of view in radians (defaults to π/3 ≈ 60°)
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize, fov: Option<f32>) -> Scene {
        let fov = fov.unwrap_or(std::f32::consts::FRAC_PI_3);
        Scene {
            world: World::new(),
            camera: Camera::new(width, height, fov),
            sphere_index: HashMap::new(),
        }
    }

    // =========================================================================
    // Sphere Management
    // =========================================================================

    /// Adds a new sphere to the scene and returns its ID.
    ///
    /// The sphere is created at the origin with default material (white, ambient=0.1,
    /// diffuse=0.9, specular=0.9, shininess=200).
    #[wasm_bindgen(js_name = addSphere)]
    pub fn add_sphere(&mut self) -> u64 {
        let sphere = Sphere::new();
        let id = sphere.id();
        let index = self.world.objects.len();
        self.world.objects.push(sphere);
        self.sphere_index.insert(id, index);
        id
    }

    /// Removes a sphere from the scene by ID.
    ///
    /// Returns true if the sphere was found and removed, false otherwise.
    #[wasm_bindgen(js_name = removeSphere)]
    pub fn remove_sphere(&mut self, id: u64) -> bool {
        if let Some(&index) = self.sphere_index.get(&id) {
            self.world.objects.remove(index);
            self.sphere_index.remove(&id);

            // Rebuild index map for all spheres after the removed one
            self.rebuild_sphere_index();
            true
        } else {
            false
        }
    }

    /// Returns the number of spheres in the scene.
    #[wasm_bindgen(js_name = getSphereCount)]
    pub fn get_sphere_count(&self) -> usize {
        self.world.objects.len()
    }

    // =========================================================================
    // Sphere Modification
    // =========================================================================

    /// Sets the transform for a sphere.
    ///
    /// The transform is applied to position, rotate, and scale the sphere in world space.
    #[wasm_bindgen(js_name = setSphereTransform)]
    pub fn set_sphere_transform(
        &mut self,
        id: u64,
        transform: Transform,
    ) -> Result<(), JsValue> {
        let sphere = self.get_sphere_mut(id)?;
        sphere
            .set_transform(transform)
            .map_err(|e| JsValue::from_str(e))
    }

    /// Sets the color of a sphere's material.
    #[wasm_bindgen(js_name = setSphereColor)]
    pub fn set_sphere_color(
        &mut self,
        id: u64,
        r: f32,
        g: f32,
        b: f32,
    ) -> Result<(), JsValue> {
        let sphere = self.get_sphere_mut(id)?;
        sphere.material.color = Tuple::color(r, g, b);
        Ok(())
    }

    /// Sets all material properties of a sphere.
    ///
    /// # Arguments
    /// * `ambient` - Ambient light reflection (0.0 - 1.0)
    /// * `diffuse` - Diffuse light reflection (0.0 - 1.0)
    /// * `specular` - Specular highlight reflection (0.0 - 1.0)
    /// * `shininess` - Specular highlight size (typically 10.0 - 200.0)
    #[wasm_bindgen(js_name = setSphereMaterial)]
    pub fn set_sphere_material(
        &mut self,
        id: u64,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
    ) -> Result<(), JsValue> {
        let sphere = self.get_sphere_mut(id)?;
        sphere.material.ambient = ambient;
        sphere.material.diffuse = diffuse;
        sphere.material.specular = specular;
        sphere.material.shininess = shininess;
        Ok(())
    }

    // =========================================================================
    // Light Management
    // =========================================================================

    /// Sets the scene's point light source.
    ///
    /// # Arguments
    /// * `x`, `y`, `z` - Light position in world space
    /// * `r`, `g`, `b` - Light intensity/color (typically 0.0 - 1.0 for each component)
    #[wasm_bindgen(js_name = setLight)]
    pub fn set_light(&mut self, x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) {
        self.world.light = Some(Light::new(Tuple::point(x, y, z), Tuple::color(r, g, b)));
    }

    // =========================================================================
    // Camera Management
    // =========================================================================

    /// Positions and orients the camera using "look at" semantics.
    ///
    /// # Arguments
    /// * `from_x`, `from_y`, `from_z` - Camera position in world space
    /// * `to_x`, `to_y`, `to_z` - Point the camera is looking at
    /// * `up_x`, `up_y`, `up_z` - Up direction vector (typically 0, 1, 0)
    #[wasm_bindgen(js_name = lookAt)]
    pub fn look_at(
        &mut self,
        from_x: f32,
        from_y: f32,
        from_z: f32,
        to_x: f32,
        to_y: f32,
        to_z: f32,
        up_x: f32,
        up_y: f32,
        up_z: f32,
    ) -> Result<(), JsValue> {
        let transform = view_transform(
            Tuple::point(from_x, from_y, from_z),
            Tuple::point(to_x, to_y, to_z),
            Tuple::vector(up_x, up_y, up_z),
        );
        self.camera
            .set_transform(transform)
            .map_err(|e| JsValue::from_str(e))
    }

    // =========================================================================
    // Rendering
    // =========================================================================

    /// Renders the scene and returns an ImageData object for use with HTML Canvas.
    ///
    /// The returned ImageData can be drawn to a canvas context using `putImageData()`.
    pub fn render(&self) -> ImageData {
        let canvas = self.camera.render(&self.world);
        canvas.to_image_data()
    }

    // =========================================================================
    // Private Helpers
    // =========================================================================

    /// Gets a mutable reference to a sphere by ID, or returns an error.
    fn get_sphere_mut(&mut self, id: u64) -> Result<&mut Sphere, JsValue> {
        let index = self.sphere_index.get(&id).ok_or_else(|| {
            JsValue::from_str(&format!("Sphere with ID {} not found", id))
        })?;
        self.world
            .objects
            .get_mut(*index)
            .ok_or_else(|| JsValue::from_str("Internal error: sphere index out of bounds"))
    }

    /// Rebuilds the sphere_index map after a removal operation.
    fn rebuild_sphere_index(&mut self) {
        self.sphere_index.clear();
        for (index, sphere) in self.world.objects.iter().enumerate() {
            self.sphere_index.insert(sphere.id(), index);
        }
    }
}

// =============================================================================
// Buffer-Based Rendering (for SharedArrayBuffer integration)
// =============================================================================

// Buffer layout constants (dimensions passed as parameters, not in buffer)
const HEADER_SIZE: usize = 128;
const SPHERE_SIZE: usize = 128;

const OFF_SPHERE_COUNT: usize = 0;
const OFF_CAMERA_FROM: usize = 4;
const OFF_CAMERA_TO: usize = 16;
const OFF_CAMERA_UP: usize = 28;
const OFF_LIGHT_POS: usize = 40;
const OFF_LIGHT_COLOR: usize = 52;

/// Renders a scene from a SharedArrayBuffer containing scene data.
///
/// Dimensions are passed as parameters to allow the same buffer to be rendered
/// at different resolutions (e.g., preview vs full-res) without race conditions.
#[wasm_bindgen(js_name = renderFromBuffer)]
pub fn render_from_buffer(buffer: &[u8], width: u32, height: u32, fov: f32) -> ImageData {
    let width = width as usize;
    let height = height as usize;

    let camera = parse_camera(buffer, width, height, fov);
    let world = parse_world(buffer);

    let canvas = camera.render(&world);
    canvas.to_image_data()
}

fn parse_camera(buffer: &[u8], width: usize, height: usize, fov: f32) -> Camera {
    let from = read_point(buffer, OFF_CAMERA_FROM);
    let to = read_point(buffer, OFF_CAMERA_TO);
    let up = read_vector(buffer, OFF_CAMERA_UP);

    let transform = view_transform(from, to, up);
    let mut camera = Camera::new(width, height, fov);
    camera
        .set_transform(transform)
        .expect("View transform should be valid");
    camera
}

fn parse_world(buffer: &[u8]) -> World {
    let mut world = World::new();

    // Parse light
    let light_pos = read_point(buffer, OFF_LIGHT_POS);
    let light_color = read_color(buffer, OFF_LIGHT_COLOR);
    world.light = Some(Light::new(light_pos, light_color));

    // Parse spheres
    let sphere_count = read_u32(buffer, OFF_SPHERE_COUNT) as usize;
    for i in 0..sphere_count {
        let offset = HEADER_SIZE + i * SPHERE_SIZE;
        let sphere = parse_sphere(buffer, offset);
        world.objects.push(sphere);
    }

    world
}

fn parse_sphere(buffer: &[u8], offset: usize) -> Sphere {
    use crate::material::Material;

    let transform_matrix = read_matrix(buffer, offset);
    let color = read_color(buffer, offset + 64);
    let ambient = read_f32(buffer, offset + 76);
    let diffuse = read_f32(buffer, offset + 80);
    let specular = read_f32(buffer, offset + 84);
    let shininess = read_f32(buffer, offset + 88);

    let mut sphere = Sphere::new();
    sphere
        .set_transform(Transform::from_matrix(transform_matrix))
        .expect("Transform should be valid");
    sphere.material = Material {
        color,
        ambient,
        diffuse,
        specular,
        shininess,
    };

    sphere
}

// Buffer reading helper functions
fn read_u32(buffer: &[u8], offset: usize) -> u32 {
    let bytes: [u8; 4] = buffer[offset..offset + 4].try_into().unwrap();
    u32::from_le_bytes(bytes)
}

fn read_f32(buffer: &[u8], offset: usize) -> f32 {
    let bytes: [u8; 4] = buffer[offset..offset + 4].try_into().unwrap();
    f32::from_le_bytes(bytes)
}

fn read_point(buffer: &[u8], offset: usize) -> Tuple {
    Tuple::point(
        read_f32(buffer, offset),
        read_f32(buffer, offset + 4),
        read_f32(buffer, offset + 8),
    )
}

fn read_vector(buffer: &[u8], offset: usize) -> Tuple {
    Tuple::vector(
        read_f32(buffer, offset),
        read_f32(buffer, offset + 4),
        read_f32(buffer, offset + 8),
    )
}

fn read_color(buffer: &[u8], offset: usize) -> Tuple {
    Tuple::color(
        read_f32(buffer, offset),
        read_f32(buffer, offset + 4),
        read_f32(buffer, offset + 8),
    )
}

fn read_matrix(buffer: &[u8], offset: usize) -> crate::matrix::Matrix4 {
    use crate::matrix::Matrix4;

    let mut values = [0.0f32; 16];
    for i in 0..16 {
        values[i] = read_f32(buffer, offset + i * 4);
    }
    Matrix4::from_array(values)
}

/// Builds a transform and writes the resulting matrix to a buffer.
///
/// This allows TypeScript to use the Rust Transform builder API while
/// writing the result directly to SharedArrayBuffer without extra allocations.
///
/// The matrix is written in column-major order as 16 f32 values (64 bytes total).
///
/// # Arguments
/// * `transform` - The Transform to compile and write
/// * `buffer` - Mutable slice of the target buffer
/// * `offset` - Byte offset where the matrix will be written
///
/// # Panics
/// Panics if offset + 64 > buffer.len()
#[wasm_bindgen(js_name = writeTransformToBuffer)]
pub fn write_transform_to_buffer(transform: Transform, buffer: &mut [u8], offset: usize) {
    let matrix = transform.build();
    write_matrix_to_buffer(buffer, offset, matrix);
}

/// Helper function to write a Matrix4 to a buffer at the specified offset.
///
/// Writes 16 f32 values (64 bytes) in column-major order matching the
/// Matrix4 internal representation.
fn write_matrix_to_buffer(
    buffer: &mut [u8],
    offset: usize,
    matrix: crate::matrix::Matrix4,
) {
    for col in 0..4 {
        for row in 0..4 {
            let value = matrix.get(row, col);
            let bytes = value.to_le_bytes();
            let pos = offset + (col * 4 + row) * 4;
            buffer[pos..pos + 4].copy_from_slice(&bytes);
        }
    }
}

// =============================================================================
// Buffer Write Helpers (internal)
// =============================================================================

/// Calculate byte offset for a sphere by ID.
fn sphere_offset(sphere_id: u32) -> usize {
    HEADER_SIZE + (sphere_id as usize) * SPHERE_SIZE
}

/// Write a single f32 to buffer at offset.
fn write_f32(buffer: &mut [u8], offset: usize, value: f32) {
    let bytes = value.to_le_bytes();
    buffer[offset..offset + 4].copy_from_slice(&bytes);
}

/// Write a single u32 to buffer at offset.
fn write_u32(buffer: &mut [u8], offset: usize, value: u32) {
    let bytes = value.to_le_bytes();
    buffer[offset..offset + 4].copy_from_slice(&bytes);
}

/// Write 3 f32s (point/vector/color) to buffer at offset.
fn write_f32x3(buffer: &mut [u8], offset: usize, x: f32, y: f32, z: f32) {
    write_f32(buffer, offset, x);
    write_f32(buffer, offset + 4, y);
    write_f32(buffer, offset + 8, z);
}

// =============================================================================
// Buffer Write Functions (exported to JavaScript)
// =============================================================================

/// Returns the required buffer size for a scene with the given maximum number of spheres.
#[wasm_bindgen(js_name = getSceneBufferSize)]
pub fn get_scene_buffer_size(max_spheres: u32) -> u32 {
    (HEADER_SIZE + SPHERE_SIZE * max_spheres as usize) as u32
}

/// Writes camera data (from, to, up points) to the buffer.
#[wasm_bindgen(js_name = writeCameraToBuffer)]
#[allow(clippy::too_many_arguments)]
pub fn write_camera_to_buffer(
    buffer: &mut [u8],
    from_x: f32,
    from_y: f32,
    from_z: f32,
    to_x: f32,
    to_y: f32,
    to_z: f32,
    up_x: f32,
    up_y: f32,
    up_z: f32,
) {
    write_f32x3(buffer, OFF_CAMERA_FROM, from_x, from_y, from_z);
    write_f32x3(buffer, OFF_CAMERA_TO, to_x, to_y, to_z);
    write_f32x3(buffer, OFF_CAMERA_UP, up_x, up_y, up_z);
}

/// Writes light data (position and color) to the buffer.
#[wasm_bindgen(js_name = writeLightToBuffer)]
pub fn write_light_to_buffer(
    buffer: &mut [u8],
    x: f32,
    y: f32,
    z: f32,
    r: f32,
    g: f32,
    b: f32,
) {
    write_f32x3(buffer, OFF_LIGHT_POS, x, y, z);
    write_f32x3(buffer, OFF_LIGHT_COLOR, r, g, b);
}

/// Initializes a sphere in the buffer with an identity transform.
///
/// Also updates the sphere count to include this sphere.
#[wasm_bindgen(js_name = initializeSphereInBuffer)]
pub fn initialize_sphere_in_buffer(buffer: &mut [u8], sphere_id: u32) {
    // Update sphere count (sphere_id is 0-indexed, so count = id + 1)
    let new_count = sphere_id + 1;
    let current_count = read_u32(buffer, OFF_SPHERE_COUNT);
    if new_count > current_count {
        write_u32(buffer, OFF_SPHERE_COUNT, new_count);
    }

    // Write identity matrix at sphere's transform offset
    let offset = sphere_offset(sphere_id);
    write_matrix_to_buffer(buffer, offset, crate::matrix::Matrix4::identity());
}

/// Writes a sphere's color to the buffer.
#[wasm_bindgen(js_name = writeSphereColorToBuffer)]
pub fn write_sphere_color_to_buffer(
    buffer: &mut [u8],
    sphere_id: u32,
    r: f32,
    g: f32,
    b: f32,
) {
    let offset = sphere_offset(sphere_id) + 64; // Color is at offset 64 within sphere
    write_f32x3(buffer, offset, r, g, b);
}

/// Writes a sphere's material properties to the buffer.
#[wasm_bindgen(js_name = writeSphereMaterialToBuffer)]
pub fn write_sphere_material_to_buffer(
    buffer: &mut [u8],
    sphere_id: u32,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
) {
    let offset = sphere_offset(sphere_id) + 76; // Material is at offset 76 within sphere
    write_f32(buffer, offset, ambient);
    write_f32(buffer, offset + 4, diffuse);
    write_f32(buffer, offset + 8, specular);
    write_f32(buffer, offset + 12, shininess);
}

/// Writes a sphere's transform to the buffer.
#[wasm_bindgen(js_name = writeSphereTransformToBuffer)]
pub fn write_sphere_transform_to_buffer(
    transform: Transform,
    buffer: &mut [u8],
    sphere_id: u32,
) {
    let offset = sphere_offset(sphere_id);
    let matrix = transform.build();
    write_matrix_to_buffer(buffer, offset, matrix);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::Matrix4;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn create_empty_scene() {
        let scene = Scene::new(100, 100, None);
        assert_eq!(scene.get_sphere_count(), 0);
    }

    #[wasm_bindgen_test]
    fn add_and_count_spheres() {
        let mut scene = Scene::new(100, 100, None);
        scene.add_sphere();
        scene.add_sphere();
        assert_eq!(scene.get_sphere_count(), 2);
    }

    #[wasm_bindgen_test]
    fn remove_sphere() {
        let mut scene = Scene::new(100, 100, None);
        let id1 = scene.add_sphere();
        let id2 = scene.add_sphere();
        let id3 = scene.add_sphere();

        assert!(scene.remove_sphere(id2));
        assert_eq!(scene.get_sphere_count(), 2);

        // Verify remaining spheres can still be modified
        assert!(scene.set_sphere_color(id1, 1.0, 0.0, 0.0).is_ok());
        assert!(scene.set_sphere_color(id3, 0.0, 0.0, 1.0).is_ok());

        // Verify removed sphere returns error
        assert!(scene.set_sphere_color(id2, 0.0, 1.0, 0.0).is_err());
    }

    #[wasm_bindgen_test]
    fn set_sphere_transform() {
        let mut scene = Scene::new(100, 100, None);
        let id = scene.add_sphere();

        let transform = Transform::new()
            .scale(2.0, 2.0, 2.0)
            .translate(1.0, 0.0, 0.0);

        assert!(scene.set_sphere_transform(id, transform).is_ok());
    }

    #[wasm_bindgen_test]
    fn invalid_sphere_id_returns_error() {
        let mut scene = Scene::new(100, 100, None);
        assert!(scene.set_sphere_color(999, 1.0, 0.0, 0.0).is_err());
    }

    #[wasm_bindgen_test]
    fn configure_scene_for_rendering() {
        // Note: We cannot test render() directly in Node.js because ImageData
        // is a browser-only API. This test verifies the scene can be fully
        // configured without errors.
        let mut scene = Scene::new(10, 10, None);

        // Add a sphere
        let sphere_id = scene.add_sphere();
        scene.set_sphere_color(sphere_id, 1.0, 0.2, 0.1).unwrap();

        // Set up light
        scene.set_light(-10.0, 10.0, -10.0, 1.0, 1.0, 1.0);

        // Position camera
        scene
            .look_at(0.0, 0.0, -5.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0)
            .expect("look_at should not fail with valid view parameters");

        // Verify scene is configured correctly
        assert_eq!(scene.get_sphere_count(), 1);
        assert!(scene.world.light.is_some());
    }

    #[wasm_bindgen_test]
    fn look_at_creates_valid_transform() {
        use crate::transform::view_transform;

        // Test view_transform directly to verify it produces an invertible matrix
        let transform = view_transform(
            Tuple::point(0.0, 0.0, -5.0),
            Tuple::point(0.0, 0.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
        );

        // This should not panic
        let inverse = transform.inverse();
        assert!(
            inverse.is_some(),
            "view_transform should produce an invertible matrix"
        );
    }

    #[wasm_bindgen_test]
    fn write_transform_to_buffer_identity() {
        let transform = Transform::new();
        let mut buffer = vec![0u8; 64];

        write_transform_to_buffer(transform, &mut buffer, 0);

        let matrix = read_matrix(&buffer, 0);
        assert_eq!(matrix, Matrix4::identity());
    }

    #[wasm_bindgen_test]
    fn write_transform_to_buffer_scale() {
        let transform = Transform::new().scale(2.0, 3.0, 4.0);
        let mut buffer = vec![0u8; 64];

        write_transform_to_buffer(transform, &mut buffer, 0);

        let matrix = read_matrix(&buffer, 0);
        let expected = Matrix4::scaling(2.0, 3.0, 4.0);
        assert_eq!(matrix, expected);
    }

    #[wasm_bindgen_test]
    fn write_transform_to_buffer_translate() {
        let transform = Transform::new().translate(1.0, 2.0, 3.0);
        let mut buffer = vec![0u8; 64];

        write_transform_to_buffer(transform, &mut buffer, 0);

        let matrix = read_matrix(&buffer, 0);
        let expected = Matrix4::translation(1.0, 2.0, 3.0);
        assert_eq!(matrix, expected);
    }

    #[wasm_bindgen_test]
    fn write_transform_to_buffer_chained() {
        // This matches the "right sphere" transform from main.ts
        let transform = Transform::new()
            .scale(0.5, 0.5, 0.5)
            .translate(1.5, 0.5, -0.5);

        let mut buffer = vec![0u8; 64];
        write_transform_to_buffer(transform, &mut buffer, 0);

        let matrix = read_matrix(&buffer, 0);

        // Transform applies operations in reverse order
        let expected =
            Matrix4::translation(1.5, 0.5, -0.5) * Matrix4::scaling(0.5, 0.5, 0.5);

        assert_eq!(matrix, expected);
    }

    #[wasm_bindgen_test]
    fn write_transform_to_buffer_at_offset() {
        let transform = Transform::new().scale(2.0, 2.0, 2.0);
        let mut buffer = vec![0u8; 128];

        // Write at offset 64
        write_transform_to_buffer(transform, &mut buffer, 64);

        // Verify first 64 bytes are still zero
        assert_eq!(&buffer[0..64], &[0u8; 64]);

        // Verify matrix was written at offset
        let matrix = read_matrix(&buffer, 64);
        let expected = Matrix4::scaling(2.0, 2.0, 2.0);
        assert_eq!(matrix, expected);
    }

    // =========================================================================
    // Buffer Write Function Tests
    // =========================================================================

    #[wasm_bindgen_test]
    fn get_scene_buffer_size_calculates_correctly() {
        // 0 spheres: just header
        assert_eq!(get_scene_buffer_size(0), HEADER_SIZE as u32);

        // 1 sphere: header + 1 sphere
        assert_eq!(get_scene_buffer_size(1), (HEADER_SIZE + SPHERE_SIZE) as u32);

        // 256 spheres
        assert_eq!(
            get_scene_buffer_size(256),
            (HEADER_SIZE + 256 * SPHERE_SIZE) as u32
        );
    }

    #[wasm_bindgen_test]
    fn write_camera_to_buffer_roundtrip() {
        let mut buffer = vec![0u8; HEADER_SIZE + SPHERE_SIZE];

        write_camera_to_buffer(
            &mut buffer,
            1.0,
            2.0,
            3.0, // from
            4.0,
            5.0,
            6.0, // to
            0.0,
            1.0,
            0.0, // up
        );

        let from = read_point(&buffer, OFF_CAMERA_FROM);
        let to = read_point(&buffer, OFF_CAMERA_TO);
        let up = read_vector(&buffer, OFF_CAMERA_UP);

        assert_eq!(from, Tuple::point(1.0, 2.0, 3.0));
        assert_eq!(to, Tuple::point(4.0, 5.0, 6.0));
        assert_eq!(up, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[wasm_bindgen_test]
    fn write_light_to_buffer_roundtrip() {
        let mut buffer = vec![0u8; HEADER_SIZE + SPHERE_SIZE];

        write_light_to_buffer(
            &mut buffer,
            -10.0,
            10.0,
            -10.0, // position
            1.0,
            1.0,
            1.0, // color
        );

        let pos = read_point(&buffer, OFF_LIGHT_POS);
        let color = read_color(&buffer, OFF_LIGHT_COLOR);

        assert_eq!(pos, Tuple::point(-10.0, 10.0, -10.0));
        assert_eq!(color, Tuple::color(1.0, 1.0, 1.0));
    }

    #[wasm_bindgen_test]
    fn initialize_sphere_in_buffer_sets_count_and_identity() {
        let mut buffer = vec![0u8; HEADER_SIZE + SPHERE_SIZE * 3];

        // Initialize sphere 0
        initialize_sphere_in_buffer(&mut buffer, 0);
        assert_eq!(read_u32(&buffer, OFF_SPHERE_COUNT), 1);

        // Verify identity matrix
        let matrix = read_matrix(&buffer, sphere_offset(0));
        assert_eq!(matrix, Matrix4::identity());

        // Initialize sphere 2 (skipping 1)
        initialize_sphere_in_buffer(&mut buffer, 2);
        assert_eq!(read_u32(&buffer, OFF_SPHERE_COUNT), 3);
    }

    #[wasm_bindgen_test]
    fn write_sphere_color_to_buffer_roundtrip() {
        let mut buffer = vec![0u8; HEADER_SIZE + SPHERE_SIZE];

        write_sphere_color_to_buffer(&mut buffer, 0, 1.0, 0.5, 0.25);

        let color = read_color(&buffer, sphere_offset(0) + 64);
        assert_eq!(color, Tuple::color(1.0, 0.5, 0.25));
    }

    #[wasm_bindgen_test]
    fn write_sphere_material_to_buffer_roundtrip() {
        let mut buffer = vec![0u8; HEADER_SIZE + SPHERE_SIZE];

        write_sphere_material_to_buffer(&mut buffer, 0, 0.1, 0.7, 0.3, 200.0);

        let offset = sphere_offset(0) + 76;
        assert_eq!(read_f32(&buffer, offset), 0.1);
        assert_eq!(read_f32(&buffer, offset + 4), 0.7);
        assert_eq!(read_f32(&buffer, offset + 8), 0.3);
        assert_eq!(read_f32(&buffer, offset + 12), 200.0);
    }

    #[wasm_bindgen_test]
    fn write_sphere_transform_to_buffer_roundtrip() {
        let mut buffer = vec![0u8; HEADER_SIZE + SPHERE_SIZE * 2];

        let transform = Transform::new()
            .scale(0.5, 0.5, 0.5)
            .translate(1.5, 0.5, -0.5);

        write_sphere_transform_to_buffer(transform, &mut buffer, 1);

        let matrix = read_matrix(&buffer, sphere_offset(1));
        let expected =
            Matrix4::translation(1.5, 0.5, -0.5) * Matrix4::scaling(0.5, 0.5, 0.5);
        assert_eq!(matrix, expected);
    }

    #[wasm_bindgen_test]
    fn complete_scene_buffer_roundtrip() {
        // Create a buffer with enough space for 4 spheres
        let buffer_size = get_scene_buffer_size(4) as usize;
        let mut buffer = vec![0u8; buffer_size];

        // Set up camera
        write_camera_to_buffer(
            &mut buffer,
            0.0,
            1.5,
            -5.0, // from
            0.0,
            1.0,
            0.0, // to
            0.0,
            1.0,
            0.0, // up
        );

        // Set up light
        write_light_to_buffer(&mut buffer, -10.0, 10.0, -10.0, 1.0, 1.0, 1.0);

        // Add floor sphere
        initialize_sphere_in_buffer(&mut buffer, 0);
        write_sphere_transform_to_buffer(
            Transform::new().scale(10.0, 0.01, 10.0),
            &mut buffer,
            0,
        );
        write_sphere_color_to_buffer(&mut buffer, 0, 1.0, 0.9, 0.9);
        write_sphere_material_to_buffer(&mut buffer, 0, 0.1, 0.9, 0.0, 200.0);

        // Verify sphere count
        assert_eq!(read_u32(&buffer, OFF_SPHERE_COUNT), 1);

        // Verify camera
        let from = read_point(&buffer, OFF_CAMERA_FROM);
        assert_eq!(from, Tuple::point(0.0, 1.5, -5.0));

        // Verify floor sphere color
        let color = read_color(&buffer, sphere_offset(0) + 64);
        assert_eq!(color, Tuple::color(1.0, 0.9, 0.9));
    }
}
