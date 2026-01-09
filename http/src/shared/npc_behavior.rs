use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HNpcProp {
    Cruise,
    Chase,
}
impl From<rc::NpcProp> for HNpcProp {
    fn from(core_prop_mode: rc::NpcProp) -> Self {
        match core_prop_mode {
            rc::NpcProp::Cruise => Self::Cruise,
            rc::NpcProp::Chase => Self::Chase,
        }
    }
}
impl From<HNpcProp> for rc::NpcProp {
    fn from(h_prop_mode: HNpcProp) -> Self {
        match h_prop_mode {
            HNpcProp::Cruise => Self::Cruise,
            HNpcProp::Chase => Self::Chase,
        }
    }
}
