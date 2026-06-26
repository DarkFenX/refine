use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{cmd::remove_item, util::HExecError};

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HRemoveItemCmd {
    #[serde_as(as = "DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: remove_item::HRemoveItemCmd,
}
impl HRemoveItemCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), HExecError> {
        self.item_cmd.execute(core_sol, self.item_id)
    }
}
