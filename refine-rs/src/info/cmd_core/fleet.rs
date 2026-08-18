use crate::{CmdResps, FleetId, FleetIdBr, FleetInfo, FleetInfoMode, err::BrResolveError, shared::OverridableMap};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FleetInfoCmd {
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
    core: FleetInfoCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetInfoCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fleet(mut self, mode: FleetInfoMode) -> Self {
        self.fleet_mode = mode;
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetInfoCmd {
    pub(in crate::info) fn into_ctx_item_br(self, fleet_id: impl Into<FleetIdBr>) -> FleetInfoCmdCtxFleetBr {
        FleetInfoCmdCtxFleetBr {
            fleet_id: fleet_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetInfoCmdCtxFleetBr {
    pub(in crate::info) fn br_resolve(self, resps: &CmdResps) -> Result<FleetInfoCmdCtxFleet, BrResolveError> {
        Ok(FleetInfoCmdCtxFleet {
            fleet_id: resps.resolve_fleet_id(self.fleet_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetInfoCmd {
    pub(crate) fn execute(self, core_fleet: &mut rc::FleetMut) -> FleetInfo {
        FleetInfo::from_core(core_fleet, &OverridableMap::from_default(self.fleet_mode))
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
