use serde::Deserialize;

#[derive(Copy, Clone, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HSolInfoMode {
    Id,
    #[default]
    Full,
}

#[derive(Copy, Clone, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HFitInfoMode {
    Id,
    #[default]
    Full,
}

#[derive(Copy, Clone, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HItemInfoMode {
    Id,
    #[default]
    Partial,
    Full,
}

#[derive(Copy, Clone, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HFleetInfoMode {
    #[default]
    Id,
    Full,
}

#[derive(Copy, Clone, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HValidInfoMode {
    Simple,
    #[default]
    Detailed,
}
