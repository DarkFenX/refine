use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn set_implant_state(&mut self, item_id: &ItemId, state: bool) -> Result<(), SetImplantStateError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.set_implant_state_internal(item_key, state)?)
    }
    pub(in crate::sol) fn set_implant_state_internal(
        &mut self,
        item_key: ItemKey,
        state: bool,
    ) -> Result<(), ItemKindMatchError> {
        let implant = self.uad.items.get_mut(item_key).get_implant_mut()?;
        let old_a_state = implant.get_a_state();
        implant.set_implant_state(state);
        let new_a_state = implant.get_a_state();
        self.change_item_key_state_in_svc(item_key, old_a_state, new_a_state);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetImplantStateError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotImplant(#[from] ItemKindMatchError),
}
