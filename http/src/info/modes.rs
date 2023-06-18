#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HSsInfoMode {
    Id,
    Full,
}
impl From<Option<HSsInfoMode>> for HSsInfoMode {
    fn from(mode_opt: Option<HSsInfoMode>) -> Self {
        match mode_opt {
            Some(v) => v,
            None => Self::Full,
        }
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
        match mode_opt {
            Some(v) => v,
            None => Self::Full,
        }
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
        match mode_opt {
            Some(v) => v,
            None => Self::Partial,
        }
    }
}
