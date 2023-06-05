#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum ModAggrMode {
    Stack,
    Min(rc::ReeInt),
    Max(rc::ReeInt),
}
impl From<&rc::consts::ModAggrMode> for ModAggrMode {
    fn from(value: &rc::consts::ModAggrMode) -> Self {
        match value {
            rc::consts::ModAggrMode::Stack => Self::Stack,
            rc::consts::ModAggrMode::Min(key) => Self::Min(*key),
            rc::consts::ModAggrMode::Max(key) => Self::Max(*key),
        }
    }
}
impl Into<rc::consts::ModAggrMode> for &ModAggrMode {
    fn into(self) -> rc::consts::ModAggrMode {
        match self {
            ModAggrMode::Stack => rc::consts::ModAggrMode::Stack,
            ModAggrMode::Min(key) => rc::consts::ModAggrMode::Min(*key),
            ModAggrMode::Max(key) => rc::consts::ModAggrMode::Max(*key),
        }
    }
}
