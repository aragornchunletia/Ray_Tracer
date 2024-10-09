// Module declarations
pub mod vec3;
pub mod color;
pub mod progress_bar;
pub mod ray;
pub mod hittable;
pub mod hittable_list;

// Re-exports
pub use vec3::*;
pub use color::*;
pub use ray::*;
pub use hittable::*;
pub use hittable_list::*;

// Constants
pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.14159265358979323846264338327950288_f64;

// Function to convert degrees to radians
pub fn degrees_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}
