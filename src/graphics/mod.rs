pub mod canvas;
pub mod colors;
pub mod scanline;
mod shapes;
pub mod triangles;

// Re-export so callers can write `sprites::Sprite` directly
pub use canvas::Canvas;
pub use colors::{BLACK, BLUE, CYAN, Color, GREEN, MAGENTA, RED, WHITE, YELLOW, alpha_blend};
//noinspection RsUnusedImport
pub use shapes::{calc_cube, calc_torus};
pub use triangles::Triangle3d;
