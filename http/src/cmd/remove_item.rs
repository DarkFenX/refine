use crate::{cmd::shared::HRmMode, util::HExecError};

#[derive(serde::Deserialize)]
pub(crate) struct HRemoveItemCmd {
    rm_mode: Option<HRmMode>,
}
impl HRemoveItemCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem, item_id: rc::SolItemId) -> Result<(), HExecError> {
        match core_sol.remove_item(&item_id, self.rm_mode.as_ref().unwrap_or(&HRmMode::Free).into()) {
            Ok(()) => Ok(()),
            Err(core_err) => Err(match core_err {
                rc::err::RemoveItemError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                rc::err::RemoveItemError::UnremovableAutocharge(e) => HExecError::UnremovableAutocharge(e),
            }),
        }
    }
}
impl Default for HRemoveItemCmd {
    fn default() -> Self {
        Self { rm_mode: None }
    }
}
