use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HModRack {
    High,
    Mid,
    Low,
}
impl From<&rc::ModRack> for HModRack {
    fn from(core_rack: &rc::ModRack) -> Self {
        match core_rack {
            rc::ModRack::High => Self::High,
            rc::ModRack::Mid => Self::Mid,
            rc::ModRack::Low => Self::Low,
        }
    }
}
impl From<&HModRack> for rc::ModRack {
    fn from(h_rack: &HModRack) -> Self {
        match h_rack {
            HModRack::High => Self::High,
            HModRack::Mid => Self::Mid,
            HModRack::Low => Self::Low,
        }
    }
}
