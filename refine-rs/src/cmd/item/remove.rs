use crate::cmd::{BasicRemoveItemError, basic::CmdItemRemoveICtx};

#[derive(Default)]
pub struct RemoveItemCmd {
    basic: CmdItemRemoveICtx,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RemoveItemCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_rm_mode(mut self, rm_mode: rc::RmMode) -> Self {
        self.basic.rm_mode = Some(rm_mode);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RemoveItemCmd {
    pub(crate) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<(), BasicRemoveItemError> {
        self.basic.execute(core_sol, item_id)
    }
}
