use crate::{
    defs::{AttrVal, SolItemId},
    err::basic::{ItemFoundError, ItemKindMatchError, ProjFoundError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn change_drone_proj(
        &mut self,
        item_id: &SolItemId,
        projectee_item_id: &SolItemId,
        range: Option<AttrVal>,
    ) -> Result<(), ChangeDroneProjError> {
        // Check if projection is defined before changing it
        let drone = self.items.get_item_mut(item_id)?.get_drone_mut()?;
        let old_range = match drone.get_projs().get(projectee_item_id) {
            Some(old_range) => *old_range,
            None => return Err(ProjFoundError::new(*item_id, *projectee_item_id).into()),
        };
        // Do nothing if ranges are equal
        if range == old_range {
            return Ok(());
        }
        // Update skeleton
        drone.get_projs_mut().add(*projectee_item_id, range);
        // Update services
        self.change_item_id_projection_range_in_svcs(item_id, projectee_item_id, range);
        Ok(())
    }
}

#[derive(Debug)]
pub enum ChangeDroneProjError {
    ProjectorNotFound(ItemFoundError),
    ProjectorIsNotDrone(ItemKindMatchError),
    ProjectionNotFound(ProjFoundError),
}
impl std::error::Error for ChangeDroneProjError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ProjectorNotFound(e) => Some(e),
            Self::ProjectorIsNotDrone(e) => Some(e),
            Self::ProjectionNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for ChangeDroneProjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ProjectorNotFound(e) => e.fmt(f),
            Self::ProjectorIsNotDrone(e) => e.fmt(f),
            Self::ProjectionNotFound(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for ChangeDroneProjError {
    fn from(error: ItemFoundError) -> Self {
        Self::ProjectorNotFound(error)
    }
}
impl From<ItemKindMatchError> for ChangeDroneProjError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ProjectorIsNotDrone(error)
    }
}
impl From<ProjFoundError> for ChangeDroneProjError {
    fn from(error: ProjFoundError) -> Self {
        Self::ProjectionNotFound(error)
    }
}
