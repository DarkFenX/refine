use serde_tuple::{Deserialize_tuple, Serialize_tuple};

#[derive(Copy, Clone, Serialize_tuple, Deserialize_tuple)]
pub(crate) struct HCoordinates {
    x: f64,
    y: f64,
    z: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HCoordinates {
    pub(crate) fn from_core(core_coordinates: rc::Coordinates) -> Self {
        Self {
            x: core_coordinates.x.into_f64(),
            y: core_coordinates.y.into_f64(),
            z: core_coordinates.z.into_f64(),
        }
    }
    pub(crate) fn into_core(self) -> rc::Coordinates {
        rc::Coordinates {
            x: rc::Value::from_f64(self.x),
            y: rc::Value::from_f64(self.y),
            z: rc::Value::from_f64(self.z),
        }
    }
}
