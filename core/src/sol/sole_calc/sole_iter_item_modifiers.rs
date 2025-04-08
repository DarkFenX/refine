use crate::{
    err::basic::{ItemFoundError, ItemLoadedError},
    sol::{AttrId, ItemId, ItemKey, SolarSystem, err::KeyedItemLoadedError, svc::calc::ModificationInfo},
};

impl SolarSystem {
    pub fn iter_item_modifiers(
        &mut self,
        item_id: &ItemId,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        match self.iter_item_modifiers_internal(item_key) {
            Ok(mods_iter) => Ok(mods_iter),
            Err(_) => Err(ItemLoadedError { item_id: *item_id }.into()),
        }
    }
    pub(in crate::sol) fn iter_item_modifiers_internal(
        &mut self,
        item_key: ItemKey,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, KeyedItemLoadedError> {
        self.svc.calc.iter_item_mods(&self.uad, item_key)
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
impl From<ItemFoundError> for IterItemModifiersError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemLoadedError> for IterItemModifiersError {
    fn from(error: ItemLoadedError) -> Self {
        Self::ItemNotLoaded(error)
    }
}
