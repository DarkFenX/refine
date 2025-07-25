use crate::{
    def::OF,
    sol::{RevProjs, SolarSystem},
    svc::Svc,
    ud::{UData, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_update_item_radius_in_projs(
        u_data: &mut UData,
        rev_projs: &RevProjs,
        svc: &mut Svc,
        item_key: UItemKey,
    ) {
        let u_item = u_data.items.get_mut(item_key);
        // Outgoing projections - service change should be handled in calling method
        let item_radius = u_item.get_r_axt().map(|v| v.radius).unwrap_or(OF(0.0));
        for u_prange in u_item.get_projs_mut().unwrap().iter_ranges_mut() {
            u_prange.update_src_rad(item_radius);
        }
        // Incoming projections
        for &projector_key in rev_projs.iter_projectors(&item_key) {
            let projector_u_item = u_data.items.get_mut(projector_key);
            if let Some(u_prange) = projector_u_item.get_projs_mut().unwrap().get_range_mut(&item_key)
                && u_prange.update_tgt_rad(item_radius)
            {
                let u_prange = Some(*u_prange);
                let projector_u_item = u_data.items.get(projector_key);
                let u_item = u_data.items.get(item_key);
                SolarSystem::util_change_item_proj_range(
                    u_data,
                    svc,
                    projector_key,
                    projector_u_item,
                    item_key,
                    u_item,
                    u_prange,
                );
            }
        }
    }
}
