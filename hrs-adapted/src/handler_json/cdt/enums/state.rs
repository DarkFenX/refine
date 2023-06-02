#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum State {
    Ghost,
    Offline,
    Online,
    Active,
    Overload,
}
impl From<&rc::consts::State> for State {
    fn from(value: &rc::consts::State) -> Self {
        match value {
            rc::consts::State::Ghost => Self::Ghost,
            rc::consts::State::Offline => Self::Offline,
            rc::consts::State::Online => Self::Online,
            rc::consts::State::Active => Self::Active,
            rc::consts::State::Overload => Self::Overload,
        }
    }
}
impl Into<rc::consts::State> for &State {
    fn into(self) -> rc::consts::State {
        match self {
            State::Ghost => rc::consts::State::Ghost,
            State::Offline => rc::consts::State::Offline,
            State::Online => rc::consts::State::Online,
            State::Active => rc::consts::State::Active,
            State::Overload => rc::consts::State::Overload,
        }
    }
}
