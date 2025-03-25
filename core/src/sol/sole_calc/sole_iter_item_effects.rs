use crate::{
    err::basic::{ItemFoundError, ItemLoadedError},
    sol::{EffectId, EffectInfo, ItemId, SolarSystem},
};

impl SolarSystem {
    pub fn iter_item_effects(
        &self,
        item_id: &ItemId,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        let item = self.uad.items.get_item(item_id)?;
        let a_effect_ids = item.get_a_effect_datas_err()?.keys();
        let effect_infos = a_effect_ids.map(|a_effect_id| {
            let running = self.svc.is_effect_running(item_id, a_effect_id);
            let mode = item.get_effect_modes().get(a_effect_id);
            (a_effect_id.into(), EffectInfo { running, mode: *mode })
        });
        Ok(effect_infos)
    }
}

#[derive(Debug)]
pub enum IterItemEffectsError {
    ItemNotFound(ItemFoundError),
    ItemNotLoaded(ItemLoadedError),
}
impl std::error::Error for IterItemEffectsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemNotLoaded(e) => Some(e),
        }
    }
}
impl std::fmt::Display for IterItemEffectsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemNotLoaded(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for IterItemEffectsError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemLoadedError> for IterItemEffectsError {
    fn from(error: ItemLoadedError) -> Self {
        Self::ItemNotLoaded(error)
    }
}
