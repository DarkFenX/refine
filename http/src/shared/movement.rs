use serde_tuple::{Deserialize_tuple, Serialize_tuple};

#[derive(Copy, Clone, Serialize_tuple, Deserialize_tuple)]
pub(crate) struct HMovement {
    azimuth: f64,
    elevation: f64,
    speed: f64,
}
impl HMovement {
    pub(crate) fn from_core(core_movement: rc::Movement) -> Self {
        Self {
            azimuth: core_movement.direction.azimuth.into_f64(),
            elevation: core_movement.direction.elevation.into_f64(),
            speed: core_movement.speed.into_f64(),
        }
    }
    pub(crate) fn into_core(self) -> rc::Movement {
        rc::Movement {
            direction: rc::Direction {
                azimuth: rc::Value::from_f64(self.azimuth),
                elevation: rc::Value::from_f64(self.elevation),
            },
            speed: rc::PValue::from_f64_clamped(self.speed),
        }
    }
}
