use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ProjFoundError},
    sol::{ItemId, SolarSystem},
};

impl SolarSystem {
    pub fn remove_drone_proj(
        &mut self,
        item_id: &ItemId,
        projectee_item_id: &ItemId,
    ) -> Result<(), RemoveDroneProjError> {
        // Check if projection is defined
        let drone_item = self.uad.items.get_item(item_id)?;
        let drone = drone_item.get_drone()?;
        if !drone.get_projs().contains(projectee_item_id) {
            return Err(ProjFoundError {
                projector_item_id: *item_id,
                projectee_item_id: *projectee_item_id,
            }
            .into());
        };
        // Update services
        let projectee_item = self.uad.items.get_item(projectee_item_id).unwrap();
        self.svc.remove_item_projection(&self.uad, drone_item, projectee_item);
        // Update user data
        self.proj_tracker.unreg_projectee(item_id, projectee_item_id);
        let drone = self.uad.items.get_item_mut(item_id).unwrap().get_drone_mut().unwrap();
        drone.get_projs_mut().remove(projectee_item_id);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveDroneProjError {
    ProjectorNotFound(ItemFoundError),
    ProjectorIsNotDrone(ItemKindMatchError),
    ProjectionNotFound(ProjFoundError),
}
impl std::error::Error for RemoveDroneProjError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ProjectorNotFound(e) => Some(e),
            Self::ProjectorIsNotDrone(e) => Some(e),
            Self::ProjectionNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveDroneProjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ProjectorNotFound(e) => e.fmt(f),
            Self::ProjectorIsNotDrone(e) => e.fmt(f),
            Self::ProjectionNotFound(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for RemoveDroneProjError {
    fn from(error: ItemFoundError) -> Self {
        Self::ProjectorNotFound(error)
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
