////////////////////////////////////////////////////////////////////////////////////////////////////
// Basic modes
////////////////////////////////////////////////////////////////////////////////////////////////////
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
        Self::Partial
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Modes combined into parameters for specific "endpoints"
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct SrcInfoModes {
    #[cfg_attr(feature = "serde", serde(default))]
    pub src: SrcInfoMode = SrcInfoMode::default(),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct SolInfoModes {
    #[cfg_attr(feature = "serde", serde(default))]
    pub sol: SolInfoMode = SolInfoMode::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub fleet: FleetInfoMode = FleetInfoMode::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub fit: FitInfoMode = FitInfoMode::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub item: ItemInfoMode = ItemInfoMode::default(),
}
impl SolInfoModes {
    pub(crate) fn get_fleet_modes(&self) -> FleetInfoModes {
        FleetInfoModes { fleet: self.fleet }
    }
    pub(crate) fn get_fit_modes(&self) -> FitInfoModes {
        FitInfoModes {
            fit: self.fit,
            item: self.item,
        }
    }
    pub(crate) fn get_item_modes(&self) -> ItemInfoModes {
        ItemInfoModes { item: self.item }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct FleetInfoModes {
    #[cfg_attr(feature = "serde", serde(default))]
    pub fleet: FleetInfoMode = FleetInfoMode::default(),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct FitInfoModes {
    #[cfg_attr(feature = "serde", serde(default))]
    pub fit: FitInfoMode = FitInfoMode::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub item: ItemInfoMode = ItemInfoMode::default(),
}
impl FitInfoModes {
    pub(crate) fn get_item_modes(&self) -> ItemInfoModes {
        ItemInfoModes { item: self.item }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct ItemInfoModes {
    #[cfg_attr(feature = "serde", serde(default))]
    pub item: ItemInfoMode = ItemInfoMode::default(),
}
