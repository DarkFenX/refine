////////////////////////////////////////////////////////////////////////////////////////////////////
// Info modes
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Info arguments for specific entities
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct SolInfoArgs {
    #[cfg_attr(feature = "serde", serde(default))]
    pub sol: SolInfoMode = SolInfoMode::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub fleet: FleetInfoMode = FleetInfoMode::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub fit: FitInfoMode = FitInfoMode::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub item: ItemInfoMode = ItemInfoMode::default(),
}
impl SolInfoArgs {
    pub(crate) fn get_fleet_args(&self) -> FleetInfoArgs {
        FleetInfoArgs { fleet: self.fleet }
    }
    pub(crate) fn get_fit_args(&self) -> FitInfoArgs {
        FitInfoArgs {
            fit: self.fit,
            item: self.item,
        }
    }
    pub(crate) fn get_item_args(&self) -> ItemInfoArgs {
        ItemInfoArgs { item: self.item }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct FleetInfoArgs {
    #[cfg_attr(feature = "serde", serde(default))]
    pub fleet: FleetInfoMode = FleetInfoMode::default(),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct FitInfoArgs {
    #[cfg_attr(feature = "serde", serde(default))]
    pub fit: FitInfoMode = FitInfoMode::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub item: ItemInfoMode = ItemInfoMode::default(),
}
impl FitInfoArgs {
    pub(crate) fn get_item_args(&self) -> ItemInfoArgs {
        ItemInfoArgs { item: self.item }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct ItemInfoArgs {
    #[cfg_attr(feature = "serde", serde(default))]
    pub item: ItemInfoMode = ItemInfoMode::default(),
}
