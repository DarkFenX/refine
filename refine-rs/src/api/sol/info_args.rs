use crate::{FitInfoModes, FleetInfoModes, ItemInfoModes, SolInfoMode, val::ValInfoMode};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct SolInfoArgs {
    #[cfg_attr(feature = "serde", serde(default))]
    pub sol: SolInfoMode = SolInfoMode::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub fleet: FleetInfoModes = FleetInfoModes::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub fit: FitInfoModes = FitInfoModes::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub item: ItemInfoModes = ItemInfoModes::default(),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct ValSolInfoArgs {
    #[cfg_attr(feature = "serde", serde(default))]
    pub validation: ValInfoMode = ValInfoMode::default(),
}
