use crate::{
    api::FleetMut,
    def::ItemId,
    err::basic::{ItemFoundError, ItemReceiveProjError},
    ud::UItemKey,
};

impl<'a> FleetMut<'a> {
    pub(super) fn get_stat_applied_projectee_key(
        &self,
        projectee_item_id: &ItemId,
    ) -> Result<UItemKey, FleetStatAppliedError> {
        let projectee_key = self.sol.u_data.items.key_by_id_err(projectee_item_id)?;
        let projectee_u_item = self.sol.u_data.items.get(projectee_key);
        if projectee_u_item.get_direct_physics().is_none() {
            return Err(ItemReceiveProjError {
                item_id: projectee_u_item.get_item_id(),
                item_kind: projectee_u_item.get_name(),
            }
            .into());
        }
        Ok(projectee_key)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FleetStatAppliedError {
    #[error("{0}")]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ProjecteeCantTakeProjs(#[from] ItemReceiveProjError),
}
