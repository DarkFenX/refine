use std::collections::HashMap;

use crate::{FitId, FitIdBackref, FleetId, FleetIdBackref, ItemId, ItemIdBackref};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Solar system
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
// Fleet
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
    pub(crate) fn from_pub_mode(pub_mode: FleetInfoMode) -> Self {
        Self {
            default: pub_mode,
            overrides: HashMap::new(),
        }
    }
    pub(crate) fn from_pub_modes_regular(pub_modes: FleetInfoModes) -> Self {
        Self {
            default: pub_modes.default,
            overrides: pub_modes.overrides.into_iter().collect(),
        }
    }
    pub(in crate::info) fn get(&self, id: &FleetId) -> FleetInfoMode {
        match self.overrides.get(id) {
            Some(mode) => *mode,
            None => self.default,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fit
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
    pub(crate) fn from_pub_mode(pub_mode: FitInfoMode) -> Self {
        Self {
            default: pub_mode,
            overrides: HashMap::new(),
        }
    }
    pub(crate) fn from_pub_modes_regular(pub_modes: FitInfoModes) -> Self {
        Self {
            default: pub_modes.default,
            overrides: pub_modes.overrides.into_iter().collect(),
        }
    }
    pub(in crate::info) fn get(&self, id: &FitId) -> FitInfoMode {
        match self.overrides.get(id) {
            Some(mode) => *mode,
            None => self.default,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Item
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
    pub(crate) fn from_pub_modes_regular(pub_modes: ItemInfoModes) -> Self {
        Self {
            default: pub_modes.default,
            overrides: pub_modes.overrides.into_iter().collect(),
        }
    }
    pub(in crate::info) fn get(&self, id: &ItemId) -> ItemInfoMode {
        match self.overrides.get(id) {
            Some(mode) => *mode,
            None => self.default,
        }
    }
}
