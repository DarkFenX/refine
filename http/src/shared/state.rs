#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HState {
    Ghost,
    Offline,
    Online,
    Active,
    Overload,
}
impl From<&rc::SolItemState> for HState {
    fn from(core_state: &rc::SolItemState) -> Self {
        match core_state {
            rc::SolItemState::Offline => Self::Offline,
            rc::SolItemState::Online => Self::Online,
            rc::SolItemState::Active => Self::Active,
            rc::SolItemState::Ghost => Self::Ghost,
            rc::SolItemState::Overload => Self::Overload,
        }
    }
}
impl Into<rc::SolItemState> for &HState {
    fn into(self) -> rc::SolItemState {
        match self {
            HState::Offline => rc::SolItemState::Offline,
            HState::Online => rc::SolItemState::Online,
            HState::Active => rc::SolItemState::Active,
            HState::Ghost => rc::SolItemState::Ghost,
            HState::Overload => rc::SolItemState::Overload,
        }
    }
}
