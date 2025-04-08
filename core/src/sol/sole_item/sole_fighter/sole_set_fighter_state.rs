use itertools::Itertools;

use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, uad::item::MinionState},
};

impl SolarSystem {
    pub fn set_fighter_state(&mut self, item_id: &ItemId, state: MinionState) -> Result<(), SetFighterStateError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.set_fighter_state_internal(item_key, state)?)
    }
    pub(in crate::sol) fn set_fighter_state_internal(
        &mut self,
        item_key: ItemKey,
        state: MinionState,
    ) -> Result<(), ItemKindMatchError> {
        // Update user data for fighter
        let fighter = self.uad.items.get_mut(item_key).get_fighter_mut()?;
        let autocharge_keys = fighter.get_autocharges().values().copied().collect_vec();
        let old_a_state = fighter.get_a_state();
        fighter.set_fighter_state(state);
        let new_a_state = fighter.get_a_state();
        // Update services for fighter
        self.change_item_key_state_in_svc(item_key, old_a_state, new_a_state);
        for autocharge_key in autocharge_keys {
            // Update user data for autocharge
            let autocharge = self.uad.items.get_mut(autocharge_key).get_autocharge_mut().unwrap();
            let old_a_state = autocharge.get_a_state();
            autocharge.set_a_state(state.into());
            // Update services for autocharge
            let new_a_state = autocharge.get_a_state();
            self.change_item_key_state_in_svc(autocharge_key, old_a_state, new_a_state);
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
