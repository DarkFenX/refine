use crate::{
    Charge, ItemId, Sol,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn get_charge(&mut self, item_id: &ItemId) -> Result<Charge<'_>, ChargeGetError> {
        let core = self.inner_mut();
        let charge_uid = core.u_data.items.int_id_by_ext_id_err(item_id)?;
        core.u_data.items.get(charge_uid).dc_charge()?;
        Ok(Charge::new(core, charge_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ChargeGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotCharge(#[from] ItemKindMatchError),
}
