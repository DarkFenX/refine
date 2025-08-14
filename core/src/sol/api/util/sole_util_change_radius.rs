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
        let item_radius = u_item.get_axt().map(|v| v.radius).unwrap_or(OF(0.0));
        for u_proj_data in u_item.get_projs_mut().unwrap().iter_datas_mut() {
            u_proj_data.update_src_rad(item_radius);
        }
        // Incoming projections
        for projector_key in rev_projs.iter_projectors(&item_key) {
            let projector_u_item = u_data.items.get_mut(projector_key);
            if let Some(u_proj_data) = projector_u_item.get_projs_mut().unwrap().get_proj_data_mut(&item_key)
                && u_proj_data.update_tgt_rad(item_radius)
            {
                let u_proj_data = Some(*u_proj_data);
                SolarSystem::util_change_item_proj_data(u_data, svc, projector_key, item_key, u_proj_data);
            }
        }
    }
}
