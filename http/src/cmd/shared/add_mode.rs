#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum AddMode {
    Append,
    Equip,
    Insert(reefast::ReeIdx),
    Place(reefast::ReeIdx, bool),
}
impl Into<reefast::OrdAddMode> for AddMode {
    fn into(self) -> reefast::OrdAddMode {
        match self {
            Self::Append => reefast::OrdAddMode::Append,
            Self::Equip => reefast::OrdAddMode::Equip,
            Self::Insert(i) => reefast::OrdAddMode::Insert(i),
            Self::Place(i, r) => reefast::OrdAddMode::Place(i, r),
        }
    }
}
