use crate::{
    AddedFitIdResp,
    cmd::inner::{AddFitError, ICmdFitAddFCtxRIds},
};

#[derive(Default)]
pub struct AddFitCmd {
    inner: ICmdFitAddFCtxRIds = ICmdFitAddFCtxRIds { .. },
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AddFitCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fleet_id(mut self, fleet_id: rc::FleetId) -> Self {
        self.inner.fleet_id = Some(fleet_id);
        self
    }
    pub fn with_sec_status(mut self, sec_status: rc::FitSecStatus) -> Self {
        self.inner.shared.sec_status = Some(sec_status);
        self
    }
    pub fn with_rah_incoming_dps(mut self, rah_incoming_dps: rc::DpsProfile) -> Self {
        self.inner.shared.rah_incoming_dps = Some(rah_incoming_dps);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AddFitCmd {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<AddedFitIdResp, AddFitError> {
        self.inner.execute(core_sol)
    }
}
