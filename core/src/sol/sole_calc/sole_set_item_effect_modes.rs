use crate::{
    defs::{EEffectId, SolItemId},
    sol::{err::basic::ItemFoundError, SolEffectMode, SolView, SolarSystem},
};

impl SolarSystem {
    pub fn set_item_effect_modes(
        &mut self,
        item_id: &SolItemId,
        modes: impl Iterator<Item = (EEffectId, SolEffectMode)>,
    ) -> Result<(), SetItemEffectModesError> {
        let effect_modes = self.items.get_item_mut(item_id)?.get_effect_modes_mut();
        for (effect_id, effect_mode) in modes {
            effect_modes.set(effect_id, effect_mode)
        }
        let item = self.items.get_item(item_id).unwrap();
        self.svcs.process_effects(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            item,
            item.get_state(),
        );
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetItemEffectModesError {
    ItemNotFound(ItemFoundError),
}
impl From<ItemFoundError> for SetItemEffectModesError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
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
