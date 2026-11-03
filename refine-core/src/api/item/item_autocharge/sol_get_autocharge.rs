use crate::{
    Autocharge, ItemId, SolarSystem,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolarSystem {
    pub fn get_autocharge(&mut self, item_id: &ItemId) -> Result<Autocharge<'_>, AutochargeGetError> {
        let autocharge_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(autocharge_uid).dc_autocharge()?;
        Ok(Autocharge::new(self, autocharge_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AutochargeGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotAutocharge(#[from] ItemKindMatchError),
}
