use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ItemReceiveProjError, ProjNotFoundError},
    sol::{AttrVal, ItemId, SolarSystem},
};

impl SolarSystem {
    pub fn add_drone_proj(
        &mut self,
        item_id: &ItemId,
        projectee_item_id: ItemId,
        range: Option<AttrVal>,
    ) -> Result<(), AddDroneProjError> {
        // Check projector
        let drone = self
            .uad
            .items
            .get_item(item_id)
            .map_err(AddDroneProjError::ProjectorNotFound)?
            .get_drone()
            .map_err(AddDroneProjError::ProjectorIsNotDrone)?;
        // Check if projection has already been defined
        if drone.get_projs().contains(&projectee_item_id) {
            return Err(AddDroneProjError::ProjectionAlreadyExists(ProjNotFoundError::new(
                *item_id,
                projectee_item_id,
            )));
        }
        // Check if projectee can receive projections
        let projectee_item = self
            .uad
            .items
            .get_item(&projectee_item_id)
            .map_err(AddDroneProjError::ProjecteeNotFound)?;
        if !projectee_item.can_receive_projs() {
            return Err(AddDroneProjError::ProjecteeCantTakeProjs(ItemReceiveProjError::new(
                projectee_item_id,
                projectee_item.get_name(),
            )));
        }
        // Update user data
        let drone = self.uad.items.get_item_mut(item_id).unwrap().get_drone_mut().unwrap();
        drone.get_projs_mut().add(projectee_item_id, range);
        self.proj_tracker.reg_projectee(*item_id, projectee_item_id);
        // Update services
        self.add_item_id_projection_to_svc(item_id, &projectee_item_id, range);
        Ok(())
    }
}

#[derive(Debug)]
pub enum AddDroneProjError {
    ProjectorNotFound(ItemFoundError),
    ProjectorIsNotDrone(ItemKindMatchError),
    ProjecteeNotFound(ItemFoundError),
    ProjecteeCantTakeProjs(ItemReceiveProjError),
    ProjectionAlreadyExists(ProjNotFoundError),
}
impl std::error::Error for AddDroneProjError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ProjectorNotFound(e) => Some(e),
            Self::ProjectorIsNotDrone(e) => Some(e),
            Self::ProjecteeNotFound(e) => Some(e),
            Self::ProjecteeCantTakeProjs(e) => Some(e),
            Self::ProjectionAlreadyExists(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddDroneProjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ProjectorNotFound(e) => e.fmt(f),
            Self::ProjectorIsNotDrone(e) => e.fmt(f),
            Self::ProjecteeNotFound(e) => e.fmt(f),
            Self::ProjecteeCantTakeProjs(e) => e.fmt(f),
            Self::ProjectionAlreadyExists(e) => e.fmt(f),
        }
    }
}
impl From<ProjNotFoundError> for AddDroneProjError {
    fn from(error: ProjNotFoundError) -> Self {
        Self::ProjectionAlreadyExists(error)
    }
}
