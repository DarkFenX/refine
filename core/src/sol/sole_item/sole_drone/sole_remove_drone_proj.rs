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
        let item = self.uad.items.get(item_key);
        let drone = item.get_drone()?;
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

#[derive(Debug)]
pub enum RemoveDroneProjError {
    ProjectorNotFound(ItemFoundError),
    ProjectorIsNotDrone(ItemKindMatchError),
    ProjecteeNotFound(ItemFoundError),
    ProjectionNotFound(ProjFoundError),
}
impl std::error::Error for RemoveDroneProjError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ProjectorNotFound(e) => Some(e),
            Self::ProjectorIsNotDrone(e) => Some(e),
            Self::ProjecteeNotFound(e) => Some(e),
            Self::ProjectionNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveDroneProjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ProjectorNotFound(e) => e.fmt(f),
            Self::ProjectorIsNotDrone(e) => e.fmt(f),
            Self::ProjecteeNotFound(e) => e.fmt(f),
            Self::ProjectionNotFound(e) => e.fmt(f),
        }
    }
}
impl From<ItemKindMatchError> for RemoveDroneProjError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ProjectorIsNotDrone(error)
    }
}
impl From<ProjFoundError> for RemoveDroneProjError {
    fn from(error: ProjFoundError) -> Self {
        Self::ProjectionNotFound(error)
    }
}
