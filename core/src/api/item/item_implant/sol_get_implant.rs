use crate::{
    api::{Implant, ImplantMut},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_implant(&self, item_id: &ItemId) -> Result<Implant<'_>, GetImplantError> {
        let implant_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(implant_uid).dc_implant()?;
        Ok(Implant::new(self, implant_uid))
    }
    pub fn get_implant_mut(&mut self, item_id: &ItemId) -> Result<ImplantMut<'_>, GetImplantError> {
        let implant_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(implant_uid).dc_implant()?;
        Ok(ImplantMut::new(self, implant_uid))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetImplantError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotImplant(#[from] ItemKindMatchError),
}
