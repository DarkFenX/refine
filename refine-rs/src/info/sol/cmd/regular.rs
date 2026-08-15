use crate::{
    FitId, FitInfoMode, FitInfoModes, FleetId, FleetInfoMode, FleetInfoModes, ItemId, ItemInfoMode, ItemInfoModes,
    SolInfo, SolInfoExt, SolInfoMode, SolarSystemId, SrcAlias,
    info::{FitInfoModesInt, FleetInfoModesInt, ItemInfoModesInt},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct SolInfoCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    sol: SolInfoMode = SolInfoMode::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    fleet: FleetInfoModes = FleetInfoModes::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    fit: FitInfoModes = FitInfoModes::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    item: ItemInfoModes = ItemInfoModes::default(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolInfoCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_sol(mut self, mode: SolInfoMode) -> Self {
        self.sol = mode;
        self
    }
    pub fn with_fleet_default(mut self, mode: FleetInfoMode) -> Self {
        self.fleet.default = mode;
        self
    }
    pub fn with_fleet_overrides(mut self, mode: FleetInfoMode, item_ids: impl Iterator<Item = FleetId>) -> Self {
        self.fleet.overrides.push((mode, item_ids.collect()));
        self
    }
    pub fn with_fit_default(mut self, mode: FitInfoMode) -> Self {
        self.fit.default = mode;
        self
    }
    pub fn with_fit_overrides(mut self, mode: FitInfoMode, item_ids: impl Iterator<Item = FitId>) -> Self {
        self.fit.overrides.push((mode, item_ids.collect()));
        self
    }
    pub fn with_item_default(mut self, mode: ItemInfoMode) -> Self {
        self.item.default = mode;
        self
    }
    pub fn with_item_overrides(mut self, mode: ItemInfoMode, item_ids: impl Iterator<Item = ItemId>) -> Self {
        self.item.overrides.push((mode, item_ids.collect()));
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolInfoCmd {
    pub(crate) fn execute_into_info(
        self,
        sol_id: SolarSystemId,
        src_alias: SrcAlias,
        core_sol: &mut rc::SolarSystem,
    ) -> SolInfo {
        SolInfo::from_ids_and_core(
            sol_id,
            src_alias,
            core_sol,
            self.sol,
            &FleetInfoModesInt::from_pub_modes_regular(self.fleet),
            &FitInfoModesInt::from_pub_modes_regular(self.fit),
            &ItemInfoModesInt::from_pub_modes_regular(self.item),
        )
    }

    pub(crate) fn execute_into_info_ext(self, core_sol: &mut rc::SolarSystem) -> Option<SolInfoExt> {
        SolInfoExt::try_from_core(
            core_sol,
            self.sol,
            &FleetInfoModesInt::from_pub_modes_regular(self.fleet),
            &FitInfoModesInt::from_pub_modes_regular(self.fit),
            &ItemInfoModesInt::from_pub_modes_regular(self.item),
        )
    }
}
