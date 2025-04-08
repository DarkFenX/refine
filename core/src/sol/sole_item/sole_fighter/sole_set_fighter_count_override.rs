use crate::{
    err::basic::{FighterCountError, ItemFoundError, ItemKindMatchError},
    sol::{Count, ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn set_fighter_count_override(
        &mut self,
        item_id: &ItemId,
        count_override: Count,
    ) -> Result<(), SetFighterCountOverrideError> {
        if count_override < 1 {
            return Err(FighterCountError { count: count_override }.into());
        }
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.set_fighter_count_override_internal(item_key, count_override)?)
    }
    pub(in crate::sol) fn set_fighter_count_override_internal(
        &mut self,
        item_key: ItemKey,
        count_override: Count,
    ) -> Result<(), ItemKindMatchError> {
        // Update user data
        let fighter = self.uad.items.get_mut(item_key).get_fighter_mut()?;
        let old_count = fighter.get_count().map(|v| v.current);
        fighter.set_count_override(Some(count_override));
        let new_count = fighter.get_count().map(|v| v.current);
        // Update services
        if old_count != new_count {
            let fighter = self.uad.items.get(item_key).get_fighter().unwrap();
            self.svc.fighter_count_changed(&self.uad, item_key, fighter);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetFighterCountOverrideError {
    ItemNotFound(ItemFoundError),
    ItemIsNotFighter(ItemKindMatchError),
    FighterCountError(FighterCountError),
}
impl std::error::Error for SetFighterCountOverrideError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotFighter(e) => Some(e),
            Self::FighterCountError(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetFighterCountOverrideError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotFighter(e) => e.fmt(f),
            Self::FighterCountError(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for SetFighterCountOverrideError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetFighterCountOverrideError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotFighter(error)
    }
}
impl From<FighterCountError> for SetFighterCountOverrideError {
    fn from(error: FighterCountError) -> Self {
        Self::FighterCountError(error)
    }
}
