use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_fighter_count_override(&mut self, item_id: &ItemId) -> Result<(), RemoveFighterCountOverrideError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.remove_fighter_count_override_internal(item_key)?)
    }
    pub(in crate::sol) fn remove_fighter_count_override_internal(
        &mut self,
        item_key: ItemKey,
    ) -> Result<(), ItemKindMatchError> {
        // Update user data
        let fighter = self.uad.items.get_mut(item_key).get_fighter_mut()?;
        let old_count = fighter.get_count().map(|v| v.current);
        fighter.set_count_override(None);
        let new_count = fighter.get_count().map(|v| v.current);
        // Update services
        if old_count != new_count {
            let fighter = self.uad.items.get(item_key).get_fighter().unwrap();
            self.svc.fighter_count_changed(&self.uad, item_key, fighter);
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveFighterCountOverrideError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotFighter(#[from] ItemKindMatchError),
}
