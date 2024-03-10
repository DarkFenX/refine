#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HSsInfoMode {
    Id,
    Full,
}
impl From<Option<HSsInfoMode>> for HSsInfoMode {
    fn from(mode_opt: Option<HSsInfoMode>) -> Self {
        mode_opt.unwrap_or_else(|| Self::Full)
    }
}

#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HFitInfoMode {
    Id,
    Full,
}
impl From<Option<HFitInfoMode>> for HFitInfoMode {
    fn from(mode_opt: Option<HFitInfoMode>) -> Self {
        mode_opt.unwrap_or_else(|| Self::Full)
    }
}

#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HItemInfoMode {
    Id,
    Partial,
    Full,
}
impl From<Option<HItemInfoMode>> for HItemInfoMode {
    fn from(mode_opt: Option<HItemInfoMode>) -> Self {
        mode_opt.unwrap_or_else(|| Self::Partial)
    }
}
