#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Default)]
pub enum SrcInfoMode {
    Partial,
    #[default]
    Full,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Default)]
pub enum SolInfoMode {
    Id,
    #[default]
    Full,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Default)]
pub enum FleetInfoMode {
    #[default]
    Id,
    Full,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Default)]
pub enum FitInfoMode {
    Id,
    #[default]
    Full,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Default)]
pub enum ItemInfoMode {
    Id,
    Partial,
    #[default]
    Full,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Default)]
pub enum ValInfoMode {
    Simple,
    #[default]
    Detailed,
}
