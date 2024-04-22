#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HModRack {
    High,
    Mid,
    Low,
}
impl From<&rc::SolModRack> for HModRack {
    fn from(core_rack: &rc::SolModRack) -> Self {
        match core_rack {
            rc::SolModRack::High => Self::High,
            rc::SolModRack::Mid => Self::Mid,
            rc::SolModRack::Low => Self::Low,
        }
    }
}
impl Into<rc::SolModRack> for &HModRack {
    fn into(self) -> rc::SolModRack {
        match self {
            HModRack::High => rc::SolModRack::High,
            HModRack::Mid => rc::SolModRack::Mid,
            HModRack::Low => rc::SolModRack::Low,
        }
    }
}
