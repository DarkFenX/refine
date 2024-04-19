#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cmd) enum HAddMode {
    Append,
    Equip,
    Insert(rc::Idx),
    Place(rc::Idx, bool),
}
impl Into<rc::SsOrdAddMode> for &HAddMode {
    fn into(self) -> rc::SsOrdAddMode {
        match self {
            HAddMode::Append => rc::SsOrdAddMode::Append,
            HAddMode::Equip => rc::SsOrdAddMode::Equip,
            HAddMode::Insert(i) => rc::SsOrdAddMode::Insert(*i),
            HAddMode::Place(i, r) => rc::SsOrdAddMode::Place(*i, *r),
        }
    }
}
