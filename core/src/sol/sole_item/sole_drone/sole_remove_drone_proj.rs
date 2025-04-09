use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ProjFoundError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_drone_proj(
        &mut self,
        item_id: &ItemId,
        projectee_item_id: &ItemId,
    ) -> Result<(), RemoveDroneProjError> {
        let item_key = self
            .uad
            .items
            .key_by_id_err(item_id)
            .map_err(RemoveDroneProjError::ProjectorNotFound)?;
        let projectee_item_key = self
            .uad
            .items
            .key_by_id_err(projectee_item_id)
            .map_err(RemoveDroneProjError::ProjecteeNotFound)?;
        self.remove_drone_proj_internal(item_key, projectee_item_key)
    }
    pub(in crate::sol) fn remove_drone_proj_internal(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
    ) -> Result<(), RemoveDroneProjError> {
        // Check if projection is defined
        let drone = self.uad.items.get(item_key).get_drone()?;
        let projectee_item = self.uad.items.get(projectee_item_key);
        if !drone.get_projs().contains(&projectee_item_key) {
            return Err(ProjFoundError {
                projector_item_id: drone.get_item_id(),
                projectee_item_id: projectee_item.get_item_id(),
            }
            .into());
        };
        // Update services
        self.svc
            .remove_item_projection(&self.uad, item_key, projectee_item_key, projectee_item);
        // Update user data
        self.proj_tracker.unreg_projectee(&item_key, &projectee_item_key);
        let drone = self.uad.items.get_mut(item_key).get_drone_mut().unwrap();
        drone.get_projs_mut().remove(&projectee_item_key);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveDroneProjError {
    #[error("{0}")]
    ProjectorNotFound(#[source] ItemFoundError),
    #[error("{0}")]
    ProjectorIsNotDrone(#[from] ItemKindMatchError),
    #[error("{0}")]
    ProjecteeNotFound(#[source] ItemFoundError),
    #[error("{0}")]
    ProjectionNotFound(#[from] ProjFoundError),
}
