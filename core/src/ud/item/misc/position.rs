use crate::def::AttrVal;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub(crate) struct UPosition {
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
    // Degrees relatively X axis counter-clockwise
    pub(crate) plane: AttrVal,
    // Degrees of elevation
    pub(crate) elevation: AttrVal,
}
