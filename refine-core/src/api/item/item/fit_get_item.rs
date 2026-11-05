use crate::{
    Fit, FitId, Item, ItemId,
    err::basic::ItemFoundError,
    sol::SolCore,
    ud::{UFitId, UItemId},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fit<'s> {
    pub fn get_item(&mut self, item_id: &ItemId) -> Result<Item<'_>, FitItemGetError> {
        let item_uid = self.sol.internal_get_fit_item_uid(self.uid, item_id)?;
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn internal_get_fit_item_uid(
        &self,
        fit_uid: UFitId,
        item_id: &ItemId,
    ) -> Result<UItemId, FitItemGetError> {
        let item_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        if self.u_data.items.get(item_uid).get_fit_uid() != Some(fit_uid) {
            return Err(FitItemGetError::ItemIsNotOnThisFit(
                self.u_data.fits.ext_id_by_int_id(fit_uid),
                *item_id,
            ));
        }
        Ok(item_uid)
    }
}
