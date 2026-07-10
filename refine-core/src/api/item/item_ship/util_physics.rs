use crate::{
    sol::{RevProjs, SolarSystem},
    svc::Svc,
    ud::{UData, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn util_update_ship_physics(
        u_data: &mut UData,
        rev_projs: &RevProjs,
        svc: &mut Svc,
        ship_uid: UItemId,
    ) {
        let u_ship = u_data.items.get_mut(ship_uid).dc_ship_mut().unwrap();
        let u_ship_physics = *u_ship.get_physics();
        // Handle outgoing projections
        let mut projections_to_update = Vec::new();
        for module_uid in u_data.fits.get(u_ship.get_fit_uid()).iter_module_uids() {
            let u_module = u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
            for (projectee_uid, u_proj_data) in u_module.get_projs_mut().iter_projectees_and_datas_mut() {
                u_proj_data.update_src_physics(u_ship_physics);
                projections_to_update.push((module_uid, projectee_uid, *u_proj_data));
            }
            if let Some(charge_uid) = u_module.get_charge_uid() {
                let u_charge = u_data.items.get_mut(charge_uid).dc_charge_mut().unwrap();
                for (projectee_uid, u_proj_data) in u_charge.get_projs_mut().iter_projectees_and_datas_mut() {
                    u_proj_data.update_src_physics(u_ship_physics);
                    projections_to_update.push((charge_uid, projectee_uid, *u_proj_data));
                }
            }
        }
        for (projector_uid, projectee_uid, proj_data) in projections_to_update {
            SolarSystem::util_change_item_proj_data(u_data, svc, projector_uid, projectee_uid, Some(proj_data));
        }
        // Handle incoming projections
        SolarSystem::util_update_physics_for_incoming(u_data, rev_projs, svc, ship_uid, u_ship_physics);
    }
}
