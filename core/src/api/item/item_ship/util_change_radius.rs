use crate::{
    misc::PValue,
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UFitId},
};

impl SolarSystem {
    pub(in crate::api) fn util_update_ship_radius_for_outgoing_projs(
        u_data: &mut UData,
        svc: &mut Svc,
        fit_uid: UFitId,
        ship_radius: PValue,
    ) {
        let mut projections_to_update = Vec::new();
        for module_uid in u_data.fits.get(fit_uid).iter_module_uids() {
            let u_module = u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
            for (projectee_uid, u_proj_data) in u_module.get_projs_mut().iter_projectees_and_datas_mut() {
                if u_proj_data.update_src_radius(ship_radius) {
                    projections_to_update.push((module_uid, projectee_uid, *u_proj_data));
                }
            }
            if let Some(charge_uid) = u_module.get_charge_uid() {
                let u_charge = u_data.items.get_mut(charge_uid).dc_charge_mut().unwrap();
                for (projectee_uid, u_proj_data) in u_charge.get_projs_mut().iter_projectees_and_datas_mut() {
                    if u_proj_data.update_src_radius(ship_radius) {
                        projections_to_update.push((charge_uid, projectee_uid, *u_proj_data));
                    }
                }
            }
        }
        for (projector_uid, projectee_uid, proj_data) in projections_to_update {
            SolarSystem::util_change_item_proj_data(u_data, svc, projector_uid, projectee_uid, Some(proj_data));
        }
    }
}
