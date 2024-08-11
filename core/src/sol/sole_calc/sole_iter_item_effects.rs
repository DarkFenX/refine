use crate::{
    defs::{EEffectId, SolItemId},
    err::basic::{ItemFoundError, ItemLoadedError},
    sol::{SolEffectInfo, SolarSystem},
};

impl SolarSystem {
    pub fn iter_item_effects<'a>(
        &'a self,
        item_id: &'a SolItemId,
    ) -> Result<impl ExactSizeIterator<Item = (EEffectId, SolEffectInfo)> + 'a, IterItemEffectsError> {
        let item = self.items.get_item(item_id)?;
        let a_effect_ids = item.get_effect_datas()?.keys();
        let effect_infos = a_effect_ids.map(move |v| {
            let running = self.svcs.is_effect_running(item_id, v);
            let mode = item.get_effect_modes().get(v);
            (*v, SolEffectInfo::new(running, *mode))
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
