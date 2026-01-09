use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cmd) enum HRmMode {
    Remove,
    Free,
}
impl From<&HRmMode> for rc::RmMode {
    fn from(h_rm_mode: &HRmMode) -> Self {
        match h_rm_mode {
            HRmMode::Remove => Self::Remove,
            HRmMode::Free => Self::Free,
        }
    }
}
