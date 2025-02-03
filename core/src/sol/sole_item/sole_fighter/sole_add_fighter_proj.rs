use itertools::Itertools;

use crate::{
    defs::{AttrVal, SolItemId},
    err::basic::{ItemFoundError, ItemKindMatchError, ItemReceiveProjError, ProjNotFoundError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn add_fighter_proj(
        &mut self,
        item_id: &SolItemId,
        projectee_item_id: SolItemId,
        range: Option<AttrVal>,
    ) -> Result<(), AddFighterProjError> {
        // Check projector
        let fighter = self
            .uad
            .items
            .get_item(item_id)
            .map_err(AddFighterProjError::ProjectorNotFound)?
            .get_fighter()
            .map_err(AddFighterProjError::ProjectorIsNotFighter)?;
        // Check if projection has already been defined
        if fighter.get_projs().contains(&projectee_item_id) {
            return Err(AddFighterProjError::ProjectionAlreadyExists(ProjNotFoundError::new(
                *item_id,
                projectee_item_id,
            )));
        }
        // Check if projectee can receive projections
        let projectee_item = self
            .uad
            .items
            .get_item(&projectee_item_id)
            .map_err(AddFighterProjError::ProjecteeNotFound)?;
        if !projectee_item.can_receive_projs() {
            return Err(AddFighterProjError::ProjecteeCantTakeProjs(ItemReceiveProjError::new(
                projectee_item_id,
                projectee_item.get_name(),
            )));
        }
        // Update user data for fighter
        let fighter = self.uad.items.get_item_mut(item_id).unwrap().get_fighter_mut().unwrap();
        let autocharge_ids = fighter.get_autocharges().values().copied().collect_vec();
        fighter.get_projs_mut().add(projectee_item_id, range);
        self.proj_tracker.reg_projectee(*item_id, projectee_item_id);
        // Update services for fighter
        self.add_item_id_projection_to_svc(item_id, &projectee_item_id, range);
        for autocharge_id in autocharge_ids {
            // Update user data for autocharge
            let autocharge = self
                .uad
                .items
                .get_item_mut(&autocharge_id)
                .unwrap()
                .get_autocharge_mut()
                .unwrap();
            autocharge.get_projs_mut().add(projectee_item_id, range);
            self.proj_tracker.reg_projectee(autocharge_id, projectee_item_id);
            // Update services for autocharge
            self.add_item_id_projection_to_svc(&autocharge_id, &projectee_item_id, range);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum AddFighterProjError {
    ProjectorNotFound(ItemFoundError),
    ProjectorIsNotFighter(ItemKindMatchError),
    ProjecteeNotFound(ItemFoundError),
    ProjecteeCantTakeProjs(ItemReceiveProjError),
    ProjectionAlreadyExists(ProjNotFoundError),
}
impl std::error::Error for AddFighterProjError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ProjectorNotFound(e) => Some(e),
            Self::ProjectorIsNotFighter(e) => Some(e),
            Self::ProjecteeNotFound(e) => Some(e),
            Self::ProjecteeCantTakeProjs(e) => Some(e),
            Self::ProjectionAlreadyExists(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddFighterProjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ProjectorNotFound(e) => e.fmt(f),
            Self::ProjectorIsNotFighter(e) => e.fmt(f),
            Self::ProjecteeNotFound(e) => e.fmt(f),
            Self::ProjecteeCantTakeProjs(e) => e.fmt(f),
            Self::ProjectionAlreadyExists(e) => e.fmt(f),
        }
    }
}
impl From<ProjNotFoundError> for AddFighterProjError {
    fn from(error: ProjNotFoundError) -> Self {
        Self::ProjectionAlreadyExists(error)
    }
}
