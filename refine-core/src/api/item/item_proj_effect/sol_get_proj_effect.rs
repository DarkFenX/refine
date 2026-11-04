use crate::{
    ItemId, ProjEffect, Sol,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn get_proj_effect(&mut self, item_id: &ItemId) -> Result<ProjEffect<'_>, ProjEffectGetError> {
        let core = self.inner_mut();
        let proj_effect_uid = core.u_data.items.int_id_by_ext_id_err(item_id)?;
        core.u_data.items.get(proj_effect_uid).dc_proj_effect()?;
        Ok(ProjEffect::new(core, proj_effect_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProjEffectGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotProjEffect(#[from] ItemKindMatchError),
}
