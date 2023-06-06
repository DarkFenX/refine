#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CState {
    Ghost,
    Offline,
    Online,
    Active,
    Overload,
}
impl From<&rc::consts::State> for CState {
    fn from(state: &rc::consts::State) -> Self {
        match state {
            rc::consts::State::Ghost => Self::Ghost,
            rc::consts::State::Offline => Self::Offline,
            rc::consts::State::Online => Self::Online,
            rc::consts::State::Active => Self::Active,
            rc::consts::State::Overload => Self::Overload,
        }
    }
}
impl Into<rc::consts::State> for &CState {
    fn into(self) -> rc::consts::State {
        match self {
            CState::Ghost => rc::consts::State::Ghost,
            CState::Offline => rc::consts::State::Offline,
            CState::Online => rc::consts::State::Online,
            CState::Active => rc::consts::State::Active,
            CState::Overload => rc::consts::State::Overload,
        }
    }
}
