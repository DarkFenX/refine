#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum State {
    Ghost,
    Offline,
    Online,
    Active,
    Overload,
}
impl Into<reefast_core::State> for State {
    fn into(self) -> reefast_core::State {
        match self {
            Self::Offline => reefast_core::State::Offline,
            Self::Online => reefast_core::State::Online,
            Self::Active => reefast_core::State::Active,
            Self::Ghost => reefast_core::State::Ghost,
            Self::Overload => reefast_core::State::Overload,
        }
    }
}
