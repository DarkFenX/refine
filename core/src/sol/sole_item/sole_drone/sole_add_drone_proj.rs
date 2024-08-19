use crate::{
    defs::{AttrVal, SolItemId},
    err::basic::{ItemFoundError, ItemKindMatchError, ItemReceiveProjError, ProjNotFoundError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn add_drone_proj(
        &mut self,
        item_id: &SolItemId,
        projectee_item_id: SolItemId,
        range: Option<AttrVal>,
    ) -> Result<(), AddDroneProjError> {
        // Check projector
        let drone = self
            .items
            .get_item(item_id)
            .map_err(|e| AddDroneProjError::ProjectorNotFound(e))?
            .get_drone()
            .map_err(|e| AddDroneProjError::ProjectorIsNotDrone(e))?;
        // Check if projection has already been defined
        if drone.get_projs().contains(&projectee_item_id) {
            return Err(AddDroneProjError::ProjectionAlreadyExists(ProjNotFoundError::new(
                *item_id,
                projectee_item_id,
            )));
        }
        // Check if projectee can receive projections
        let projectee_item = self
            .items
            .get_item(&projectee_item_id)
            .map_err(|e| AddDroneProjError::ProjecteeNotFound(e))?;
        if !projectee_item.can_receive_projs() {
            return Err(AddDroneProjError::ProjecteeCantTakeProjs(ItemReceiveProjError::new(
                projectee_item_id,
                projectee_item.get_name(),
            )));
        }
        // Update skeleton
        let drone = self.items.get_item_mut(item_id).unwrap().get_drone_mut().unwrap();
        drone.get_projs_mut().add(projectee_item_id, range);
        self.proj_tracker.reg_projectee(*item_id, projectee_item_id);
        // Update services
        self.add_item_id_projection_to_svcs(item_id, &projectee_item_id, range);
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
