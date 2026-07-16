use crate::{
    AddedFleetIdResp, FitId,
    cmd::inner::{AddFleetError, ICmdFleetAddFCtxRIds},
};

#[derive(Default)]
pub struct AddFleetCmd {
    inner: ICmdFleetAddFCtxRIds = ICmdFleetAddFCtxRIds { .. },
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AddFleetCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fit_ids(mut self, fit_ids: impl Iterator<Item = FitId>) -> Self {
        self.inner.fit_ids.clear();
        self.inner.fit_ids.extend(fit_ids);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AddFleetCmd {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<AddedFleetIdResp, AddFleetError> {
        self.inner.execute(core_sol)
    }
}
