#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HAddMode {
    Append,
    Equip,
    Insert(rc::Idx),
    Place(rc::Idx, bool),
}
impl Into<rc::OrdAddMode> for &HAddMode {
    fn into(self) -> rc::OrdAddMode {
        match self {
            HAddMode::Append => rc::OrdAddMode::Append,
            HAddMode::Equip => rc::OrdAddMode::Equip,
            HAddMode::Insert(i) => rc::OrdAddMode::Insert(*i),
            HAddMode::Place(i, r) => rc::OrdAddMode::Place(*i, *r),
        }
    }
}
