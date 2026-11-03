use crate::{
    ItemId, Sol, SwEffect,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn get_sw_effect(&mut self, item_id: &ItemId) -> Result<SwEffect<'_>, SwEffectGetError> {
        let sol = self.inner_mut();
        let sw_effect_uid = sol.u_data.items.int_id_by_ext_id_err(item_id)?;
        sol.u_data.items.get(sw_effect_uid).dc_sw_effect()?;
        Ok(SwEffect::new(sol, sw_effect_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SwEffectGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotSwEffect(#[from] ItemKindMatchError),
}
