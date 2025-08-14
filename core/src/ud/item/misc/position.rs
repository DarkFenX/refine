use crate::AttrVal;

#[derive(Copy, Clone, Default)]
pub(crate) struct UPosition {
    pub(crate) coordinate: UCoordinate,
    pub(crate) direction: UDirection,
    pub(crate) speed: AttrVal,
}

#[derive(Copy, Clone, Default)]
pub(crate) struct UCoordinate {
    pub(crate) x: AttrVal,
    pub(crate) y: AttrVal,
    pub(crate) z: AttrVal,
}

#[derive(Copy, Clone, Default)]
pub(crate) struct UDirection {
    // Degrees relatively X axis counter-clockwise
    pub(crate) plane: AttrVal,
    // Degrees of elevation
    pub(crate) elevation: AttrVal,
}
