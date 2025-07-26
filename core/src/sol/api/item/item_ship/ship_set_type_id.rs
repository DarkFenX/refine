use crate::{
    ad,
    def::ItemTypeId,
    sol::{SolarSystem, api::ShipMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_ship_a_item_id(
        &mut self,
        item_key: UItemKey,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(item_key);
        if u_item.get_type_id() == a_item_id {
            return;
        }
        SolarSystem::util_remove_ship(&self.u_data, &mut self.svc, item_key, u_item, reuse_eupdates);
        let u_ship = self.u_data.items.get_mut(item_key).get_ship_mut().unwrap();
        let fit_key = u_ship.get_fit_key();
        u_ship.set_type_id(a_item_id, reuse_eupdates, &self.u_data.src);
        // Update on-fit ship kind
        let ship_kind = u_ship.get_kind();
        let u_fit = self.u_data.fits.get_mut(fit_key);
        u_fit.kind = ship_kind;
        // Update outgoing projections for all on-ship items
        let ship_radius = self.u_data.get_ship_radius_by_fit_key(fit_key);
        SolarSystem::util_update_ship_radius_for_outgoing_projs(&mut self.u_data, &mut self.svc, fit_key, ship_radius);
        // Update incoming projections
        for &projector_key in self.rev_projs.iter_projectors(&item_key) {
            let projector_u_item = self.u_data.items.get_mut(projector_key);
            if let Some(u_prange) = projector_u_item.get_projs_mut().unwrap().get_range_mut(&item_key)
                && u_prange.update_tgt_rad(ship_radius)
            {
                let u_prange = Some(*u_prange);
                let u_item = self.u_data.items.get(item_key);
                let projector_u_item = self.u_data.items.get(projector_key);
                SolarSystem::util_change_item_proj_range(
                    &self.u_data,
                    &mut self.svc,
                    projector_key,
                    projector_u_item,
                    item_key,
                    u_item,
                    u_prange,
                );
            }
        }
        SolarSystem::util_add_ship(&self.u_data, &mut self.svc, item_key, reuse_eupdates);
    }
}

impl<'a> ShipMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_ship_a_item_id(self.key, type_id, &mut reuse_eupdates)
    }
}
