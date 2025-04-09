use crate::{
    err::basic::ItemFoundError,
    sol::{EffectId, EffectMode, ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn set_item_effect_mode(
        &mut self,
        item_id: &ItemId,
        effect_id: &EffectId,
        mode: EffectMode,
    ) -> Result<(), SetItemEffectModeError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.set_item_effect_mode_internal(item_key, effect_id, mode);
        Ok(())
    }
    pub(in crate::sol) fn set_item_effect_mode_internal(
        &mut self,
        item_key: ItemKey,
        effect_id: &EffectId,
        mode: EffectMode,
    ) {
        self.uad
            .items
            .get_mut(item_key)
            .get_effect_modes_mut()
            .set(effect_id.into(), mode);
        let item = self.uad.items.get(item_key);
        self.svc.process_effects(&self.uad, item_key, item, item.get_a_state());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetItemEffectModeError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
}
