use itertools::Itertools;

use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ProjFoundError},
    sol::{AttrVal, ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn change_fighter_proj(
        &mut self,
        item_id: &ItemId,
        projectee_item_id: &ItemId,
        range: Option<AttrVal>,
    ) -> Result<(), ChangeFighterProjError> {
        let item_key = self
            .uad
            .items
            .key_by_id_err(item_id)
            .map_err(ChangeFighterProjError::ProjectorNotFound)?;
        let projectee_item_key = self
            .uad
            .items
            .key_by_id_err(projectee_item_id)
            .map_err(ChangeFighterProjError::ProjecteeNotFound)?;
        self.change_fighter_proj_internal(item_key, projectee_item_key, range)
    }
    pub(in crate::sol) fn change_fighter_proj_internal(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
        range: Option<AttrVal>,
    ) -> Result<(), ChangeFighterProjError> {
        // Check if projection is defined before changing it
        let fighter = self.uad.items.get(item_key).get_fighter()?;
        let old_range = match fighter.get_projs().get(&projectee_item_key) {
            Some(old_range) => *old_range,
            None => {
                return Err(ProjFoundError {
                    projector_item_id: fighter.get_item_id(),
                    projectee_item_id: self.uad.items.id_by_key(projectee_item_key),
                }
                .into());
            }
        };
        // Do nothing if ranges are equal
        if range == old_range {
            return Ok(());
        }
        // Update user data for fighter
        let fighter = self.uad.items.get_mut(item_key).get_fighter_mut().unwrap();
        fighter.get_projs_mut().add(projectee_item_key, range);
        let autocharge_keys = fighter.get_autocharges().values().copied().collect_vec();
        // Update services for fighter
        self.change_item_key_projection_range_in_svc(item_key, projectee_item_key, range);
        for autocharge_key in autocharge_keys {
            // Update user data for autocharge
            let autocharge = self.uad.items.get_mut(autocharge_key).get_autocharge_mut().unwrap();
            autocharge.get_projs_mut().add(projectee_item_key, range);
            // Update services for autocharge
            self.change_item_key_projection_range_in_svc(autocharge_key, projectee_item_key, range);
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeFighterProjError {
    #[error("{0}")]
    ProjectorNotFound(#[source] ItemFoundError),
    #[error("{0}")]
    ProjectorIsNotFighter(#[from] ItemKindMatchError),
    #[error("{0}")]
    ProjecteeNotFound(#[source] ItemFoundError),
    #[error("{0}")]
    ProjectionNotFound(#[from] ProjFoundError),
}
