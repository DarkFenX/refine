use crate::{misc::Xyz, num::PValue};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub(crate) struct UPhysics {
    pub(crate) coordinates: Xyz,
    // Direction as a unit vector relatively object coordinates
    pub(crate) direction: Xyz,
    pub(crate) speed: PValue,
}
