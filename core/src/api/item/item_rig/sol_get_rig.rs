use crate::{
    api::{Rig, RigMut},
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn get_rig(&self, item_id: &ItemId) -> Result<Rig<'_>, GetRigError> {
        let rig_key = self.u_data.items.key_by_id_err(item_id)?;
        self.u_data.items.get(rig_key).dc_rig()?;
        Ok(Rig::new(self, rig_key))
    }
    pub fn get_rig_mut(&mut self, item_id: &ItemId) -> Result<RigMut<'_>, GetRigError> {
        let rig_key = self.u_data.items.key_by_id_err(item_id)?;
        self.u_data.items.get(rig_key).dc_rig()?;
        Ok(RigMut::new(self, rig_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetRigError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotRig(#[from] ItemKindMatchError),
}
