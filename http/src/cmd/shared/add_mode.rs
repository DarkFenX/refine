#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum AddMode {
    Append,
    Equip,
    Insert(reefast_core::ReeIdx),
    Place(reefast_core::ReeIdx, bool),
}
impl Into<reefast_core::OrdAddMode> for AddMode {
    fn into(self) -> reefast_core::OrdAddMode {
        match self {
            Self::Append => reefast_core::OrdAddMode::Append,
            Self::Equip => reefast_core::OrdAddMode::Equip,
            Self::Insert(i) => reefast_core::OrdAddMode::Insert(i),
            Self::Place(i, r) => reefast_core::OrdAddMode::Place(i, r),
        }
    }
}
