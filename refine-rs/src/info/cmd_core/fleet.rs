use crate::{
    CmdResps, FleetId, FleetIdBr, FleetInfo, FleetInfoMode, err::BrResolveError, info::InfoModes, shared::BrResolvable,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FleetInfoCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    fleet: FleetInfoMode,
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
        self.fleet = mode;
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetInfoCmdCtxFleetBr {
    pub(in crate::info) fn br_resolve(self, resps: &CmdResps) -> Result<FleetInfoCmdCtxFleet, BrResolveError> {
        Ok(FleetInfoCmdCtxFleet {
            fleet_id: self.fleet_id.br_resolve(resps)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetInfoCmd {
    pub(crate) fn execute(self, core_fleet: &mut rc::FleetMut) -> FleetInfo {
        FleetInfo::from_core(core_fleet, &InfoModes::from_simple(self.fleet))
    }
}
