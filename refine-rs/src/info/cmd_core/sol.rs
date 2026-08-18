use crate::{
    CmdResps, FitId, FitIdBr, FitInfoMode, FleetId, FleetIdBr, FleetInfoMode, ItemId, ItemIdBr, ItemInfoMode, SolInfo,
    SolInfoExt, SolInfoMode, SolarSystemId, SrcAlias,
    err::BrResolveError,
    info::{InfoModes, InfoModesCompact},
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct SolInfoCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    fleet_mode: InfoModes<FleetInfoMode, FleetId>,
    #[cfg_attr(feature = "serde", serde(default))]
    fit_mode: InfoModes<FitInfoMode, FitId>,
    #[cfg_attr(feature = "serde", serde(default))]
    item_mode: InfoModes<ItemInfoMode, ItemId>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: SolInfoCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct SolInfoCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    fleet_mode: InfoModesCompact<FleetInfoMode, FleetIdBr>,
    #[cfg_attr(feature = "serde", serde(default))]
    fit_mode: InfoModesCompact<FitInfoMode, FitIdBr>,
    #[cfg_attr(feature = "serde", serde(default))]
    item_mode: InfoModesCompact<ItemInfoMode, ItemIdBr>,
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
        self.fleet_mode.default = mode;
        self
    }
    pub fn with_fleet_overrides(mut self, mode: FleetInfoMode, fleet_ids: impl Iterator<Item = FleetId>) -> Self {
        for fleet_id in fleet_ids {
            self.fleet_mode.overrides.insert(fleet_id, mode);
        }
        self
    }
    pub fn with_fit_default(mut self, mode: FitInfoMode) -> Self {
        self.fit_mode.default = mode;
        self
    }
    pub fn with_fit_overrides(mut self, mode: FitInfoMode, fit_ids: impl Iterator<Item = FitId>) -> Self {
        for fit_id in fit_ids {
            self.fit_mode.overrides.insert(fit_id, mode);
        }
        self
    }
    pub fn with_item_default(mut self, mode: ItemInfoMode) -> Self {
        self.item_mode.default = mode;
        self
    }
    pub fn with_item_overrides(mut self, mode: ItemInfoMode, item_ids: impl Iterator<Item = ItemId>) -> Self {
        for item_id in item_ids {
            self.item_mode.overrides.insert(item_id, mode);
        }
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
        self.fleet_mode.default = mode;
        self
    }
    pub fn with_fleet_overrides(mut self, mode: FleetInfoMode, fleet_ids: impl Iterator<Item = FleetIdBr>) -> Self {
        self.fleet_mode.overrides.push((mode, fleet_ids.collect()));
        self
    }
    pub fn with_fit_default(mut self, mode: FitInfoMode) -> Self {
        self.fit_mode.default = mode;
        self
    }
    pub fn with_fit_overrides(mut self, mode: FitInfoMode, fit_ids: impl Iterator<Item = FitIdBr>) -> Self {
        self.fit_mode.overrides.push((mode, fit_ids.collect()));
        self
    }
    pub fn with_item_default(mut self, mode: ItemInfoMode) -> Self {
        self.item_mode.default = mode;
        self
    }
    pub fn with_item_overrides(mut self, mode: ItemInfoMode, item_ids: impl Iterator<Item = ItemIdBr>) -> Self {
        self.item_mode.overrides.push((mode, item_ids.collect()));
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolInfoCmd {
    pub(in crate::info) fn into_br(self) -> SolInfoCmdBr {
        SolInfoCmdBr {
            fleet_mode: self.fleet_mode.into_compact_br(),
            fit_mode: self.fit_mode.into_compact_br(),
            item_mode: self.item_mode.into_compact_br(),
            shared: self.shared,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolInfoCmdBr {
    pub(crate) fn br_resolve(self, resps: &CmdResps) -> Result<SolInfoCmd, BrResolveError> {
        Ok(SolInfoCmd {
            fleet_mode: InfoModes::from_compact_br(self.fleet_mode, resps)?,
            fit_mode: InfoModes::from_compact_br(self.fit_mode, resps)?,
            item_mode: InfoModes::from_compact_br(self.item_mode, resps)?,
            shared: self.shared,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
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
