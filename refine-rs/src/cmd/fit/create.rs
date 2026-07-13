use crate::cmd::{CreateFitError, CreatedFitIdResp, basic::CmdFitCreateFCtxRIds};

#[derive(Default)]
pub struct CreateFitCmd {
    basic: CmdFitCreateFCtxRIds,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CreateFitCmd {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn with_fleet_id(mut self, fleet_id: rc::FleetId) -> Self {
        self.basic.fleet_id = Some(fleet_id);
        self
    }
    pub fn with_sec_status(mut self, sec_status: rc::FitSecStatus) -> Self {
        self.basic.shared.sec_status = Some(sec_status);
        self
    }
    pub fn with_rah_incoming_dps(mut self, rah_incoming_dps: rc::DpsProfile) -> Self {
        self.basic.shared.rah_incoming_dps = Some(rah_incoming_dps);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CreateFitCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<CreatedFitIdResp, CreateFitError> {
        self.basic.execute(core_sol)
    }
}
