use crate::{
    api::{Implant, ImplantMut},
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn get_implant(&self, item_id: &ItemId) -> Result<Implant<'_>, GetImplantError> {
        let implant_key = self.u_data.items.key_by_id_err(item_id)?;
        self.u_data.items.get(implant_key).dc_implant()?;
        Ok(Implant::new(self, implant_key))
    }
    pub fn get_implant_mut(&mut self, item_id: &ItemId) -> Result<ImplantMut<'_>, GetImplantError> {
        let implant_key = self.u_data.items.key_by_id_err(item_id)?;
        self.u_data.items.get(implant_key).dc_implant()?;
        Ok(ImplantMut::new(self, implant_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetImplantError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotImplant(#[from] ItemKindMatchError),
}
