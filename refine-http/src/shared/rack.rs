use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HModRack {
    High,
    Mid,
    Low,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HModRack {
    pub(crate) fn from_core(core_rack: rc::ModRack) -> Self {
        match core_rack {
            rc::ModRack::High => Self::High,
            rc::ModRack::Mid => Self::Mid,
            rc::ModRack::Low => Self::Low,
        }
    }
    pub(crate) fn into_core(self) -> rc::ModRack {
        match self {
            Self::High => rc::ModRack::High,
            Self::Mid => rc::ModRack::Mid,
            Self::Low => rc::ModRack::Low,
        }
    }
}
