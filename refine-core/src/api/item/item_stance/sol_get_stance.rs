use crate::{
    ItemId, SolarSystem, Stance,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolarSystem {
    pub fn get_stance(&mut self, item_id: &ItemId) -> Result<Stance<'_>, StanceGetError> {
        let stance_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(stance_uid).dc_stance()?;
        Ok(Stance::new(self, stance_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StanceGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotStance(#[from] ItemKindMatchError),
}
