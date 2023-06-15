#[derive(serde::Serialize, serde::Deserialize)]
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
impl Into<rc::ModRack> for &HModRack {
    fn into(self) -> rc::ModRack {
        match self {
            HModRack::High => rc::ModRack::High,
            HModRack::Mid => rc::ModRack::Mid,
            HModRack::Low => rc::ModRack::Low,
        }
    }
}
