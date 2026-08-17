use crate::{FleetId, FleetIdBr, FleetInfo, FleetInfoMode, info::InfoModesInt};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct FleetInfoCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    fleet: FleetInfoMode,
}

// Extra context commands
#[derive(Copy, Clone)]
pub struct FleetInfoCmdCtxFleet {
    fleet_id: FleetId,
    core: FleetInfoCmd,
}
#[derive(Copy, Clone)]
pub struct FleetInfoCmdCtxFleetBr {
    fleet_id: FleetIdBr,
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
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetInfoCmd {
    pub(crate) fn execute(self, core_fleet: &mut rc::FleetMut) -> FleetInfo {
        FleetInfo::from_core(core_fleet, &InfoModesInt::from_pub_mode(self.fleet))
    }
}
