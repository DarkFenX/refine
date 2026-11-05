use crate::{
    FwEffect, ItemId, Sol,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn get_fw_effect(&mut self, item_id: &ItemId) -> Result<FwEffect<'_>, FwEffectGetError> {
        let sol = self.inner_mut();
        let fw_effect_uid = sol.u_data.items.int_id_by_ext_id_err(item_id)?;
        sol.u_data.items.get(fw_effect_uid).dc_fw_effect()?;
        Ok(FwEffect::new(sol, fw_effect_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FwEffectGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotFwEffect(#[from] ItemKindMatchError),
}
