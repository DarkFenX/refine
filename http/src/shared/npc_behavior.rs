use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HNpcProp {
    Cruise,
    Chase,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HNpcProp {
    pub(crate) fn from_core(core_prop_mode: rc::NpcProp) -> Self {
        match core_prop_mode {
            rc::NpcProp::Cruise => Self::Cruise,
            rc::NpcProp::Chase => Self::Chase,
        }
    }
    pub(crate) fn into_core(self) -> rc::NpcProp {
        match self {
            Self::Cruise => rc::NpcProp::Cruise,
            Self::Chase => rc::NpcProp::Chase,
        }
    }
}
