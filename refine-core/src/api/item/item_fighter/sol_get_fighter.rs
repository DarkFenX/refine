use crate::{
    Fighter, ItemId, Sol,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn get_fighter(&mut self, item_id: &ItemId) -> Result<Fighter<'_>, FighterGetError> {
        let core = self.inner_mut();
        let fighter_uid = core.u_data.items.int_id_by_ext_id_err(item_id)?;
        core.u_data.items.get(fighter_uid).dc_fighter()?;
        Ok(Fighter::new(core, fighter_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FighterGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotFighter(#[from] ItemKindMatchError),
}
