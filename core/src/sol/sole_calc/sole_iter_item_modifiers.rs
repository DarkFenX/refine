use crate::{
    err::basic::{ItemFoundError, ItemLoadedError},
    sol::{
        AttrId, ItemId, SolarSystem,
        svc::calc::{LoadedItemFoundError, ModificationInfo},
    },
};

impl SolarSystem {
    pub fn iter_item_modifiers(
        &mut self,
        item_id: &ItemId,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
        let modifiers = self.svc.calc.iter_item_mods(&self.uad, item_id)?;
        Ok(modifiers)
    }
}

#[derive(Debug)]
pub enum IterItemModifiersError {
    ItemNotFound(ItemFoundError),
    ItemNotLoaded(ItemLoadedError),
}
impl std::error::Error for IterItemModifiersError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemNotLoaded(e) => Some(e),
        }
    }
}
impl std::fmt::Display for IterItemModifiersError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemNotLoaded(e) => e.fmt(f),
        }
    }
}
impl From<LoadedItemFoundError> for IterItemModifiersError {
    fn from(error: LoadedItemFoundError) -> Self {
        match error {
            LoadedItemFoundError::ItemNotFound(e) => Self::ItemNotFound(e),
            LoadedItemFoundError::ItemNotLoaded(e) => Self::ItemNotLoaded(e),
        }
    }
}
