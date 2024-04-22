#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cmd) enum HAddMode {
    Append,
    Equip,
    Insert(rc::Idx),
    Place(rc::Idx, bool),
}
impl Into<rc::SolOrdAddMode> for &HAddMode {
    fn into(self) -> rc::SolOrdAddMode {
        match self {
            HAddMode::Append => rc::SolOrdAddMode::Append,
            HAddMode::Equip => rc::SolOrdAddMode::Equip,
            HAddMode::Insert(i) => rc::SolOrdAddMode::Insert(*i),
            HAddMode::Place(i, r) => rc::SolOrdAddMode::Place(*i, *r),
        }
    }
}
