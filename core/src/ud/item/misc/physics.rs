use crate::def::AttrVal;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub(crate) struct UPhysics {
    pub(crate) coordinates: UCoordinates,
    pub(crate) direction: UDirection,
    pub(crate) speed: AttrVal,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub(crate) struct UCoordinates {
    pub(crate) x: AttrVal,
    pub(crate) y: AttrVal,
    pub(crate) z: AttrVal,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub(crate) struct UDirection {
    // Radians relatively X axis counter-clockwise
    pub(crate) azimuth: AttrVal,
    // Radians of elevation
    pub(crate) elevation: AttrVal,
}
