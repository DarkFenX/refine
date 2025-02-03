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
impl From<&HModRack> for rc::SolModRack {
    fn from(h_rack: &HModRack) -> Self {
        match h_rack {
            HModRack::High => Self::High,
            HModRack::Mid => Self::Mid,
            HModRack::Low => Self::Low,
        }
    }
}
