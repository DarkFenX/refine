use crate::{def::AttrVal, ud::UCoordinates};

#[derive(Copy, Clone)]
pub struct Coordinates {
    pub x: AttrVal,
    pub y: AttrVal,
    pub z: AttrVal,
}
impl Coordinates {
    pub fn new(x: AttrVal, y: AttrVal, z: AttrVal) -> Self {
        Self { x, y, z }
    }
}
impl From<UCoordinates> for Coordinates {
    fn from(u_coordinates: UCoordinates) -> Self {
        Self {
            x: u_coordinates.x,
            y: u_coordinates.y,
            z: u_coordinates.z,
        }
    }
}
impl From<Coordinates> for UCoordinates {
    fn from(coordinates: Coordinates) -> Self {
        Self {
            x: coordinates.x,
            y: coordinates.y,
            z: coordinates.z,
        }
    }
}
