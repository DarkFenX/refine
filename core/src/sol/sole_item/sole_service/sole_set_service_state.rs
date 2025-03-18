use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{SolarSystem, uad::item::SolServiceState},
};

impl SolarSystem {
    pub fn set_service_state(
        &mut self,
        item_id: &SolItemId,
        state: SolServiceState,
    ) -> Result<(), SetServiceStateError> {
        let service = self.uad.items.get_item_mut(item_id)?.get_service_mut()?;
        let old_state = service.get_state();
        service.set_service_state(state);
        let new_state = service.get_state();
        self.change_item_id_state_in_svc(item_id, old_state, new_state);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetServiceStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotService(ItemKindMatchError),
}
impl std::error::Error for SetServiceStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotService(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetServiceStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotService(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for SetServiceStateError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetServiceStateError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotService(error)
    }
}
