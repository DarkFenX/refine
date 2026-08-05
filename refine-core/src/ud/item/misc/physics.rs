use crate::{
    misc::Xyz,
    num::{PValue, Value},
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub(crate) struct UPhysics {
    pub(crate) coordinates: Xyz,
    // Direction as a unit vector relatively object coordinates
    pub(crate) direction: Xyz = Xyz {
        x: Value::ONE,
        y: Value::ZERO,
        z: Value::ZERO,
    },
    pub(crate) speed: PValue,
}
