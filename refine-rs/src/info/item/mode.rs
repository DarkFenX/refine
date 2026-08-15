use std::collections::HashMap;

use crate::{ItemId, ItemIdBackref};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
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
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemInfoModesInt {
    pub(crate) fn from_pub_modes_regular(pub_modes: ItemInfoModes) -> Self {
        Self {
            default: pub_modes.default,
            overrides: pub_modes.overrides.into_iter().collect(),
        }
    }
}
