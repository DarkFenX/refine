use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn set_booster_state(&mut self, item_id: &SolItemId, state: bool) -> Result<(), SetBoosterStateError> {
        let booster = self.items.get_item_mut(item_id)?.get_booster_mut()?;
        let old_state = booster.state;
        booster.set_bool_state(state);
        let new_state = booster.state;
        self.change_item_id_state_in_svcs(item_id, old_state, new_state);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetBoosterStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotBooster(ItemKindMatchError),
}
impl std::error::Error for SetBoosterStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotBooster(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetBoosterStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotBooster(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for SetBoosterStateError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetBoosterStateError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotBooster(error)
    }
}
