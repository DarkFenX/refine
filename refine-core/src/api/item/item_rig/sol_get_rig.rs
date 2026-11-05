use crate::{
    ItemId, Rig, Sol,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn get_rig(&mut self, item_id: &ItemId) -> Result<Rig<'_>, RigGetError> {
        let sol = self.inner_mut();
        let rig_uid = sol.u_data.items.int_id_by_ext_id_err(item_id)?;
        sol.u_data.items.get(rig_uid).dc_rig()?;
        Ok(Rig::new(sol, rig_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RigGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotRig(#[from] ItemKindMatchError),
}
