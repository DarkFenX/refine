use crate::{
    defs::SolItemId,
    err::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn set_stance_state(&mut self, item_id: &SolItemId, state: bool) -> Result<(), SetStanceStateError> {
        let stance = self.items.get_item_mut(item_id)?.get_stance_mut()?;
        let old_state = stance.state;
        stance.set_bool_state(state);
        let new_state = stance.state;
        self.change_item_id_state_in_svcs(item_id, old_state, new_state);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetStanceStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotStance(ItemKindMatchError),
}
impl From<ItemFoundError> for SetStanceStateError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetStanceStateError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotStance(error)
    }
}
impl std::error::Error for SetStanceStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotStance(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetStanceStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotStance(e) => e.fmt(f),
        }
    }
}
