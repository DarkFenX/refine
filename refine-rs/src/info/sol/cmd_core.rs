use crate::{
    CmdResps, FitId, FitIdBr, FitInfoMode, FleetId, FleetIdBr, FleetInfoMode, ItemId, ItemIdBr, ItemInfoMode, SolInfo,
    SolInfoExt, SolInfoMode, SolarSystemId, SrcAlias,
    shared::{CmdResidue, OvrdCompact, OvrdMapLight},
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct SolInfoCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    fleet_mode: OvrdMapLight<FleetId, FleetInfoMode>,
    #[cfg_attr(feature = "serde", serde(default))]
    fit_mode: OvrdMapLight<FitId, FitInfoMode>,
    #[cfg_attr(feature = "serde", serde(default))]
    item_mode: OvrdMapLight<ItemId, ItemInfoMode>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: SolInfoCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct SolInfoCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    fleet_mode: OvrdCompact<FleetIdBr, FleetInfoMode>,
    #[cfg_attr(feature = "serde", serde(default))]
    fit_mode: OvrdCompact<FitIdBr, FitInfoMode>,
    #[cfg_attr(feature = "serde", serde(default))]
    item_mode: OvrdCompact<ItemIdBr, ItemInfoMode>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: SolInfoCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
struct SolInfoCmdShared {
    #[cfg_attr(feature = "serde", serde(default))]
    sol_mode: SolInfoMode,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolInfoCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_sol(mut self, mode: SolInfoMode) -> Self {
        self.shared.sol_mode = mode;
        self
    }
    pub fn with_fleet_default(mut self, mode: FleetInfoMode) -> Self {
        self.fleet_mode.set_default(mode);
        self
    }
    pub fn with_fleet_overrides(mut self, mode: FleetInfoMode, fleet_ids: impl Iterator<Item = FleetId>) -> Self {
        self.fleet_mode.add_overrides(mode, fleet_ids);
        self
    }
    pub fn with_fit_default(mut self, mode: FitInfoMode) -> Self {
        self.fit_mode.set_default(mode);
        self
    }
    pub fn with_fit_overrides(mut self, mode: FitInfoMode, fit_ids: impl Iterator<Item = FitId>) -> Self {
        self.fit_mode.add_overrides(mode, fit_ids);
        self
    }
    pub fn with_item_default(mut self, mode: ItemInfoMode) -> Self {
        self.item_mode.set_default(mode);
        self
    }
    pub fn with_item_overrides(mut self, mode: ItemInfoMode, item_ids: impl Iterator<Item = ItemId>) -> Self {
        self.item_mode.add_overrides(mode, item_ids);
        self
    }
}

impl SolInfoCmdBr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_sol(mut self, mode: SolInfoMode) -> Self {
        self.shared.sol_mode = mode;
        self
    }
    pub fn with_fleet_default(mut self, mode: FleetInfoMode) -> Self {
        self.fleet_mode.set_default(mode);
        self
    }
    pub fn with_fleet_overrides(mut self, mode: FleetInfoMode, fleet_ids: impl Iterator<Item = FleetIdBr>) -> Self {
        self.fleet_mode.add_overrides(mode, fleet_ids);
        self
    }
    pub fn with_fit_default(mut self, mode: FitInfoMode) -> Self {
        self.fit_mode.set_default(mode);
        self
    }
    pub fn with_fit_overrides(mut self, mode: FitInfoMode, fit_ids: impl Iterator<Item = FitIdBr>) -> Self {
        self.fit_mode.add_overrides(mode, fit_ids);
        self
    }
    pub fn with_item_default(mut self, mode: ItemInfoMode) -> Self {
        self.item_mode.set_default(mode);
        self
    }
    pub fn with_item_overrides(mut self, mode: ItemInfoMode, item_ids: impl Iterator<Item = ItemIdBr>) -> Self {
        self.item_mode.add_overrides(mode, item_ids);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolInfoCmdBr {
    pub(crate) fn br_resolve(self, resps: &CmdResps) -> SolInfoCmd {
        SolInfoCmd {
            fleet_mode: OvrdMapLight::from_compact_with_br_resolution(self.fleet_mode, resps),
            fit_mode: OvrdMapLight::from_compact_with_br_resolution(self.fit_mode, resps),
            item_mode: OvrdMapLight::from_compact_with_br_resolution(self.item_mode, resps),
            shared: self.shared,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolInfoCmdBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::None
    }
}

impl SolInfoCmd {
    pub(crate) fn execute(self, sol_id: SolarSystemId, src_alias: SrcAlias, core_sol: &mut rc::SolarSystem) -> SolInfo {
        SolInfo::from_ids_and_core(
            sol_id,
            src_alias,
            core_sol,
            self.shared.sol_mode,
            &self.fleet_mode,
            &self.fit_mode,
            &self.item_mode,
        )
    }
    pub(crate) fn execute_into_info_ext(self, core_sol: &mut rc::SolarSystem) -> Option<SolInfoExt> {
        SolInfoExt::try_from_core(
            core_sol,
            self.shared.sol_mode,
            &self.fleet_mode,
            &self.fit_mode,
            &self.item_mode,
        )
    }
}
