use serde::Deserialize;

#[derive(Copy, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cmd) enum HAddMode {
    Append,
    Equip,
    Insert(usize),
    Replace(usize),
}
impl HAddMode {
    pub(in crate::cmd) fn into_core(self) -> rc::AddMode {
        match self {
            Self::Append => rc::AddMode::Append,
            Self::Equip => rc::AddMode::Equip,
            Self::Insert(i) => rc::AddMode::Insert(rc::Index::from_usize(i)),
            Self::Replace(i) => rc::AddMode::Replace(rc::Index::from_usize(i)),
        }
    }
}
