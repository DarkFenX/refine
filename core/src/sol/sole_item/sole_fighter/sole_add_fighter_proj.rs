use itertools::Itertools;

use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ItemReceiveProjError, ProjNotFoundError},
    sol::{AttrVal, ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn add_fighter_proj(
        &mut self,
        item_id: &ItemId,
        projectee_item_id: &ItemId,
        range: Option<AttrVal>,
    ) -> Result<(), AddFighterProjError> {
        let item_key = self
            .uad
            .items
            .key_by_id_err(item_id)
            .map_err(AddFighterProjError::ProjectorNotFound)?;
        let projectee_item_key = self
            .uad
            .items
            .key_by_id_err(projectee_item_id)
            .map_err(AddFighterProjError::ProjecteeNotFound)?;
        self.add_fighter_proj_internal(item_key, projectee_item_key, range)
    }
    pub(in crate::sol) fn add_fighter_proj_internal(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
        range: Option<AttrVal>,
    ) -> Result<(), AddFighterProjError> {
        // Check projector
        let fighter = self.uad.items.get(item_key).get_fighter()?;
        // Check if projection has already been defined
        let projectee_item = self.uad.items.get(projectee_item_key);
        if fighter.get_projs().contains(&projectee_item_key) {
            return Err(ProjNotFoundError {
                projector_item_id: fighter.get_item_id(),
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
        // Update user data for fighter
        let fighter = self.uad.items.get_mut(item_key).get_fighter_mut().unwrap();
        let autocharge_keys = fighter.get_autocharges().values().copied().collect_vec();
        fighter.get_projs_mut().add(projectee_item_key, range);
        self.proj_tracker.reg_projectee(item_key, projectee_item_key);
        // Update services for fighter
        self.add_item_key_projection_to_svc(item_key, projectee_item_key, range);
        for autocharge_key in autocharge_keys {
            // Update user data for autocharge
            let autocharge = self.uad.items.get_mut(autocharge_key).get_autocharge_mut().unwrap();
            autocharge.get_projs_mut().add(projectee_item_key, range);
            self.proj_tracker.reg_projectee(autocharge_key, projectee_item_key);
            // Update services for autocharge
            self.add_item_key_projection_to_svc(autocharge_key, projectee_item_key, range);
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddFighterProjError {
    #[error("{0}")]
    ProjectorNotFound(#[source] ItemFoundError),
    #[error("{0}")]
    ProjectorIsNotFighter(#[from] ItemKindMatchError),
    #[error("{0}")]
    ProjecteeNotFound(#[source] ItemFoundError),
    #[error("{0}")]
    ProjecteeCantTakeProjs(#[from] ItemReceiveProjError),
    #[error("{0}")]
    ProjectionAlreadyExists(#[from] ProjNotFoundError),
}
