use serde::Deserialize;

use crate::{cmd::shared::HRmMode, util::HExecError};

#[derive(Default, Deserialize)]
pub(crate) struct HRemoveItemCmd {
    rm_mode: Option<HRmMode>,
}
impl HRemoveItemCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem, item_id: rc::ItemId) -> Result<(), HExecError> {
        let core_item = core_sol.get_item_mut(&item_id).map_err(|error| match error {
            rc::err::GetItemError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
        })?;
        core_item
            .remove(self.rm_mode.as_ref().unwrap_or(&HRmMode::Free).into())
            .map_err(|error| match error {
                rc::err::RemoveItemError::UnremovableAutocharge => HExecError::UnremovableAutocharge,
            })?;
        Ok(())
    }
}
