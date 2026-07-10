use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HMinionState {
    InBay,
    InSpace,
    Engaging,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HMinionState {
    pub(crate) fn from_core(core_minion_state: rc::MinionState) -> Self {
        match core_minion_state {
            rc::MinionState::InBay => Self::InBay,
            rc::MinionState::InSpace => Self::InSpace,
            rc::MinionState::Engaging => Self::Engaging,
        }
    }
    pub(crate) fn into_core(self) -> rc::MinionState {
        match self {
            Self::InBay => rc::MinionState::InBay,
            Self::InSpace => rc::MinionState::InSpace,
            Self::Engaging => rc::MinionState::Engaging,
        }
    }
}
