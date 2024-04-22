#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HSolInfoMode {
    Id,
    Full,
}
impl From<Option<HSolInfoMode>> for HSolInfoMode {
    fn from(mode_opt: Option<HSolInfoMode>) -> Self {
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

#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HFleetInfoMode {
    Id,
    Full,
}
impl From<Option<HFleetInfoMode>> for HFleetInfoMode {
    fn from(mode_opt: Option<HFleetInfoMode>) -> Self {
        mode_opt.unwrap_or_else(|| Self::Id)
    }
}
