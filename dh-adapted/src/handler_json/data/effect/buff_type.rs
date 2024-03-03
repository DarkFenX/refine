#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CBuffType {
    Everything,
    FleetShips,
}
impl From<&rc::ad::ABuffType> for CBuffType {
    fn from(buff_type: &rc::ad::ABuffType) -> Self {
        match buff_type {
            rc::ad::ABuffType::Everything => Self::Everything,
            rc::ad::ABuffType::FleetShips => Self::FleetShips,
        }
    }
}
impl Into<rc::ad::ABuffType> for &CBuffType {
    fn into(self) -> rc::ad::ABuffType {
        match self {
            CBuffType::Everything => rc::ad::ABuffType::Everything,
            CBuffType::FleetShips => rc::ad::ABuffType::FleetShips,
        }
    }
}
