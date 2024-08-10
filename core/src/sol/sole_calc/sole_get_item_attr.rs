use crate::{
    defs::{EAttrId, SolItemId},
    err::ItemFoundError,
    sol::{SolAttrVal, SolView, SolarSystem},
};

impl SolarSystem {
    pub fn get_item_attr(&mut self, item_id: &SolItemId, attr_id: &EAttrId) -> Result<SolAttrVal> {
        self.svcs.calc_get_item_attr_val(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            item_id,
            attr_id,
        )
    }
}

#[derive(Debug)]
pub enum GetItemAttrError {
    ItemFoundError(ItemFoundError),
}
impl From<ItemFoundError> for GetItemAttrError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemFoundError(error)
    }
}
impl std::error::Error for GetItemAttrError {}
impl std::fmt::Display for GetItemAttrError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemFoundError(e) => e.fmt(f),
        }
    }
}
