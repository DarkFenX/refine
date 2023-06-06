#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CModAggrMode {
    Stack,
    Min(rc::ReeInt),
    Max(rc::ReeInt),
}
impl From<&rc::consts::ModAggrMode> for CModAggrMode {
    fn from(mod_aggr_mode: &rc::consts::ModAggrMode) -> Self {
        match mod_aggr_mode {
            rc::consts::ModAggrMode::Stack => Self::Stack,
            rc::consts::ModAggrMode::Min(key) => Self::Min(*key),
            rc::consts::ModAggrMode::Max(key) => Self::Max(*key),
        }
    }
}
impl Into<rc::consts::ModAggrMode> for &CModAggrMode {
    fn into(self) -> rc::consts::ModAggrMode {
        match self {
            CModAggrMode::Stack => rc::consts::ModAggrMode::Stack,
            CModAggrMode::Min(key) => rc::consts::ModAggrMode::Min(*key),
            CModAggrMode::Max(key) => rc::consts::ModAggrMode::Max(*key),
        }
    }
}
