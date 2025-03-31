use crate::{cmd::remove_item, util::HExecError};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HRemoveItemCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: remove_item::HRemoveItemCmd,
}
impl HRemoveItemCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), HExecError> {
        self.item_cmd.execute(core_sol, self.item_id)
    }
}
