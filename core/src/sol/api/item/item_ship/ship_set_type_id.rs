use crate::{
    ad,
    def::ItemTypeId,
    sol::{SolarSystem, api::ShipMut},
    uad::{UadEffectUpdates, UadItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_ship_a_item_id(
        &mut self,
        item_key: UadItemKey,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        let uad_item = self.uad.items.get(item_key);
        if uad_item.get_a_item_id() == a_item_id {
            return;
        }
        SolarSystem::util_remove_ship(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        let uad_ship = self.uad.items.get_mut(item_key).get_ship_mut().unwrap();
        let fit_key = uad_ship.get_fit_key();
        uad_ship.set_a_item_id(a_item_id, reuse_eupdates, &self.uad.src);
        // Update on-fit ship kind
        let ship_kind = uad_ship.get_kind();
        let uad_fit = self.uad.fits.get_mut(fit_key);
        uad_fit.kind = ship_kind;
        // Update outgoing projections for all on-ship items
        let ship_radius = self.uad.get_ship_radius_by_fit_key(fit_key);
        SolarSystem::util_update_ship_radius_for_outgoing_projs(&mut self.uad, &mut self.svc, fit_key, ship_radius);
        // Update incoming projections
        for &projector_key in self.rprojs.iter_projectors(&item_key) {
            let projector_uad_item = self.uad.items.get_mut(projector_key);
            if let Some(uad_prange) = projector_uad_item.get_projs_mut().unwrap().get_range_mut(&item_key)
                && uad_prange.update_tgt_rad(ship_radius)
            {
                let uad_prange = Some(*uad_prange);
                let uad_item = self.uad.items.get(item_key);
                let projector_uad_item = self.uad.items.get(projector_key);
                SolarSystem::util_change_item_proj_range(
                    &self.uad,
                    &mut self.svc,
                    projector_key,
                    projector_uad_item,
                    item_key,
                    uad_item,
                    uad_prange,
                );
            }
        }
        SolarSystem::util_add_ship(&self.uad, &mut self.svc, item_key, reuse_eupdates);
    }
}

impl<'a> ShipMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UadEffectUpdates::new();
        self.sol
            .internal_set_ship_a_item_id(self.key, type_id, &mut reuse_eupdates)
    }
}
