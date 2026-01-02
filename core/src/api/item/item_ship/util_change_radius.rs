use crate::{
    def::AttrVal,
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UFitId},
};

impl SolarSystem {
    pub(in crate::api) fn util_update_ship_radius_for_outgoing_projs(
        u_data: &mut UData,
        svc: &mut Svc,
        fit_key: UFitId,
        ship_radius: AttrVal,
    ) {
        let mut projections_to_update = Vec::new();
        for module_key in u_data.fits.get(fit_key).iter_module_keys() {
            let u_module = u_data.items.get_mut(module_key).dc_module_mut().unwrap();
            for (projectee_key, u_proj_data) in u_module.get_projs_mut().iter_projectees_and_datas_mut() {
                if u_proj_data.update_src_radius(ship_radius) {
                    projections_to_update.push((module_key, projectee_key, *u_proj_data));
                }
            }
            if let Some(charge_key) = u_module.get_charge_key() {
                let u_charge = u_data.items.get_mut(charge_key).dc_charge_mut().unwrap();
                for (projectee_key, u_proj_data) in u_charge.get_projs_mut().iter_projectees_and_datas_mut() {
                    if u_proj_data.update_src_radius(ship_radius) {
                        projections_to_update.push((charge_key, projectee_key, *u_proj_data));
                    }
                }
            }
        }
        for (projector_key, projectee_key, proj_data) in projections_to_update {
            SolarSystem::util_change_item_proj_data(u_data, svc, projector_key, projectee_key, Some(proj_data));
        }
    }
}
