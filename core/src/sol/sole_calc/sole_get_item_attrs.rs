use crate::{
    defs::{EAttrId, SolItemId},
    err::ItemFoundError,
    sol::{SolAttrVal, SolView, SolarSystem},
};

impl SolarSystem {
    pub fn get_item_attrs(
        &mut self,
        item_id: &SolItemId,
    ) -> Result<impl ExactSizeIterator<Item = (EAttrId, SolAttrVal)>, GetItemAttrsError> {
        let item = self.items.get_item(item_id)?;
        self.svcs
            .calc_iter_item_attr_vals(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), item)
    }
}

#[derive(Debug)]
pub enum GetItemAttrsError {
    ItemFoundError(ItemFoundError),
}
impl From<ItemFoundError> for GetItemAttrsError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemFoundError(error)
    }
}
impl std::error::Error for GetItemAttrsError {}
impl std::fmt::Display for GetItemAttrsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemFoundError(e) => e.fmt(f),
        }
    }
}
