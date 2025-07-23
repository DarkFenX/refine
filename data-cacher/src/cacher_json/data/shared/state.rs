#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::cacher_json) enum CState {
    Ghost,
    Offline,
    Online,
    Active,
    Overload,
}
impl From<&rc::ad::AState> for CState {
    fn from(a_state: &rc::ad::AState) -> Self {
        match a_state {
            rc::ad::AState::Ghost => Self::Ghost,
            rc::ad::AState::Offline => Self::Offline,
            rc::ad::AState::Online => Self::Online,
            rc::ad::AState::Active => Self::Active,
            rc::ad::AState::Overload => Self::Overload,
        }
    }
}
impl From<&CState> for rc::ad::AState {
    fn from(c_state: &CState) -> Self {
        match c_state {
            CState::Ghost => Self::Ghost,
            CState::Offline => Self::Offline,
            CState::Online => Self::Online,
            CState::Active => Self::Active,
            CState::Overload => Self::Overload,
        }
    }
}
