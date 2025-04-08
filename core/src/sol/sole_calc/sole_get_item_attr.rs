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
        self.svc.calc.get_item_attr_val_full(&self.uad, item_key, &a_attr_id)
    }
}

#[derive(Debug)]
pub enum GetItemAttrError {
    ItemNotFound(ItemFoundError),
    ItemNotLoaded(ItemLoadedError),
    AttrMetaNotFound(AttrMetaFoundError),
}
impl std::error::Error for GetItemAttrError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemNotLoaded(e) => Some(e),
            Self::AttrMetaNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetItemAttrError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemNotLoaded(e) => e.fmt(f),
            Self::AttrMetaNotFound(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for GetItemAttrError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemLoadedError> for GetItemAttrError {
    fn from(error: ItemLoadedError) -> Self {
        Self::ItemNotLoaded(error)
    }
}
impl From<AttrMetaFoundError> for GetItemAttrError {
    fn from(error: AttrMetaFoundError) -> Self {
        Self::AttrMetaNotFound(error)
    }
}
