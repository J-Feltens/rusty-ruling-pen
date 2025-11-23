pub mod canvas;
pub mod colors;
pub mod scanline;
mod shapes;
pub mod triangles;

// Re-export so callers can write `sprites::Sprite` directly
pub use canvas::Canvas;
pub use colors::{Color, WHITE, alpha_blend, rgb2u32};
//noinspection RsUnusedImport
pub use shapes::Cube;
pub use triangles::Triangle3d;
