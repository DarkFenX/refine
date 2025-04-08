use crate::{
    err::basic::{ItemFoundError, ItemLoadedError},
    sol::{AttrId, ItemId, ItemKey, SolarSystem, err::KeyedItemLoadedError, svc::calc::CalcAttrVal},
};

impl SolarSystem {
    pub fn iter_item_attrs(
        &mut self,
        item_id: &ItemId,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, CalcAttrVal)>, IterItemAttrsError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        match self.iter_item_attrs_internal(item_key) {
            Ok(iter) => Ok(iter),
            Err(_) => Err(ItemLoadedError { item_id: *item_id }.into()),
        }
    }
    pub(in crate::sol) fn iter_item_attrs_internal(
        &mut self,
        item_key: ItemKey,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, CalcAttrVal)>, KeyedItemLoadedError> {
        let attrs = self.svc.calc.iter_item_attr_vals(&self.uad, item_key)?;
        Ok(attrs)
    }
}

#[derive(Debug)]
pub enum IterItemAttrsError {
    ItemNotFound(ItemFoundError),
    ItemNotLoaded(ItemLoadedError),
}
impl std::error::Error for IterItemAttrsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemNotLoaded(e) => Some(e),
        }
    }
}
impl std::fmt::Display for IterItemAttrsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemNotLoaded(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for IterItemAttrsError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemLoadedError> for IterItemAttrsError {
    fn from(error: ItemLoadedError) -> Self {
        Self::ItemNotLoaded(error)
    }
}
