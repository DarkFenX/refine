use crate::{
    ItemId, Rig, SolarSystem,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolarSystem {
    pub fn get_rig(&mut self, item_id: &ItemId) -> Result<Rig<'_>, RigGetError> {
        let rig_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(rig_uid).dc_rig()?;
        Ok(Rig::new(self, rig_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RigGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotRig(#[from] ItemKindMatchError),
}
