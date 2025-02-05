#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HMinionState {
    InBay,
    InSpace,
    Engaging,
}
impl From<&rc::SolMinionState> for HMinionState {
    fn from(core_minion_state: &rc::SolMinionState) -> Self {
        match core_minion_state {
            rc::SolMinionState::InBay => Self::InBay,
            rc::SolMinionState::InSpace => Self::InSpace,
            rc::SolMinionState::Engaging => Self::Engaging,
        }
    }
}
impl From<&HMinionState> for rc::SolMinionState {
    fn from(h_minion_state: &HMinionState) -> Self {
        match h_minion_state {
            HMinionState::InBay => Self::InBay,
            HMinionState::InSpace => Self::InSpace,
            HMinionState::Engaging => Self::Engaging,
        }
    }
}
