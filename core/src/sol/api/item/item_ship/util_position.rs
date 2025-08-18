use crate::{
    sol::{SolarSystem, rev_projs::RevProjs},
    svc::Svc,
    ud::{UData, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_update_ship_position(
        u_data: &mut UData,
        rev_projs: &RevProjs,
        svc: &mut Svc,
        ship_key: UItemKey,
    ) {
        let u_ship = u_data.items.get_mut(ship_key).get_ship_mut().unwrap();
        let u_ship_pos = *u_ship.get_position();
        // Handle outgoing projections
        let mut projections_to_update = Vec::new();
        for module_key in u_data.fits.get(u_ship.get_fit_key()).iter_module_keys() {
            let u_module = u_data.items.get_mut(module_key).get_module_mut().unwrap();
            for (projectee_key, u_proj_data) in u_module.get_projs_mut().iter_projectees_and_datas_mut() {
                u_proj_data.update_src_pos(u_ship_pos);
                projections_to_update.push((module_key, projectee_key, *u_proj_data));
            }
            if let Some(charge_key) = u_module.get_charge_key() {
                let u_charge = u_data.items.get_mut(charge_key).get_charge_mut().unwrap();
                for (projectee_key, u_proj_data) in u_charge.get_projs_mut().iter_projectees_and_datas_mut() {
                    u_proj_data.update_src_pos(u_ship_pos);
                    projections_to_update.push((charge_key, projectee_key, *u_proj_data));
                }
            }
        }
        for (projector_key, projectee_key, proj_data) in projections_to_update {
            SolarSystem::util_change_item_proj_data(u_data, svc, projector_key, projectee_key, Some(proj_data));
        }
        // Handle incoming projections
        SolarSystem::util_update_position_for_incoming(u_data, rev_projs, svc, ship_key, u_ship_pos);
    }
}
