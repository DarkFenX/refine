use itertools::chain;

use crate::{
    ad,
    def::{ItemKey, ItemTypeId, OF},
    sol::{SolarSystem, api::ShipMut},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_ship_a_item_id(&mut self, item_key: ItemKey, a_item_id: ad::AItemId) {
        let uad_item = self.uad.items.get(item_key);
        if uad_item.get_a_item_id() == a_item_id {
            return;
        }
        SolarSystem::util_remove_ship(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        let uad_ship = self.uad.items.get_mut(item_key).get_ship_mut().unwrap();
        let fit_key = uad_ship.get_fit_key();
        uad_ship.set_a_item_id(&self.uad.src, a_item_id);
        // Update on-fit ship kind
        let ship_kind = uad_ship.get_kind();
        let uad_fit = self.uad.fits.get_mut(fit_key);
        uad_fit.kind = ship_kind;
        // Update outgoing projections for all on-ship items
        let uad_ship = self.uad.items.get(item_key).get_ship().unwrap();
        let ship_radius = uad_ship.get_a_extras().and_then(|v| v.radius).unwrap_or(OF(0.0));
        let mut projections_to_update = Vec::new();
        for &module_item_key in chain!(
            uad_fit.mods_high.iter_keys(),
            uad_fit.mods_mid.iter_keys(),
            uad_fit.mods_low.iter_keys()
        ) {
            let uad_module = self.uad.items.get_mut(module_item_key).get_module_mut().unwrap();
            for (projectee_item_key, uad_prange) in uad_module.get_projs_mut().iter_projectees_and_ranges_mut() {
                if uad_prange.update_src_rad(ship_radius) {
                    projections_to_update.push((module_item_key, *projectee_item_key, *uad_prange));
                }
            }
            if let Some(charge_item_key) = uad_module.get_charge_item_key() {
                let uad_charge = self.uad.items.get_mut(charge_item_key).get_charge_mut().unwrap();
                for (projectee_item_key, uad_prange) in uad_charge.get_projs_mut().iter_projectees_and_ranges_mut() {
                    if uad_prange.update_src_rad(ship_radius) {
                        projections_to_update.push((charge_item_key, *projectee_item_key, *uad_prange));
                    }
                }
            }
        }
        for (projector_item_key, projectee_item_key, prange) in projections_to_update {
            let projectee_uad_item = self.uad.items.get(projectee_item_key);
            SolarSystem::util_change_item_proj_range(
                &self.uad,
                &mut self.svc,
                &self.reffs,
                projector_item_key,
                projectee_item_key,
                projectee_uad_item,
                Some(prange),
            );
        }
        // Update incoming projections
        for &projector_item_key in self.rprojs.iter_projectors(&item_key) {
            let projector_uad_item = self.uad.items.get_mut(projector_item_key);
            if let Some(uad_prange) = projector_uad_item.get_projs_mut().unwrap().get_mut_range(&item_key)
                && uad_prange.update_tgt_rad(ship_radius)
            {
                let uad_prange = Some(*uad_prange);
                let uad_item = self.uad.items.get(item_key);
                SolarSystem::util_change_item_proj_range(
                    &self.uad,
                    &mut self.svc,
                    &self.reffs,
                    projector_item_key,
                    item_key,
                    uad_item,
                    uad_prange,
                );
            }
        }
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_add_ship(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
    }
}

impl<'a> ShipMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        self.sol.internal_set_ship_a_item_id(self.key, type_id)
    }
}
