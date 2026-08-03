use crate::{Value, misc::Xyz};

/// Position of an item in space, in meters.
///
/// Keep in mind that in EVE, ships are spheres with non-zero radius, and overview distance is a
/// surface-to-surface distance. So, ships which are exactly 10 km apart center-to-center will have
/// shorter surface-to-surface distance. This matters a lot, since most of EVE effects are using the
/// surface-to-surface distance to calculate their strength.
#[cfg_attr(
    feature = "serde",
    derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)
)]
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
