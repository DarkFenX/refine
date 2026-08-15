use crate::{FitInfoMode, ItemInfoModes, val::ValInfoMode};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FitInfoArgs {
    #[cfg_attr(feature = "serde", serde(default))]
    pub fit: FitInfoMode = FitInfoMode::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub item: ItemInfoModes = ItemInfoModes::default(),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct ValFitInfoArgs {
    #[cfg_attr(feature = "serde", serde(default))]
    pub validation: ValInfoMode = ValInfoMode::default(),
}
