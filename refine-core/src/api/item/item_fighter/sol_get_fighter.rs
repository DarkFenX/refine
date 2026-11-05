use crate::{
    Fighter, ItemId, Sol,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn get_fighter(&mut self, item_id: &ItemId) -> Result<Fighter<'_>, FighterGetError> {
        let sol = self.inner_mut();
        let fighter_uid = sol.u_data.items.int_id_by_ext_id_err(item_id)?;
        sol.u_data.items.get(fighter_uid).dc_fighter()?;
        Ok(Fighter::new(sol, fighter_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FighterGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotFighter(#[from] ItemKindMatchError),
}
