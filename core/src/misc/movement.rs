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
    /// Radians relatively X axis counter-clockwise.
    pub azimuth: AttrVal,
    /// Radians of elevation.
    pub elevation: AttrVal,
}
impl From<UDirection> for Direction {
    fn from(u_direction: UDirection) -> Self {
        Self {
            azimuth: u_direction.azimuth,
            elevation: u_direction.elevation,
        }
    }
}
impl From<Direction> for UDirection {
    fn from(direction: Direction) -> Self {
        Self {
            azimuth: direction.azimuth,
            elevation: direction.elevation,
        }
    }
}
