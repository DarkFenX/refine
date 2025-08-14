#[derive(Copy, Clone, serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(crate) struct HCoordinates {
    x: rc::AttrVal,
    y: rc::AttrVal,
    z: rc::AttrVal,
}
impl From<rc::Coordinates> for HCoordinates {
    fn from(core_coordinates: rc::Coordinates) -> Self {
        Self {
            x: core_coordinates.x,
            y: core_coordinates.y,
            z: core_coordinates.z,
        }
    }
}
impl From<HCoordinates> for rc::Coordinates {
    fn from(h_coordinates: HCoordinates) -> Self {
        Self {
            x: h_coordinates.x,
            y: h_coordinates.y,
            z: h_coordinates.z,
        }
    }
}
