#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cmd) enum HRmMode {
    Remove,
    Free,
}
impl Into<rc::SolOrdRmMode> for &HRmMode {
    fn into(self) -> rc::SolOrdRmMode {
        match self {
            HRmMode::Remove => rc::SolOrdRmMode::Remove,
            HRmMode::Free => rc::SolOrdRmMode::Free,
        }
    }
}
