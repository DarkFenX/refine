use crate::cmd::basic::{ChangeFleetError, CmdFleetChangeICtxRIds};

#[derive(Default)]
pub struct ChangeFleetCmd {
    basic: CmdFleetChangeICtxRIds,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChangeFleetCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_add_fit_ids(mut self, add_fit_ids: impl ExactSizeIterator<Item = rc::FitId>) -> Self {
        self.basic.add_fit_ids.clear();
        self.basic.add_fit_ids.extend(add_fit_ids);
        self
    }
    pub fn with_rm_fit_ids(mut self, rm_fit_ids: impl ExactSizeIterator<Item = rc::FitId>) -> Self {
        self.basic.rm_fit_ids.clear();
        self.basic.rm_fit_ids.extend(rm_fit_ids);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChangeFleetCmd {
    pub(crate) fn execute(&self, core_fleet: &mut rc::FleetMut) -> Result<(), ChangeFleetError> {
        self.basic.execute(core_fleet)
    }
}
