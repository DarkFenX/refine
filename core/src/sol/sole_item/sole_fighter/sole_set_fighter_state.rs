use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{item::SolItemState, SolarSystem},
};

impl SolarSystem {
    pub fn set_fighter_state(&mut self, item_id: &SolItemId, state: SolItemState) -> Result<(), SetFighterStateError> {
        let fighter = self.items.get_item_mut(item_id)?.get_fighter_mut()?;
        let old_state = fighter.state;
        fighter.state = state;
        self.change_item_id_state_in_svcs(item_id, old_state, state);
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
