use crate::{
    ad::AItemId,
    api::{ItemTypeId, ShipMut},
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_ship_type_aid(
        &mut self,
        ship_uid: UItemId,
        type_aid: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(ship_uid);
        if u_item.get_type_aid() == type_aid {
            return;
        }
        SolarSystem::util_remove_ship(&mut self.u_data, &mut self.svc, ship_uid, reuse_eupdates);
        let u_ship = self.u_data.items.get_mut(ship_uid).dc_ship_mut().unwrap();
        let fit_uid = u_ship.get_fit_uid();
        u_ship.set_type_aid(type_aid, &self.u_data.src);
        // Update on-fit ship kind
        let ship_kind = u_ship.get_kind();
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        u_fit.ship_kind = ship_kind;
        // Update outgoing projections for all on-ship items
        let ship_radius = self.u_data.get_fit_ship_radius(fit_uid);
        SolarSystem::util_update_ship_radius_for_outgoing_projs(&mut self.u_data, &mut self.svc, fit_uid, ship_radius);
        // Update incoming projections
        for projector_uid in self.rev_projs.iter_projectors(&ship_uid) {
            let projector_u_item = self.u_data.items.get_mut(projector_uid);
            if let Some(u_proj_data) = projector_u_item.get_projs_mut().unwrap().get_proj_data_mut(&ship_uid)
                && u_proj_data.update_tgt_radius(ship_radius)
            {
                let u_proj_data = Some(*u_proj_data);
                SolarSystem::util_change_item_proj_data(
                    &self.u_data,
                    &mut self.svc,
                    projector_uid,
                    ship_uid,
                    u_proj_data,
                );
            }
        }
        SolarSystem::util_add_ship(&mut self.u_data, &mut self.svc, ship_uid, reuse_eupdates);
    }
}

impl<'a> ShipMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_ship_type_aid(self.uid, type_id.into_aid(), &mut reuse_eupdates)
    }
}
