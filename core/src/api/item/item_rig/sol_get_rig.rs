use crate::{
    api::{Rig, RigMut},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_rig(&self, item_id: &ItemId) -> Result<Rig<'_>, GetRigError> {
        let rig_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(rig_uid).dc_rig()?;
        Ok(Rig::new(self, rig_uid))
    }
    pub fn get_rig_mut(&mut self, item_id: &ItemId) -> Result<RigMut<'_>, GetRigError> {
        let rig_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(rig_uid).dc_rig()?;
        Ok(RigMut::new(self, rig_uid))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetRigError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotRig(#[from] ItemKindMatchError),
}
