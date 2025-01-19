#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cmd) enum HAddMode {
    Append,
    Equip,
    Insert(rc::Idx),
    Replace(rc::Idx),
}
impl Into<rc::SolModAddMode> for &HAddMode {
    fn into(self) -> rc::SolModAddMode {
        match self {
            HAddMode::Append => rc::SolModAddMode::Append,
            HAddMode::Equip => rc::SolModAddMode::Equip,
            HAddMode::Insert(i) => rc::SolModAddMode::Insert(*i),
            HAddMode::Replace(i) => rc::SolModAddMode::Replace(*i),
        }
    }
}
