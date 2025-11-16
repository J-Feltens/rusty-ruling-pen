pub mod canvas;
pub mod colors;
pub mod scanline;
pub mod triangles;

// Re-export so callers can write `sprites::Sprite` directly
pub use canvas::Canvas;
pub use colors::{
    BLACK, BLUE, CYAN, Color, GREEN, MAGENTA, RED, WHITE, YELLOW, alpha_blend, rgb2u32,
};
pub use scanline::EdgeTable;
pub use triangles::Triangle3d;
