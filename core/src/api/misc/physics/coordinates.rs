use crate::{def::AttrVal, util::Xyz};

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
impl From<Xyz> for Coordinates {
    fn from(coordinates: Xyz) -> Self {
        Self {
            x: coordinates.x,
            y: coordinates.y,
            z: coordinates.z,
        }
    }
}
impl From<Coordinates> for Xyz {
    fn from(coordinates: Coordinates) -> Self {
        Self {
            x: coordinates.x,
            y: coordinates.y,
            z: coordinates.z,
        }
    }
}
