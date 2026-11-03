use crate::{
    Implant, ItemId, Sol,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn get_implant(&mut self, item_id: &ItemId) -> Result<Implant<'_>, ImplantGetError> {
        let sol = self.inner_mut();
        let implant_uid = sol.u_data.items.int_id_by_ext_id_err(item_id)?;
        sol.u_data.items.get(implant_uid).dc_implant()?;
        Ok(Implant::new(sol, implant_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ImplantGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotImplant(#[from] ItemKindMatchError),
}
