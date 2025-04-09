use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn set_booster_state(&mut self, item_id: &ItemId, state: bool) -> Result<(), SetBoosterStateError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.set_booster_state_internal(item_key, state)?)
    }
    pub(in crate::sol) fn set_booster_state_internal(
        &mut self,
        item_key: ItemKey,
        state: bool,
    ) -> Result<(), ItemKindMatchError> {
        let booster = self.uad.items.get_mut(item_key).get_booster_mut()?;
        let old_a_state = booster.get_a_state();
        booster.set_booster_state(state);
        let new_a_state = booster.get_a_state();
        self.change_item_key_state_in_svc(item_key, old_a_state, new_a_state);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetBoosterStateError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotBooster(#[from] ItemKindMatchError),
}
