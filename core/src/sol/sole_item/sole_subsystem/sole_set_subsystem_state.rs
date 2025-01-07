use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn set_subsystem_state(&mut self, item_id: &SolItemId, state: bool) -> Result<(), SetSubsystemStateError> {
        let subsystem = self.uad.items.get_item_mut(item_id)?.get_subsystem_mut()?;
        let old_state = subsystem.get_state();
        subsystem.set_bool_state(state);
        let new_state = subsystem.get_state();
        self.change_item_id_state_in_svc(item_id, old_state, new_state);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetSubsystemStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotSubsystem(ItemKindMatchError),
}
impl std::error::Error for SetSubsystemStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotSubsystem(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetSubsystemStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotSubsystem(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for SetSubsystemStateError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetSubsystemStateError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotSubsystem(error)
    }
}
