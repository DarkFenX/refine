use crate::{
    err::basic::ItemFoundError,
    sol::{EffectId, EffectMode, ItemId, SolarSystem},
};

impl SolarSystem {
    pub fn set_item_effect_modes(
        &mut self,
        item_id: &ItemId,
        modes: impl Iterator<Item = (EffectId, EffectMode)>,
    ) -> Result<(), SetItemEffectModesError> {
        let effect_modes = self.uad.items.get_item_mut(item_id)?.get_effect_modes_mut();
        for (effect_id, effect_mode) in modes {
            effect_modes.set(effect_id.into(), effect_mode)
        }
        let item = self.uad.items.get_item(item_id).unwrap();
        self.svc.process_effects(&self.uad, item, item.get_a_state());
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetItemEffectModesError {
    ItemNotFound(ItemFoundError),
}
impl std::error::Error for SetItemEffectModesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetItemEffectModesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for SetItemEffectModesError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
