use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ItemReceiveProjError, ProjNotFoundError},
    sol::{AttrVal, ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn add_drone_proj(
        &mut self,
        item_id: &ItemId,
        projectee_item_id: &ItemId,
        range: Option<AttrVal>,
    ) -> Result<(), AddDroneProjError> {
        let item_key = self
            .uad
            .items
            .key_by_id_err(item_id)
            .map_err(AddDroneProjError::ProjectorNotFound)?;
        let projectee_item_key = self
            .uad
            .items
            .key_by_id_err(projectee_item_id)
            .map_err(AddDroneProjError::ProjecteeNotFound)?;
        self.add_drone_proj_internal(item_key, projectee_item_key, range)
    }
    pub(in crate::sol) fn add_drone_proj_internal(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
        range: Option<AttrVal>,
    ) -> Result<(), AddDroneProjError> {
        // Check projector
        let drone = self
            .uad
            .items
            .get(item_key)
            .get_drone()
            .map_err(AddDroneProjError::ProjectorIsNotDrone)?;
        // Check if projection has already been defined
        let projectee_item = self.uad.items.get(projectee_item_key);
        if drone.get_projs().contains(&projectee_item_key) {
            return Err(AddDroneProjError::ProjectionAlreadyExists(ProjNotFoundError {
                projector_item_id: drone.get_item_id(),
                projectee_item_id: projectee_item.get_item_id(),
            }));
        }
        // Check if projectee can receive projections
        if !projectee_item.can_receive_projs() {
            return Err(AddDroneProjError::ProjecteeCantTakeProjs(ItemReceiveProjError {
                item_id: projectee_item.get_item_id(),
                item_kind: projectee_item.get_name(),
            }));
        }
        // Update user data
        let drone = self.uad.items.get_mut(item_key).get_drone_mut().unwrap();
        drone.get_projs_mut().add(projectee_item_key, range);
        self.proj_tracker.reg_projectee(item_key, projectee_item_key);
        // Update services
        self.add_item_key_projection_to_svc(item_key, projectee_item_key, range);
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
