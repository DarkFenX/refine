use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn set_rig_state(&mut self, item_id: &ItemId, state: bool) -> Result<(), SetRigStateError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.set_rig_state_internal(item_key, state)?)
    }
    pub(in crate::sol) fn set_rig_state_internal(
        &mut self,
        item_key: ItemKey,
        state: bool,
    ) -> Result<(), ItemKindMatchError> {
        let rig = self.uad.items.get_mut(item_key).get_rig_mut()?;
        let old_a_state = rig.get_a_state();
        rig.set_rig_state(state);
        let new_a_state = rig.get_a_state();
        self.change_item_key_state_in_svc(item_key, old_a_state, new_a_state);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetRigStateError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotRig(#[from] ItemKindMatchError),
}
