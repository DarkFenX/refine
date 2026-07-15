use serde::Deserialize;

#[derive(Copy, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cmd) enum HAddMode {
    Append,
    Equip,
    Insert(usize),
    Replace(usize),
}

#[derive(Copy, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cmd) enum HMvMode {
    Shift(usize),
    Swap(usize),
}

#[derive(Copy, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cmd) enum HRmMode {
    Remove,
    Free,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
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

impl HMvMode {
    pub(in crate::cmd) fn into_core(self) -> rc::MoveMode {
        match self {
            Self::Shift(i) => rc::MoveMode::Shift(rc::Index::from_usize(i)),
            Self::Swap(i) => rc::MoveMode::Swap(rc::Index::from_usize(i)),
        }
    }
}

impl HRmMode {
    pub(in crate::cmd) fn into_core(self) -> rc::RemoveMode {
        match self {
            Self::Remove => rc::RemoveMode::Remove,
            Self::Free => rc::RemoveMode::Free,
        }
    }
}
