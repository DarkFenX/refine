use serde::Deserialize;

use crate::{cmd::basic::HItemRemoveCmdICtx, util::HExecError};

#[derive(Default, Deserialize)]
pub(crate) struct HItemRemoveCmd {
    #[serde(flatten)]
    basic: HItemRemoveCmdICtx,
}
impl HItemRemoveCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem, item_id: &rc::ItemId) -> Result<(), HExecError> {
        self.basic.execute(core_sol, item_id)
    }
}
