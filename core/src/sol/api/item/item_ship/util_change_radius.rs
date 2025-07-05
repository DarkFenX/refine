use itertools::chain;

use crate::{
    def::{AttrVal, FitKey},
    sol::{SolarSystem, reffs::REffs},
    svc::Svc,
    uad::Uad,
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_update_ship_radius_for_outgoing_projs(
        uad: &mut Uad,
        svc: &mut Svc,
        reffs: &REffs,
        fit_key: FitKey,
        ship_radius: AttrVal,
    ) {
        let mut projections_to_update = Vec::new();
        let uad_fit = uad.fits.get(fit_key);
        for &module_key in chain!(
            uad_fit.mods_high.iter_keys(),
            uad_fit.mods_mid.iter_keys(),
            uad_fit.mods_low.iter_keys()
        ) {
            let uad_module = uad.items.get_mut(module_key).get_module_mut().unwrap();
            for (projectee_key, uad_prange) in uad_module.get_projs_mut().iter_projectees_and_ranges_mut() {
                if uad_prange.update_src_rad(ship_radius) {
                    projections_to_update.push((module_key, projectee_key, *uad_prange));
                }
            }
            if let Some(charge_key) = uad_module.get_charge_key() {
                let uad_charge = uad.items.get_mut(charge_key).get_charge_mut().unwrap();
                for (projectee_key, uad_prange) in uad_charge.get_projs_mut().iter_projectees_and_ranges_mut() {
                    if uad_prange.update_src_rad(ship_radius) {
                        projections_to_update.push((charge_key, projectee_key, *uad_prange));
                    }
                }
            }
        }
        for (projector_key, projectee_key, prange) in projections_to_update {
            let projectee_uad_item = uad.items.get(projectee_key);
            SolarSystem::util_change_item_proj_range(
                uad,
                svc,
                reffs,
                projector_key,
                projectee_key,
                projectee_uad_item,
                Some(prange),
            );
        }
    }
}
