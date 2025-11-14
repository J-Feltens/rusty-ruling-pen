pub mod canvas;
pub mod vec2d;

// Re-export so callers can write `sprites::Sprite` directly
pub use canvas::Canvas;
pub use vec2d::Vector2d;
