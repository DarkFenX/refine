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

#[derive(thiserror::Error, Debug)]
pub enum IterItemModifiersError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
}
