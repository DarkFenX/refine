#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cmd) enum HAddMode {
    Append,
    Equip,
    Insert(rc::Idx),
    Replace(rc::Idx),
}
impl Into<rc::SolAddMode> for &HAddMode {
    fn into(self) -> rc::SolAddMode {
        match self {
            HAddMode::Append => rc::SolAddMode::Append,
            HAddMode::Equip => rc::SolAddMode::Equip,
            HAddMode::Insert(i) => rc::SolAddMode::Insert(*i),
            HAddMode::Replace(i) => rc::SolAddMode::Replace(*i),
        }
    }
}
