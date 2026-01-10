use crate::{misc::Xyz, num::Value};

#[derive(Copy, Clone)]
pub struct Coordinates {
    pub x: Value,
    pub y: Value,
    pub z: Value,
}
impl Coordinates {
    pub fn new(x: Value, y: Value, z: Value) -> Self {
        Self { x, y, z }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Coordinates {
    pub(in crate::api) fn from_xyz(xyz: Xyz) -> Self {
        Self {
            x: xyz.x,
            y: xyz.y,
            z: xyz.z,
        }
    }
    pub(in crate::api) fn into_xyz(self) -> Xyz {
        Xyz {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}
