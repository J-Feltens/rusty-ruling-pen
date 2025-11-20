pub mod ivec2d;
pub mod matrices;
pub mod vec2d;
pub mod vec3d;
pub mod vec4d;

// Re-export so callers can write `sprites::Sprite` directly
pub use ivec2d::IntegerVector2d;
pub use matrices::Matrix3x3;
pub use vec2d::Vector2d;
pub use vec3d::Vector3d;
pub use vec4d::Vector4d;
