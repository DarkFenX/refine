use crate::{
    err::basic::{ItemFoundError, ProjFoundError},
    sol::{ItemId, ItemKey, SolarSystem, api::DroneMut},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_remove_drone_proj(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined
        let uad_drone = self.uad.items.get(item_key).get_drone().unwrap();
        let projectee_uad_item = self.uad.items.get(projectee_item_key);
        if !uad_drone.get_projs().contains(&projectee_item_key) {
            return Err(ProjFoundError {
                projector_item_id: uad_drone.get_item_id(),
                projectee_item_id: projectee_uad_item.get_item_id(),
            });
        };
        // Update services
        self.svc
            .remove_item_projection(&self.uad, item_key, projectee_item_key, projectee_uad_item);
        // Update user data
        self.proj_tracker.unreg_projectee(&item_key, &projectee_item_key);
        let uad_drone = self.uad.items.get_mut(item_key).get_drone_mut().unwrap();
        uad_drone.get_projs_mut().remove(&projectee_item_key);
        Ok(())
    }
}

impl<'a> DroneMut<'a> {
    pub fn remove_proj(self, projectee_item_id: &ItemId) -> Result<Self, RemoveDroneProjError> {
        let projectee_item_key = self.sol.uad.items.key_by_id_err(projectee_item_id)?;
        self.sol.internal_remove_drone_proj(self.key, projectee_item_key)?;
        Ok(self)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveDroneProjError {
    #[error("{0}")]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ProjectionNotFound(#[from] ProjFoundError),
}
