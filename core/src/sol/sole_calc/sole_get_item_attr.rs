use crate::{
    defs::{EAttrId, SolItemId},
    err::basic::{AttrMetaFoundError, ItemFoundError, ItemLoadedError},
    sol::{svc::err::AttrCalcError, SolAttrVal, SolView, SolarSystem},
};

impl SolarSystem {
    pub fn get_item_attr(&mut self, item_id: &SolItemId, attr_id: &EAttrId) -> Result<SolAttrVal, GetItemAttrError> {
        let val = self.svcs.calc_get_item_attr_val(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            item_id,
            attr_id,
        )?;
        Ok(val)
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
impl From<AttrCalcError> for GetItemAttrError {
    fn from(error: AttrCalcError) -> Self {
        match error {
            AttrCalcError::ItemNotFound(e) => Self::ItemNotFound(e),
            AttrCalcError::ItemNotLoaded(e) => Self::ItemNotLoaded(e),
            AttrCalcError::AttrMetaNotFound(e) => Self::AttrMetaNotFound(e),
        }
    }
}
