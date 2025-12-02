mod camera;
pub mod canvas;
pub mod colors;
mod fragment_shader;
mod lighting;
pub mod scanline;
mod shapes;
pub mod triangles;

// Re-export so callers can write `sprites::Sprite` directly
pub use camera::Camera;
pub use canvas::{Canvas, SSAA};
pub use colors::alpha_blend;
pub use lighting::PointLight;
pub use shapes::{calc_cube, calc_sphere, calc_teapot, calc_torus};
pub use triangles::Triangle3d;
