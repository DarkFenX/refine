#[derive(Copy, Clone, serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(crate) struct HMovement {
    plane: rc::AttrVal,
    elevation: rc::AttrVal,
    speed: rc::AttrVal,
}
impl From<rc::Movement> for HMovement {
    fn from(core_movement: rc::Movement) -> Self {
        Self {
            plane: core_movement.direction.plane,
            elevation: core_movement.direction.elevation,
            speed: core_movement.speed,
        }
    }
}
impl From<HMovement> for rc::Movement {
    fn from(h_movement: HMovement) -> Self {
        Self {
            direction: rc::Direction {
                plane: h_movement.plane,
                elevation: h_movement.elevation,
            },
            speed: h_movement.speed,
        }
    }
}
