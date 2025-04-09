use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn set_sw_effect_state(&mut self, item_id: &ItemId, state: bool) -> Result<(), SetSwEffectStateError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.set_sw_effect_state_internal(item_key, state)?)
    }
    pub(in crate::sol) fn set_sw_effect_state_internal(
        &mut self,
        item_key: ItemKey,
        state: bool,
    ) -> Result<(), ItemKindMatchError> {
        let sw_effect = self.uad.items.get_mut(item_key).get_sw_effect_mut()?;
        let old_a_state = sw_effect.get_a_state();
        sw_effect.set_sw_effect_state(state);
        let new_a_state = sw_effect.get_a_state();
        self.change_item_key_state_in_svc(item_key, old_a_state, new_a_state);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetSwEffectStateError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotSwEffect(#[from] ItemKindMatchError),
}
