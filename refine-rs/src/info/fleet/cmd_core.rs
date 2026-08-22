use crate::{
    CmdResps, FitId, FitIdBr, FitInfoMode, FleetId, FleetIdBr, FleetInfo, FleetInfoMode, ItemId, ItemIdBr,
    ItemInfoMode,
    err::BrResolveError,
    shared::{OvrdCompact, OvrdMapLight},
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FleetInfoCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    fit_mode: OvrdMapLight<FitId, FitInfoMode>,
    #[cfg_attr(feature = "serde", serde(default))]
    item_mode: OvrdMapLight<ItemId, ItemInfoMode>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: FleetInfoCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FleetInfoCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    fit_mode: OvrdCompact<FitIdBr, FitInfoMode>,
    #[cfg_attr(feature = "serde", serde(default))]
    item_mode: OvrdCompact<ItemIdBr, ItemInfoMode>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: FleetInfoCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
struct FleetInfoCmdShared {
    #[cfg_attr(feature = "serde", serde(default))]
    fleet_mode: FleetInfoMode,
}

// Extra context commands
#[derive(Clone)]
pub struct FleetInfoCmdCtxFleet {
    fleet_id: FleetId,
    core: FleetInfoCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct FleetInfoCmdCtxFleetBr {
    fleet_id: FleetIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FleetInfoCmdBr,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetInfoCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fleet(mut self, mode: FleetInfoMode) -> Self {
        self.shared.fleet_mode = mode;
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

impl FleetInfoCmdBr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fleet(mut self, mode: FleetInfoMode) -> Self {
        self.shared.fleet_mode = mode;
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
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetInfoCmdBr {
    pub(in crate::info) fn into_ctx_fleet_br(self, fleet_id: impl Into<FleetIdBr>) -> FleetInfoCmdCtxFleetBr {
        FleetInfoCmdCtxFleetBr {
            fleet_id: fleet_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetInfoCmdBr {
    fn br_resolve(self, resps: &CmdResps) -> Result<FleetInfoCmd, BrResolveError> {
        Ok(FleetInfoCmd {
            fit_mode: OvrdMapLight::from_compact_with_br_resolution(self.fit_mode, resps)?,
            item_mode: OvrdMapLight::from_compact_with_br_resolution(self.item_mode, resps)?,
            shared: self.shared,
        })
    }
}

impl FleetInfoCmdCtxFleetBr {
    pub(in crate::info) fn br_resolve(self, resps: &CmdResps) -> Result<FleetInfoCmdCtxFleet, BrResolveError> {
        Ok(FleetInfoCmdCtxFleet {
            fleet_id: resps.resolve_fleet_id(self.fleet_id)?,
            core: self.core.br_resolve(resps)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetInfoCmd {
    pub(crate) fn execute(self, core_fleet: &mut rc::FleetMut) -> FleetInfo {
        FleetInfo::from_core(
            core_fleet,
            &OvrdMapLight::from_default(self.shared.fleet_mode),
            &self.fit_mode,
            &self.item_mode,
        )
    }
}

impl FleetInfoCmdCtxFleet {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<FleetInfo, FleetGetFleetInfoError> {
        let mut core_fleet = core_sol.get_fleet_mut(&self.fleet_id)?;
        Ok(self.core.execute(&mut core_fleet))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FleetGetFleetInfoError {
    #[error(transparent)]
    FleetGet(#[from] rc::err::GetFleetError),
}
