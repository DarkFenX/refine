#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HModAggrMode {
    Stack,
    Min(rc::EBuffId),
    Max(rc::EBuffId),
}
impl From<&rc::ModAggrMode> for HModAggrMode {
    fn from(core_aggr_mode: &rc::ModAggrMode) -> Self {
        match core_aggr_mode {
            rc::ModAggrMode::Stack => Self::Stack,
            rc::ModAggrMode::Min(key) => Self::Min(*key),
            rc::ModAggrMode::Max(key) => Self::Max(*key),
        }
    }
}
