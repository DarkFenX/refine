use crate::{
    FwEffect, ItemId, SolarSystem,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolarSystem {
    pub fn get_fw_effect(&mut self, item_id: &ItemId) -> Result<FwEffect<'_>, FwEffectGetError> {
        let fw_effect_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(fw_effect_uid).dc_fw_effect()?;
        Ok(FwEffect::new(self, fw_effect_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FwEffectGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotFwEffect(#[from] ItemKindMatchError),
}
