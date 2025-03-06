use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn remove_fighter_count_override(
        &mut self,
        item_id: &SolItemId,
    ) -> Result<(), RemoveFighterCountOverrideError> {
        // Update user data
        let fighter = self.uad.items.get_item_mut(item_id)?.get_fighter_mut()?;
        let old_count = fighter.get_count();
        fighter.set_count_override(None);
        let new_count = fighter.get_count();
        // Update services
        if old_count != new_count {
            let fighter = self.uad.items.get_item(item_id).unwrap().get_fighter().unwrap();
            self.svc.fighter_count_changed(&self.uad, fighter);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveFighterCountOverrideError {
    ItemNotFound(ItemFoundError),
    ItemIsNotFighter(ItemKindMatchError),
}
impl std::error::Error for RemoveFighterCountOverrideError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotFighter(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveFighterCountOverrideError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotFighter(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for RemoveFighterCountOverrideError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveFighterCountOverrideError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotFighter(error)
    }
}
