use itertools::Itertools;

use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ProjFoundError},
    sol::{ItemId, SolarSystem},
};

impl SolarSystem {
    pub fn remove_fighter_proj(
        &mut self,
        item_id: &ItemId,
        projectee_item_id: &ItemId,
    ) -> Result<(), RemoveFighterProjError> {
        // Check if projection is defined
        let fighter = self.uad.items.get_item(item_id)?.get_fighter()?;
        if !fighter.get_projs().contains(projectee_item_id) {
            return Err(ProjFoundError {
                projector_item_id: *item_id,
                projectee_item_id: *projectee_item_id,
            }
            .into());
        };
        let autocharge_ids = fighter.get_autocharges().values().copied().collect_vec();
        for autocharge_id in autocharge_ids {
            // Update services for autocharge
            self.remove_item_id_projection_from_svc(&autocharge_id, projectee_item_id);
            // Update user data for autocharge
            self.proj_tracker.unreg_projectee(&autocharge_id, projectee_item_id);
            let autocharge = self
                .uad
                .items
                .get_item_mut(&autocharge_id)
                .unwrap()
                .get_autocharge_mut()
                .unwrap();
            autocharge.get_projs_mut().remove(projectee_item_id);
        }
        // Update services for fighter
        self.remove_item_id_projection_from_svc(item_id, projectee_item_id);
        // Update user data for fighter
        self.proj_tracker.unreg_projectee(item_id, projectee_item_id);
        let fighter = self.uad.items.get_item_mut(item_id).unwrap().get_fighter_mut().unwrap();
        fighter.get_projs_mut().remove(projectee_item_id);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveFighterProjError {
    ProjectorNotFound(ItemFoundError),
    ProjectorIsNotFighter(ItemKindMatchError),
    ProjectionNotFound(ProjFoundError),
}
impl std::error::Error for RemoveFighterProjError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ProjectorNotFound(e) => Some(e),
            Self::ProjectorIsNotFighter(e) => Some(e),
            Self::ProjectionNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveFighterProjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ProjectorNotFound(e) => e.fmt(f),
            Self::ProjectorIsNotFighter(e) => e.fmt(f),
            Self::ProjectionNotFound(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for RemoveFighterProjError {
    fn from(error: ItemFoundError) -> Self {
        Self::ProjectorNotFound(error)
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
