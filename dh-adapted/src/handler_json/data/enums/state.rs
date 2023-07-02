#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CState {
    Ghost,
    Offline,
    Online,
    Active,
    Overload,
}
impl From<&rc::State> for CState {
    fn from(state: &rc::State) -> Self {
        match state {
            rc::State::Ghost => Self::Ghost,
            rc::State::Offline => Self::Offline,
            rc::State::Online => Self::Online,
            rc::State::Active => Self::Active,
            rc::State::Overload => Self::Overload,
        }
    }
}
impl Into<rc::State> for &CState {
    fn into(self) -> rc::State {
        match self {
            CState::Ghost => rc::State::Ghost,
            CState::Offline => rc::State::Offline,
            CState::Online => rc::State::Online,
            CState::Active => rc::State::Active,
            CState::Overload => rc::State::Overload,
        }
    }
}
