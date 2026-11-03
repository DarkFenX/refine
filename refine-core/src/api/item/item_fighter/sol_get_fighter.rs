use crate::{
    api::Fighter,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_fighter(&mut self, item_id: &ItemId) -> Result<Fighter<'_>, FighterGetError> {
        let fighter_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(fighter_uid).dc_fighter()?;
        Ok(Fighter::new(self, fighter_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FighterGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotFighter(#[from] ItemKindMatchError),
}
