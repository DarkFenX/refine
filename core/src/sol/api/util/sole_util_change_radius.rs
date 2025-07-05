use crate::{
    def::{ItemKey, OF},
    sol::{REffs, RProjs, SolarSystem},
    svc::Svc,
    uad::Uad,
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_update_item_radius_in_projs(
        uad: &mut Uad,
        rprojs: &RProjs,
        svc: &mut Svc,
        reffs: &REffs,
        item_key: ItemKey,
    ) {
        let uad_item = uad.items.get_mut(item_key);
        // Outgoing projections - service change should be handled in calling method
        let item_radius = uad_item.get_a_extras().and_then(|v| v.radius).unwrap_or(OF(0.0));
        for uad_prange in uad_item.get_projs_mut().unwrap().iter_ranges_mut() {
            uad_prange.update_src_rad(item_radius);
        }
        // Incoming projections
        for &projector_key in rprojs.iter_projectors(&item_key) {
            let projector_uad_item = uad.items.get_mut(projector_key);
            if let Some(uad_prange) = projector_uad_item.get_projs_mut().unwrap().get_range_mut(&item_key)
                && uad_prange.update_tgt_rad(item_radius)
            {
                let uad_prange = Some(*uad_prange);
                let uad_item = uad.items.get(item_key);
                SolarSystem::util_change_item_proj_range(
                    uad,
                    svc,
                    reffs,
                    projector_key,
                    item_key,
                    uad_item,
                    uad_prange,
                );
            }
        }
    }
}
