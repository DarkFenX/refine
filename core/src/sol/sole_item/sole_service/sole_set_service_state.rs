use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, uad::item::ServiceState},
};

impl SolarSystem {
    pub fn set_service_state(&mut self, item_id: &ItemId, state: ServiceState) -> Result<(), SetServiceStateError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.set_service_state_internal(item_key, state)?)
    }
    pub(in crate::sol) fn set_service_state_internal(
        &mut self,
        item_key: ItemKey,
        state: ServiceState,
    ) -> Result<(), ItemKindMatchError> {
        let service = self.uad.items.get_mut(item_key).get_service_mut()?;
        let old_a_state = service.get_a_state();
        service.set_service_state(state);
        let new_a_state = service.get_a_state();
        self.change_item_key_state_in_svc(item_key, old_a_state, new_a_state);
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
