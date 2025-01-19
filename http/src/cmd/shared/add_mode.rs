#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cmd) enum HAddMode {
    Append,
    Equip,
    Insert(rc::Idx),
    Replace(rc::Idx),
}
impl Into<rc::SolOrdAddMode> for &HAddMode {
    fn into(self) -> rc::SolOrdAddMode {
        match self {
            HAddMode::Append => rc::SolOrdAddMode::Append,
            HAddMode::Equip => rc::SolOrdAddMode::Equip,
            HAddMode::Insert(i) => rc::SolOrdAddMode::Insert(*i),
            HAddMode::Replace(i) => rc::SolOrdAddMode::Replace(*i),
        }
    }
}
