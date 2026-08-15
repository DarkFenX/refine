use std::collections::HashMap;

use crate::{FitId, FitIdBackref, FleetId, FleetIdBackref, ItemId, ItemIdBackref};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Modes - sol
////////////////////////////////////////////////////////////////////////////////////////////////////
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Modes - fleet
////////////////////////////////////////////////////////////////////////////////////////////////////
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
#[derive(Clone)]
pub struct FleetInfoModes {
    pub default: FleetInfoMode = FleetInfoMode::default(),
    pub overrides: Vec<(FleetId, FleetInfoMode)> = Vec::new(),
}
impl FleetInfoModes {
    fn into_internal(self) -> FleetInfoModesInt {
        FleetInfoModesInt {
            default: self.default,
            overrides: self.overrides.into_iter().collect(),
        }
    }
}
const impl Default for FleetInfoModes {
    fn default() -> Self {
        Self { .. }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Clone)]
pub struct FleetInfoModesBackref {
    pub default: FleetInfoMode = FleetInfoMode::default(),
    pub overrides: Vec<(FleetIdBackref, FleetInfoMode)> = Vec::new(),
}
const impl Default for FleetInfoModesBackref {
    fn default() -> Self {
        Self { .. }
    }
}

pub(crate) struct FleetInfoModesInt {
    default: FleetInfoMode,
    overrides: HashMap<FleetId, FleetInfoMode>,
}
impl FleetInfoModesInt {
    pub(in crate::info) fn get(&self, id: &FleetId) -> FleetInfoMode {
        match self.overrides.get(id) {
            Some(mode) => *mode,
            None => self.default,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Modes - fit
////////////////////////////////////////////////////////////////////////////////////////////////////
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
#[derive(Clone)]
pub struct FitInfoModes {
    pub default: FitInfoMode = FitInfoMode::default(),
    pub overrides: Vec<(FitId, FitInfoMode)> = Vec::new(),
}
impl FitInfoModes {
    pub(crate) fn into_internal(self) -> FitInfoModesInt {
        FitInfoModesInt {
            default: self.default,
            overrides: self.overrides.into_iter().collect(),
        }
    }
}
const impl Default for FitInfoModes {
    fn default() -> Self {
        Self { .. }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Clone)]
pub struct FitInfoModesBackref {
    pub default: FitInfoMode = FitInfoMode::default(),
    pub overrides: Vec<(FitIdBackref, FitInfoMode)> = Vec::new(),
}
const impl Default for FitInfoModesBackref {
    fn default() -> Self {
        Self { .. }
    }
}

pub(crate) struct FitInfoModesInt {
    default: FitInfoMode,
    overrides: HashMap<FitId, FitInfoMode>,
}
impl FitInfoModesInt {
    pub(in crate::info) fn get(&self, id: &FitId) -> FitInfoMode {
        match self.overrides.get(id) {
            Some(mode) => *mode,
            None => self.default,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Modes - item
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone)]
pub enum ItemInfoMode {
    Id,
    Partial,
    Full,
}
const impl Default for ItemInfoMode {
    fn default() -> Self {
        Self::Partial
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Clone)]
pub struct ItemInfoModes {
    pub default: ItemInfoMode = ItemInfoMode::default(),
    pub overrides: Vec<(ItemId, ItemInfoMode)> = Vec::new(),
}
impl ItemInfoModes {
    pub(crate) fn into_internal(self) -> ItemInfoModesInt {
        ItemInfoModesInt {
            default: self.default,
            overrides: self.overrides.into_iter().collect(),
        }
    }
}
const impl Default for ItemInfoModes {
    fn default() -> Self {
        Self { .. }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Clone)]
pub struct ItemInfoModesBackref {
    pub default: ItemInfoMode = ItemInfoMode::default(),
    pub overrides: Vec<(ItemIdBackref, ItemInfoMode)> = Vec::new(),
}
const impl Default for ItemInfoModesBackref {
    fn default() -> Self {
        Self { .. }
    }
}

pub(crate) struct ItemInfoModesInt {
    default: ItemInfoMode,
    overrides: HashMap<ItemId, ItemInfoMode>,
}
impl ItemInfoModesInt {
    pub(in crate::info) fn get(&self, id: &ItemId) -> ItemInfoMode {
        match self.overrides.get(id) {
            Some(mode) => *mode,
            None => self.default,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Arguments - sol
////////////////////////////////////////////////////////////////////////////////////////////////////
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Arguments - fleet
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct FleetInfoArgs {
    #[cfg_attr(feature = "serde", serde(default))]
    pub fleet: FleetInfoMode = FleetInfoMode::default(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Arguments - fit
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FitInfoArgs {
    #[cfg_attr(feature = "serde", serde(default))]
    pub fit: FitInfoMode = FitInfoMode::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub item: ItemInfoModes = ItemInfoModes::default(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Arguments - item
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct ItemInfoArgs {
    #[cfg_attr(feature = "serde", serde(default))]
    pub item: ItemInfoModes = ItemInfoModes::default(),
}
