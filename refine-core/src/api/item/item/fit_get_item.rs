use crate::{Fit, FitId, Item, ItemId, err::basic::ItemFoundError};

impl<'s> Fit<'s> {
    pub fn get_item(&mut self, item_id: &ItemId) -> Result<Item<'_>, FitItemGetError> {
        let item_uid = self.sol.u_data.items.int_id_by_ext_id_err(item_id)?;
        if self.sol.u_data.items.get(item_uid).get_fit_uid() != Some(self.uid) {
            return Err(FitItemGetError::ItemIsNotOnThisFit(
                self.sol.u_data.fits.ext_id_by_int_id(self.uid),
                *item_id,
            ));
        }
        Ok(Item::new(self.sol, item_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FitItemGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error("item {1} is not on fit {0}")]
    ItemIsNotOnThisFit(FitId, ItemId),
}
