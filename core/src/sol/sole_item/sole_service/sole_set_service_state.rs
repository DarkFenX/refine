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

#[derive(thiserror::Error, Debug)]
pub enum SetServiceStateError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotService(#[from] ItemKindMatchError),
}
