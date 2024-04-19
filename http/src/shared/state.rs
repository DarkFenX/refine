#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HState {
    Ghost,
    Offline,
    Online,
    Active,
    Overload,
}
impl From<&rc::SsItemState> for HState {
    fn from(core_state: &rc::SsItemState) -> Self {
        match core_state {
            rc::SsItemState::Offline => Self::Offline,
            rc::SsItemState::Online => Self::Online,
            rc::SsItemState::Active => Self::Active,
            rc::SsItemState::Ghost => Self::Ghost,
            rc::SsItemState::Overload => Self::Overload,
        }
    }
}
impl Into<rc::SsItemState> for &HState {
    fn into(self) -> rc::SsItemState {
        match self {
            HState::Offline => rc::SsItemState::Offline,
            HState::Online => rc::SsItemState::Online,
            HState::Active => rc::SsItemState::Active,
            HState::Ghost => rc::SsItemState::Ghost,
            HState::Overload => rc::SsItemState::Overload,
        }
    }
}
