#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CModAggrMode {
    Stack,
    Min(rc::EBuffId),
    Max(rc::EBuffId),
}
impl From<&rc::ModAggrMode> for CModAggrMode {
    fn from(mod_aggr_mode: &rc::ModAggrMode) -> Self {
        match mod_aggr_mode {
            rc::ModAggrMode::Stack => Self::Stack,
            rc::ModAggrMode::Min(key) => Self::Min(*key),
            rc::ModAggrMode::Max(key) => Self::Max(*key),
        }
    }
}
impl Into<rc::ModAggrMode> for &CModAggrMode {
    fn into(self) -> rc::ModAggrMode {
        match self {
            CModAggrMode::Stack => rc::ModAggrMode::Stack,
            CModAggrMode::Min(key) => rc::ModAggrMode::Min(*key),
            CModAggrMode::Max(key) => rc::ModAggrMode::Max(*key),
        }
    }
}
