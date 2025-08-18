use crate::{def::AttrVal, ud::UDirection};

#[derive(Copy, Clone)]
pub struct Movement {
    /// Movement direction of an item.
    pub direction: Direction,
    /// Portion of speed item moves with, relatively its max speed.
    pub speed: AttrVal,
}

#[derive(Copy, Clone)]
pub struct Direction {
    /// Radians relatively X axis counter-clockwise.
    pub plane: AttrVal,
    /// Radians of elevation.
    pub elevation: AttrVal,
}
impl From<UDirection> for Direction {
    fn from(u_direction: UDirection) -> Self {
        Self {
            plane: u_direction.plane,
            elevation: u_direction.elevation,
        }
    }
}
impl From<Direction> for UDirection {
    fn from(direction: Direction) -> Self {
        Self {
            plane: direction.plane,
            elevation: direction.elevation,
        }
    }
}
