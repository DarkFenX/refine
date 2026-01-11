use serde::Deserialize;

#[derive(Copy, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cmd) enum HRmMode {
    Remove,
    Free,
}
impl HRmMode {
    pub(in crate::cmd) fn into_core(self) -> rc::RmMode {
        match self {
            Self::Remove => rc::RmMode::Remove,
            Self::Free => rc::RmMode::Free,
        }
    }
}
