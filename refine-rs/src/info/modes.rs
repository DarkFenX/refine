#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone)]
pub enum SrcInfoMode {
    Partial,
    Full,
}
const impl Default for SrcInfoMode {
    fn default() -> Self {
        Self::Full
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone)]
pub enum SolInfoMode {
    Id,
    Full,
}
const impl Default for SolInfoMode {
    fn default() -> Self {
        Self::Full
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone)]
pub enum FleetInfoMode {
    Id,
    Full,
}
const impl Default for FleetInfoMode {
    fn default() -> Self {
        Self::Id
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone)]
pub enum FitInfoMode {
    Id,
    Full,
}
const impl Default for FitInfoMode {
    fn default() -> Self {
        Self::Full
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone)]
pub enum ItemInfoMode {
    Id,
    Partial,
    Full,
}
const impl Default for ItemInfoMode {
    fn default() -> Self {
        Self::Full
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone)]
pub enum ValInfoMode {
    Simple,
    Detailed,
}
const impl Default for ValInfoMode {
    fn default() -> Self {
        Self::Detailed
    }
}
