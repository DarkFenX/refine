use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, SolarSystem, uad::item::ServiceState},
};

impl SolarSystem {
    pub fn set_service_state(&mut self, item_id: &ItemId, state: ServiceState) -> Result<(), SetServiceStateError> {
        let service = self.uad.items.get_item_mut(item_id)?.get_service_mut()?;
        let old_a_state = service.get_a_state();
        service.set_service_state(state);
        let new_a_state = service.get_a_state();
        self.change_item_id_state_in_svc(item_id, old_a_state, new_a_state);
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
