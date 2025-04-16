use crate::{
    err::basic::FighterCountError,
    sol::{Count, ItemKey, SolarSystem, api::FighterMut},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_set_fighter_count_override(&mut self, item_key: ItemKey, count_override: Count) {
        // Update user data
        let uad_fighter = self.uad.items.get_mut(item_key).get_fighter_mut().unwrap();
        let old_count = uad_fighter.get_count().map(|v| v.current);
        uad_fighter.set_count_override(Some(count_override));
        let new_count = uad_fighter.get_count().map(|v| v.current);
        // Update services
        if old_count != new_count {
            let uad_fighter = self.uad.items.get(item_key).get_fighter().unwrap();
            self.svc.fighter_count_changed(&self.uad, item_key, uad_fighter);
        }
    }
}

impl<'a> FighterMut<'a> {
    pub fn set_count_override(self, count_override: Count) -> Result<Self, SetFighterCountOverrideError> {
        if count_override < 1 {
            return Err(FighterCountError { count: count_override }.into());
        }
        self.sol.internal_set_fighter_count_override(self.key, count_override);
        Ok(self)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetFighterCountOverrideError {
    #[error("{0}")]
    FighterCountError(#[from] FighterCountError),
}
