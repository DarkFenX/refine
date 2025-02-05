use itertools::Itertools;

use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{uad::item::SolMinionState, SolarSystem},
};

impl SolarSystem {
    pub fn set_fighter_state(
        &mut self,
        item_id: &SolItemId,
        state: SolMinionState,
    ) -> Result<(), SetFighterStateError> {
        // Update user data for fighter
        let fighter = self.uad.items.get_item_mut(item_id)?.get_fighter_mut()?;
        let autocharge_ids = fighter.get_autocharges().values().copied().collect_vec();
        let old_state = fighter.get_state();
        fighter.set_fighter_state(state);
        // Update services for fighter
        self.change_item_id_state_in_svc(item_id, old_state, state.into());
        for autocharge_id in autocharge_ids {
            // Update user data for autocharge
            let autocharge = self
                .uad
                .items
                .get_item_mut(&autocharge_id)
                .unwrap()
                .get_autocharge_mut()
                .unwrap();
            let old_state = autocharge.get_state();
            autocharge.set_state(state.into());
            // Update services for autocharge
            let new_state = autocharge.get_state();
            self.change_item_id_state_in_svc(&autocharge_id, old_state, new_state);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetFighterStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotFighter(ItemKindMatchError),
}
impl std::error::Error for SetFighterStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotFighter(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetFighterStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotFighter(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for SetFighterStateError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetFighterStateError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotFighter(error)
    }
}
