#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HSolInfoMode {
    Id,
    Full,
}
impl Default for HSolInfoMode {
    fn default() -> Self {
        Self::Full
    }
}

#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HFitInfoMode {
    Id,
    Full,
}
impl Default for HFitInfoMode {
    fn default() -> Self {
        Self::Full
    }
}

#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HItemInfoMode {
    Id,
    Partial,
    Full,
}
impl Default for HItemInfoMode {
    fn default() -> Self {
        Self::Partial
    }
}

#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HFleetInfoMode {
    Id,
    Full,
}
impl Default for HFleetInfoMode {
    fn default() -> Self {
        Self::Id
    }
}

#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HValidInfoMode {
    Simple,
    Detailed,
}
impl Default for HValidInfoMode {
    fn default() -> Self {
        Self::Detailed
    }
}
