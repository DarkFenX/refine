use crate::{
    PValue, Ship,
    sol::{CoreInner, UndoStep},
    ud::{UItemId, UShipKind},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Ship<'s> {
    pub fn remove(self) {
        self.core.internal_remove_ship(self.uid)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_remove_ship(&mut self, ship_uid: UItemId) {
        let u_ship = self.u_data.items.get(ship_uid).dc_ship().unwrap();
        let fit_uid = u_ship.get_fit_uid();
        // Remove incoming projections
        self.internal_remove_incoming_projections(ship_uid);
        // Remove ship from services
        CoreInner::util_remove_ship(&mut self.u_data, &mut self.svc, ship_uid, &mut self.reuse.eupdates);
        // Remove ship from user data
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        u_fit.ship = None;
        u_fit.ship_kind = UShipKind::Unknown;
        self.internal_discard_item(ship_uid, UndoStep::ShipRemoved);
        // Update projections outgoing from on-ship items
        CoreInner::util_update_ship_radius_for_outgoing_projs(&mut self.u_data, &mut self.svc, fit_uid, PValue::ZERO);
    }
}
