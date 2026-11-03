use crate::{
    Implant, ItemId, SolarSystem,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolarSystem {
    pub fn get_implant(&mut self, item_id: &ItemId) -> Result<Implant<'_>, ImplantGetError> {
        let implant_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(implant_uid).dc_implant()?;
        Ok(Implant::new(self, implant_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ImplantGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotImplant(#[from] ItemKindMatchError),
}
