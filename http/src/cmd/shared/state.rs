#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum State {
    Ghost,
    Offline,
    Online,
    Active,
    Overload,
}
impl Into<reefast::State> for State {
    fn into(self) -> reefast::State {
        match self {
            Self::Offline => reefast::State::Offline,
            Self::Online => reefast::State::Online,
            Self::Active => reefast::State::Active,
            Self::Ghost => reefast::State::Ghost,
            Self::Overload => reefast::State::Overload,
        }
    }
}
