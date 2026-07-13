use crate::cmd::{
    inner::{CmdFleetCreateFCtxRIds, CreateFleetError},
    shared::CreatedFleetIdResp,
};

#[derive(Default)]
pub struct CreateFleetCmd {
    inner: CmdFleetCreateFCtxRIds,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CreateFleetCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fit_ids(mut self, fit_ids: impl ExactSizeIterator<Item = rc::FitId>) -> Self {
        self.inner.fit_ids.clear();
        self.inner.fit_ids.extend(fit_ids);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CreateFleetCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<CreatedFleetIdResp, CreateFleetError> {
        self.inner.execute(core_sol)
    }
}
