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

#[derive(thiserror::Error, Debug)]
pub enum IterItemAttrsError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
}
