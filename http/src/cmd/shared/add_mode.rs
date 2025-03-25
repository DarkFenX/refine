#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cmd) enum HAddMode {
    Append,
    Equip,
    Insert(rc::Idx),
    Replace(rc::Idx),
}
impl From<&HAddMode> for rc::AddMode {
    fn from(h_add_mode: &HAddMode) -> Self {
        match h_add_mode {
            HAddMode::Append => Self::Append,
            HAddMode::Equip => Self::Equip,
            HAddMode::Insert(i) => Self::Insert(*i),
            HAddMode::Replace(i) => Self::Replace(*i),
        }
    }
}
