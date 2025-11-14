pub mod canvas;
pub mod colors;
pub mod scanline;

// Re-export so callers can write `sprites::Sprite` directly
pub use canvas::Canvas;
pub use colors::{BLUE, CYAN, Color, GREEN, MAGENTA, RED, WHITE, YELLOW};
pub use scanline::EdgeTable;
