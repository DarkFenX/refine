#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HMinionState {
    InBay,
    InSpace,
    Engaging,
}
impl From<&rc::MinionState> for HMinionState {
    fn from(core_minion_state: &rc::MinionState) -> Self {
        match core_minion_state {
            rc::MinionState::InBay => Self::InBay,
            rc::MinionState::InSpace => Self::InSpace,
            rc::MinionState::Engaging => Self::Engaging,
        }
    }
}
impl From<&HMinionState> for rc::MinionState {
    fn from(h_minion_state: &HMinionState) -> Self {
        match h_minion_state {
            HMinionState::InBay => Self::InBay,
            HMinionState::InSpace => Self::InSpace,
            HMinionState::Engaging => Self::Engaging,
        }
    }
}
