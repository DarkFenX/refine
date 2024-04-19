#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HModRack {
    High,
    Mid,
    Low,
}
impl From<&rc::SsModRack> for HModRack {
    fn from(core_rack: &rc::SsModRack) -> Self {
        match core_rack {
            rc::SsModRack::High => Self::High,
            rc::SsModRack::Mid => Self::Mid,
            rc::SsModRack::Low => Self::Low,
        }
    }
}
impl Into<rc::SsModRack> for &HModRack {
    fn into(self) -> rc::SsModRack {
        match self {
            HModRack::High => rc::SsModRack::High,
            HModRack::Mid => rc::SsModRack::Mid,
            HModRack::Low => rc::SsModRack::Low,
        }
    }
}
