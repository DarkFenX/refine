#[derive(Copy, Clone, serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(crate) struct HMovement {
    azimuth: rc::AttrVal,
    elevation: rc::AttrVal,
    speed: rc::AttrVal,
}
impl From<rc::Movement> for HMovement {
    fn from(core_movement: rc::Movement) -> Self {
        Self {
            azimuth: core_movement.direction.azimuth,
            elevation: core_movement.direction.elevation,
            speed: core_movement.speed,
        }
    }
}
impl From<HMovement> for rc::Movement {
    fn from(h_movement: HMovement) -> Self {
        Self {
            direction: rc::Direction {
                azimuth: h_movement.azimuth,
                elevation: h_movement.elevation,
            },
            speed: h_movement.speed,
        }
    }
}
