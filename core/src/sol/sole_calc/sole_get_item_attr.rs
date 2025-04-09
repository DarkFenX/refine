use crate::{
    ad,
    err::basic::{AttrMetaFoundError, ItemFoundError, ItemLoadedError},
    sol::{
        AttrId, ItemId, ItemKey, SolarSystem,
        svc::calc::{AttrCalcError, CalcAttrVal},
    },
};

impl SolarSystem {
    pub fn get_item_attr(&mut self, item_id: &ItemId, attr_id: &AttrId) -> Result<CalcAttrVal, GetItemAttrError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        match self.get_item_attr_internal(item_key, attr_id) {
            Ok(val) => Ok(val),
            Err(error) => Err(match error {
                AttrCalcError::KeyedItemNotLoaded(_) => ItemLoadedError { item_id: *item_id }.into(),
                AttrCalcError::AttrMetaNotFound(e) => e.into(),
            }),
        }
    }
    pub(in crate::sol) fn get_item_attr_internal(
        &mut self,
        item_key: ItemKey,
        a_attr_id: &ad::AAttrId,
    ) -> Result<CalcAttrVal, AttrCalcError> {
        self.svc.calc.get_item_attr_val_full(&self.uad, item_key, a_attr_id)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemAttrError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
    #[error("{0}")]
    AttrMetaNotFound(#[from] AttrMetaFoundError),
}
