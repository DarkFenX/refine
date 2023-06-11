#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HAddMode {
    Append,
    Equip,
    Insert(rc::ReeIdx),
    Place(rc::ReeIdx, bool),
}
impl Into<rc::OrdAddMode> for HAddMode {
    fn into(self) -> rc::OrdAddMode {
        match self {
            Self::Append => rc::OrdAddMode::Append,
            Self::Equip => rc::OrdAddMode::Equip,
            Self::Insert(i) => rc::OrdAddMode::Insert(i),
            Self::Place(i, r) => rc::OrdAddMode::Place(i, r),
        }
    }
}
