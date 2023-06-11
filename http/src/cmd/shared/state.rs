#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HState {
    Ghost,
    Offline,
    Online,
    Active,
    Overload,
}
impl Into<rc::State> for HState {
    fn into(self) -> rc::State {
        match self {
            Self::Offline => rc::State::Offline,
            Self::Online => rc::State::Online,
            Self::Active => rc::State::Active,
            Self::Ghost => rc::State::Ghost,
            Self::Overload => rc::State::Overload,
        }
    }
}
