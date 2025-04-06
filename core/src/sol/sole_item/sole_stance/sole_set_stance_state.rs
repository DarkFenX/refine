use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, SolarSystem},
};

impl SolarSystem {
    pub fn set_stance_state(&mut self, item_id: &ItemId, state: bool) -> Result<(), SetStanceStateError> {
        let stance = self.uad.items.get_mut_by_id(item_id)?.get_stance_mut()?;
        let old_a_state = stance.get_a_state();
        stance.set_stance_state(state);
        let new_a_state = stance.get_a_state();
        self.change_item_id_state_in_svc(item_id, old_a_state, new_a_state);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetStanceStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotStance(ItemKindMatchError),
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
