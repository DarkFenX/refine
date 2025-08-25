use ordered_float::Float;

use crate::{
    def::AttrVal,
    ud::{UDirection, UPhysics},
};

#[derive(Copy, Clone)]
pub struct Movement {
    /// Movement direction of an item.
    pub direction: Direction,
    /// Portion of speed item moves with, relatively its max speed.
    pub speed: AttrVal,
}
impl From<&UPhysics> for Movement {
    fn from(u_physics: &UPhysics) -> Self {
        Self {
            direction: u_physics.direction.into(),
            speed: u_physics.speed,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Direction {
    /// Degrees relatively X axis counter-clockwise.
    pub azimuth: AttrVal,
    /// Degrees of elevation.
    pub elevation: AttrVal,
}
impl From<UDirection> for Direction {
    fn from(u_direction: UDirection) -> Self {
        Self {
            azimuth: u_direction.y.atan2(u_direction.x).to_degrees(),
            elevation: u_direction
                .z
                .atan2((u_direction.x.powi(2) + u_direction.y.powi(2)).sqrt())
                .to_degrees(),
        }
    }
}
impl From<Direction> for UDirection {
    fn from(direction: Direction) -> Self {
        let (az_sin, az_cos) = direction.azimuth.to_radians().sin_cos();
        let (el_sin, el_cos) = direction.elevation.to_radians().sin_cos();
        Self {
            x: az_cos * el_cos,
            y: az_sin * el_cos,
            z: el_sin,
        }
    }
}
