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
        let drone = self.uad.items.get(item_key).get_drone()?;
        // Check if projection has already been defined
        let projectee_item = self.uad.items.get(projectee_item_key);
        if drone.get_projs().contains(&projectee_item_key) {
            return Err(ProjNotFoundError {
                projector_item_id: drone.get_item_id(),
                projectee_item_id: projectee_item.get_item_id(),
            }
            .into());
        }
        // Check if projectee can receive projections
        if !projectee_item.can_receive_projs() {
            return Err(ItemReceiveProjError {
                item_id: projectee_item.get_item_id(),
                item_kind: projectee_item.get_name(),
            }
            .into());
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

#[derive(thiserror::Error, Debug)]
pub enum AddDroneProjError {
    #[error("{0}")]
    ProjectorNotFound(#[source] ItemFoundError),
    #[error("{0}")]
    ProjectorIsNotDrone(#[from] ItemKindMatchError),
    #[error("{0}")]
    ProjecteeNotFound(#[source] ItemFoundError),
    #[error("{0}")]
    ProjecteeCantTakeProjs(#[from] ItemReceiveProjError),
    #[error("{0}")]
    ProjectionAlreadyExists(#[from] ProjNotFoundError),
}
