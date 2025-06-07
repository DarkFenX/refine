use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{
        ItemId, SolarSystem,
        api::{Rig, RigMut},
    },
};

impl SolarSystem {
    pub fn get_rig(&self, item_id: &ItemId) -> Result<Rig<'_>, GetRigError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.uad.items.get(item_key).get_rig()?;
        Ok(Rig::new(self, item_key))
    }
    pub fn get_rig_mut(&mut self, item_id: &ItemId) -> Result<RigMut<'_>, GetRigError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.uad.items.get(item_key).get_rig()?;
        Ok(RigMut::new(self, item_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetRigError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotRig(#[from] ItemKindMatchError),
}
