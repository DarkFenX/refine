use crate::{
    ItemId, Sol, Stance,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn get_stance(&mut self, item_id: &ItemId) -> Result<Stance<'_>, StanceGetError> {
        let core = self.inner_mut();
        let stance_uid = core.u_data.items.int_id_by_ext_id_err(item_id)?;
        core.u_data.items.get(stance_uid).dc_stance()?;
        Ok(Stance::new(core, stance_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StanceGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotStance(#[from] ItemKindMatchError),
}
