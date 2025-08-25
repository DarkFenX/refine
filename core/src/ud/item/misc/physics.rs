use crate::{def::AttrVal, util::Xyz};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub(crate) struct UPhysics {
    pub(crate) coordinates: Xyz,
    // Direction as a unit vector relatively object coordinates
    pub(crate) direction: Xyz,
    pub(crate) speed: AttrVal,
}
