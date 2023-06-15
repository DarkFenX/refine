#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HState {
    Ghost,
    Offline,
    Online,
    Active,
    Overload,
}
impl From<&rc::State> for HState {
    fn from(core_state: &rc::State) -> Self {
        match core_state {
            rc::State::Offline => Self::Offline,
            rc::State::Online => Self::Online,
            rc::State::Active => Self::Active,
            rc::State::Ghost => Self::Ghost,
            rc::State::Overload => Self::Overload,
        }
    }
}
impl Into<rc::State> for &HState {
    fn into(self) -> rc::State {
        match self {
            HState::Offline => rc::State::Offline,
            HState::Online => rc::State::Online,
            HState::Active => rc::State::Active,
            HState::Ghost => rc::State::Ghost,
            HState::Overload => rc::State::Overload,
        }
    }
}
