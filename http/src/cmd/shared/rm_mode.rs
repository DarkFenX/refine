#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cmd) enum HRmMode {
    Remove,
    Free,
}
impl Into<rc::SolModRmMode> for &HRmMode {
    fn into(self) -> rc::SolModRmMode {
        match self {
            HRmMode::Remove => rc::SolModRmMode::Remove,
            HRmMode::Free => rc::SolModRmMode::Free,
        }
    }
}
