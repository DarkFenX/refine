use crate::{Fleet, FleetId, Item, ItemId, err::basic::ItemFoundError};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fleet<'s> {
    pub fn get_item(&mut self, item_id: &ItemId) -> Result<Item<'_>, FleetItemGetError> {
        let item_uid = self.core.u_data.items.int_id_by_ext_id_err(item_id)?;
        let fit_uid = self.core.u_data.items.get(item_uid).get_fit_uid();
        if fit_uid.and_then(|fit_uid| self.core.u_data.fits.get(fit_uid).fleet) != Some(self.uid) {
            return Err(FleetItemGetError::ItemIsNotInThisFleet(
                self.core.u_data.fleets.ext_id_by_int_id(self.uid),
                *item_id,
            ));
        }
        Ok(Item::new(self.core, item_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FleetItemGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error("item {1} is not in fleet {0}")]
    ItemIsNotInThisFleet(FleetId, ItemId),
}
