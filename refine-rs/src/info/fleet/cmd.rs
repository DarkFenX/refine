use crate::{FleetInfo, FleetInfoMode, info::FleetInfoModesInt};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct FleetInfoCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    fleet: FleetInfoMode = FleetInfoMode::default(),
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
        FleetInfo::from_core(core_fleet, &FleetInfoModesInt::from_pub_mode(self.fleet))
    }
}
