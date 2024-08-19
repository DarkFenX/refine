use itertools::Itertools;

use crate::{
    defs::{AttrVal, SolItemId},
    err::basic::{ItemFoundError, ItemKindMatchError, ProjFoundError},
    sol::{SolView, SolarSystem},
};

impl SolarSystem {
    pub fn change_fighter_proj(
        &mut self,
        item_id: &SolItemId,
        projectee_item_id: &SolItemId,
        range: Option<AttrVal>,
    ) -> Result<(), ChangeFighterProjError> {
        // Check if projection is defined before changing it
        let fighter = self.items.get_item(item_id)?.get_fighter()?;
        let old_range = match fighter.get_projs().get(projectee_item_id) {
            Some(old_range) => *old_range,
            None => return Err(ProjFoundError::new(*item_id, *projectee_item_id).into()),
        };
        // Do nothing if ranges are equal
        if range == old_range {
            return Ok(());
        }
        // Update skeleton for fighter
        let fighter = self.items.get_item_mut(item_id).unwrap().get_fighter_mut().unwrap();
        let autocharge_ids = fighter.get_autocharges().values().map(|v| *v).collect_vec();
        fighter.get_projs_mut().add(*projectee_item_id, range);
        // Update services for fighter
        let fighter_item = self.items.get_item(item_id).unwrap();
        let projectee_item = self.items.get_item(projectee_item_id).unwrap();
        self.svcs.change_item_proj_range(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            fighter_item,
            projectee_item,
            range,
        );
        for autocharge_id in autocharge_ids {
            // Update skeleton for autocharge
            let autocharge = self
                .items
                .get_item_mut(&autocharge_id)
                .unwrap()
                .get_autocharge_mut()
                .unwrap();
            autocharge.get_projs_mut().add(*projectee_item_id, range);
            // Update services for autocharge
            let autocharge_item = self.items.get_item(&autocharge_id).unwrap();
            let projectee_item = self.items.get_item(projectee_item_id).unwrap();
            self.svcs.change_item_proj_range(
                &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                autocharge_item,
                projectee_item,
                range,
            );
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ChangeFighterProjError {
    ProjectorNotFound(ItemFoundError),
    ProjectorIsNotFighter(ItemKindMatchError),
    ProjectionNotFound(ProjFoundError),
}
impl std::error::Error for ChangeFighterProjError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ProjectorNotFound(e) => Some(e),
            Self::ProjectorIsNotFighter(e) => Some(e),
            Self::ProjectionNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for ChangeFighterProjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ProjectorNotFound(e) => e.fmt(f),
            Self::ProjectorIsNotFighter(e) => e.fmt(f),
            Self::ProjectionNotFound(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for ChangeFighterProjError {
    fn from(error: ItemFoundError) -> Self {
        Self::ProjectorNotFound(error)
    }
}
impl From<ItemKindMatchError> for ChangeFighterProjError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ProjectorIsNotFighter(error)
    }
}
impl From<ProjFoundError> for ChangeFighterProjError {
    fn from(error: ProjFoundError) -> Self {
        Self::ProjectionNotFound(error)
    }
}
