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
impl From<&HState> for rc::SolItemState {
    fn from(h_state: &HState) -> Self {
        match h_state {
            HState::Offline => Self::Offline,
            HState::Online => Self::Online,
            HState::Active => Self::Active,
            HState::Ghost => Self::Ghost,
            HState::Overload => Self::Overload,
        }
    }
}
