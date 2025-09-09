pub mod primitives;
pub mod sprite;

// Re-export so callers can write `sprites::Sprite` directly
pub use primitives::Circle;
pub use sprite::Sprite;
