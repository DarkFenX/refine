use crate::{
    ad,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{EffectId, EffectMode, ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn set_booster_side_effect_state(
        &mut self,
        item_id: &ItemId,
        effect_id: &EffectId,
        state: bool,
    ) -> Result<(), SetBoosterSideEffectStateError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.set_booster_side_effect_state_internal(item_key, effect_id.into(), state)?)
    }
    pub(in crate::sol) fn set_booster_side_effect_state_internal(
        &mut self,
        item_key: ItemKey,
        a_effect_id: ad::AEffectId,
        state: bool,
    ) -> Result<(), ItemKindMatchError> {
        let booster = self.uad.items.get_mut(item_key).get_booster_mut()?;
        let effect_state = match state {
            true => EffectMode::StateCompliance,
            false => EffectMode::FullCompliance,
        };
        booster.get_effect_modes_mut().set(a_effect_id, effect_state);
        let item = self.uad.items.get(item_key);
        self.svc.process_effects(&self.uad, item_key, item, item.get_a_state());
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetBoosterSideEffectStateError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotBooster(#[from] ItemKindMatchError),
}
