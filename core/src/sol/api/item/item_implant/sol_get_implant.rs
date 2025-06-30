use crate::{
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{
        SolarSystem,
        api::{Implant, ImplantMut},
    },
};

impl SolarSystem {
    pub fn get_implant(&self, item_id: &ItemId) -> Result<Implant<'_>, GetImplantError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.uad.items.get(item_key).get_implant()?;
        Ok(Implant::new(self, item_key))
    }
    pub fn get_implant_mut(&mut self, item_id: &ItemId) -> Result<ImplantMut<'_>, GetImplantError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.uad.items.get(item_key).get_implant()?;
        Ok(ImplantMut::new(self, item_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetImplantError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotImplant(#[from] ItemKindMatchError),
}
