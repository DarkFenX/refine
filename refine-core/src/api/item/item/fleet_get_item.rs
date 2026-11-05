use crate::{
    Fleet, FleetId, Item, ItemId,
    err::basic::ItemFoundError,
    sol::SolCore,
    ud::{UFleetId, UItemId},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fleet<'s> {
    pub fn get_item(&mut self, item_id: &ItemId) -> Result<Item<'_>, FleetItemGetError> {
        let item_uid = self.sol.internal_get_fleet_item_uid(self.uid, item_id)?;
        Ok(Item::new(self.sol, item_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FleetItemGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error("item {1} is not in fleet {0}")]
    ItemIsNotInThisFleet(FleetId, ItemId),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn internal_get_fleet_item_uid(
        &self,
        fleet_uid: UFleetId,
        item_id: &ItemId,
    ) -> Result<UItemId, FleetItemGetError> {
        let item_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        let fit_uid = self.u_data.items.get(item_uid).get_fit_uid();
        if fit_uid.and_then(|fit_uid| self.u_data.fits.get(fit_uid).fleet) != Some(fleet_uid) {
            return Err(FleetItemGetError::ItemIsNotInThisFleet(
                self.u_data.fleets.ext_id_by_int_id(fleet_uid),
                *item_id,
            ));
        }
        Ok(item_uid)
    }
}
