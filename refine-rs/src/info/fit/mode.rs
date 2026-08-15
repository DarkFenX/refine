use std::collections::HashMap;

use crate::{FitId, FitIdBackref};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
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
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
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
}
