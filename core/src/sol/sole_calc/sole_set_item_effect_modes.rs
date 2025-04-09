use crate::{
    err::basic::ItemFoundError,
    sol::{EffectId, EffectMode, ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn set_item_effect_modes(
        &mut self,
        item_id: &ItemId,
        modes: impl Iterator<Item = (EffectId, EffectMode)>,
    ) -> Result<(), SetItemEffectModesError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.set_item_effect_modes_internal(item_key, modes);
        Ok(())
    }
    pub(in crate::sol) fn set_item_effect_modes_internal(
        &mut self,
        item_key: ItemKey,
        modes: impl Iterator<Item = (EffectId, EffectMode)>,
    ) {
        let effect_modes = self.uad.items.get_mut(item_key).get_effect_modes_mut();
        for (effect_id, effect_mode) in modes {
            effect_modes.set(effect_id.into(), effect_mode)
        }
        let item = self.uad.items.get(item_key);
        self.svc.process_effects(&self.uad, item_key, item, item.get_a_state());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetItemEffectModesError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
}
