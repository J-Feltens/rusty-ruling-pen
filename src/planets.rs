use crate::{colors::Color, util::Vector2d};

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Planet {
    pub position: Vector2d,
    pub mass: f64,
    pub velocity: Vector2d,
    pub acceleration: Vector2d,
    pub color: Color,
}

impl Planet {
    pub fn new(
        position: Vector2d,
        mass: f64,
        velocity: Vector2d,
        acceleration: Vector2d,
        color: Color,
    ) -> Self {
        Planet {
            position,
            mass,
            velocity,
            acceleration,
            color,
        }
    }

    pub fn pos_as_int(&self) -> (i32, i32) {
        return (self.position.x as i32, self.position.y as i32);
    }

    pub fn mass_as_int(&self) -> i32 {
        return self.mass as i32;
    }
}
