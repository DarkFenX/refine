use std::collections::HashMap;

use crate::{FleetId, FleetIdBackref};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
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
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
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
}
