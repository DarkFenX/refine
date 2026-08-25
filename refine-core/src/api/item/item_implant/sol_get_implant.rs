use crate::{
    api::{Implant, ImplantMut},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_implant(&self, item_id: &ItemId) -> Result<Implant<'_>, ImplantGetError> {
        let implant_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(implant_uid).dc_implant()?;
        Ok(Implant::new(self, implant_uid))
    }
    pub fn get_implant_mut(&mut self, item_id: &ItemId) -> Result<ImplantMut<'_>, ImplantGetError> {
        let implant_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(implant_uid).dc_implant()?;
        Ok(ImplantMut::new(self, implant_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ImplantGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotImplant(#[from] ItemKindMatchError),
}
