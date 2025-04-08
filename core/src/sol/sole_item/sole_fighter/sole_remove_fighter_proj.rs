use itertools::Itertools;

use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ProjFoundError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_fighter_proj(
        &mut self,
        item_id: &ItemId,
        projectee_item_id: &ItemId,
    ) -> Result<(), RemoveFighterProjError> {
        let item_key = self
            .uad
            .items
            .key_by_id_err(item_id)
            .map_err(RemoveFighterProjError::ProjectorNotFound)?;
        let projectee_item_key = self
            .uad
            .items
            .key_by_id_err(projectee_item_id)
            .map_err(RemoveFighterProjError::ProjecteeNotFound)?;
        self.remove_fighter_proj_internal(item_key, projectee_item_key)
    }
    pub(in crate::sol) fn remove_fighter_proj_internal(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
    ) -> Result<(), RemoveFighterProjError> {
        // Check if projection is defined
        let fighter = self.uad.items.get(item_key).get_fighter()?;
        let projectee_item = self.uad.items.get(projectee_item_key);
        if !fighter.get_projs().contains(&projectee_item_key) {
            return Err(ProjFoundError {
                projector_item_id: fighter.get_item_id(),
                projectee_item_id: projectee_item.get_item_id(),
            }
            .into());
        };
        let autocharge_keys = fighter.get_autocharges().values().copied().collect_vec();
        for autocharge_key in autocharge_keys {
            // Update services for autocharge
            self.remove_item_key_projection_from_svc(autocharge_key, projectee_item_key);
            // Update user data for autocharge
            self.proj_tracker.unreg_projectee(&autocharge_key, &projectee_item_key);
            let autocharge = self.uad.items.get_mut(autocharge_key).get_autocharge_mut().unwrap();
            autocharge.get_projs_mut().remove(&projectee_item_key);
        }
        // Update services for fighter
        self.remove_item_key_projection_from_svc(item_key, projectee_item_key);
        // Update user data for fighter
        self.proj_tracker.unreg_projectee(&item_key, &projectee_item_key);
        let fighter = self.uad.items.get_mut(item_key).get_fighter_mut().unwrap();
        fighter.get_projs_mut().remove(&projectee_item_key);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveFighterProjError {
    ProjectorNotFound(ItemFoundError),
    ProjectorIsNotFighter(ItemKindMatchError),
    ProjecteeNotFound(ItemFoundError),
    ProjectionNotFound(ProjFoundError),
}
impl std::error::Error for RemoveFighterProjError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ProjectorNotFound(e) => Some(e),
            Self::ProjectorIsNotFighter(e) => Some(e),
            Self::ProjecteeNotFound(e) => Some(e),
            Self::ProjectionNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveFighterProjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ProjectorNotFound(e) => e.fmt(f),
            Self::ProjectorIsNotFighter(e) => e.fmt(f),
            Self::ProjecteeNotFound(e) => e.fmt(f),
            Self::ProjectionNotFound(e) => e.fmt(f),
        }
    }
}
impl From<ItemKindMatchError> for RemoveFighterProjError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ProjectorIsNotFighter(error)
    }
}
impl From<ProjFoundError> for RemoveFighterProjError {
    fn from(error: ProjFoundError) -> Self {
        Self::ProjectionNotFound(error)
    }
}
