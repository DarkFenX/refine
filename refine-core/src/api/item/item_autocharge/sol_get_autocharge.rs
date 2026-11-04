use crate::{
    Autocharge, ItemId, Sol,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn get_autocharge(&mut self, item_id: &ItemId) -> Result<Autocharge<'_>, AutochargeGetError> {
        let core = self.inner_mut();
        let autocharge_uid = core.u_data.items.int_id_by_ext_id_err(item_id)?;
        core.u_data.items.get(autocharge_uid).dc_autocharge()?;
        Ok(Autocharge::new(core, autocharge_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AutochargeGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotAutocharge(#[from] ItemKindMatchError),
}
