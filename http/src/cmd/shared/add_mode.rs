#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum AddMode {
    Append,
    Equip,
    Insert(rc::ReeIdx),
    Place(rc::ReeIdx, bool),
}
impl Into<rc::OrdAddMode> for AddMode {
    fn into(self) -> rc::OrdAddMode {
        match self {
            Self::Append => rc::OrdAddMode::Append,
            Self::Equip => rc::OrdAddMode::Equip,
            Self::Insert(i) => rc::OrdAddMode::Insert(i),
            Self::Place(i, r) => rc::OrdAddMode::Place(i, r),
        }
    }
}
