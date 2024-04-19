#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CState {
    Offline,
    Online,
    Active,
    Overload,
}
impl From<&rc::ad::AState> for CState {
    fn from(state: &rc::ad::AState) -> Self {
        match state {
            rc::ad::AState::Offline => Self::Offline,
            rc::ad::AState::Online => Self::Online,
            rc::ad::AState::Active => Self::Active,
            rc::ad::AState::Overload => Self::Overload,
        }
    }
}
impl Into<rc::ad::AState> for &CState {
    fn into(self) -> rc::ad::AState {
        match self {
            CState::Offline => rc::ad::AState::Offline,
            CState::Online => rc::ad::AState::Online,
            CState::Active => rc::ad::AState::Active,
            CState::Overload => rc::ad::AState::Overload,
        }
    }
}
