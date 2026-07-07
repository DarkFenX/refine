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
    pub(in crate::cmd) fn into_core(self) -> rc::MvMode {
        match self {
            Self::Shift(i) => rc::MvMode::Shift(rc::Index::from_usize(i)),
            Self::Swap(i) => rc::MvMode::Swap(rc::Index::from_usize(i)),
        }
    }
}

impl HRmMode {
    pub(in crate::cmd) fn into_core(self) -> rc::RmMode {
        match self {
            Self::Remove => rc::RmMode::Remove,
            Self::Free => rc::RmMode::Free,
        }
    }
}
