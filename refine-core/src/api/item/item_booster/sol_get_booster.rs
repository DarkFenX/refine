use crate::{
    Booster, ItemId, Sol,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn get_booster(&mut self, item_id: &ItemId) -> Result<Booster<'_>, BoosterGetError> {
        let core = self.inner_mut();
        let booster_uid = core.u_data.items.int_id_by_ext_id_err(item_id)?;
        core.u_data.items.get(booster_uid).dc_booster()?;
        Ok(Booster::new(core, booster_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BoosterGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotBooster(#[from] ItemKindMatchError),
}
