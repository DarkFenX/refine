#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cmd) enum HRmMode {
    Remove,
    Free,
}
impl Into<rc::SolRmMode> for &HRmMode {
    fn into(self) -> rc::SolRmMode {
        match self {
            HRmMode::Remove => rc::SolRmMode::Remove,
            HRmMode::Free => rc::SolRmMode::Free,
        }
    }
}
