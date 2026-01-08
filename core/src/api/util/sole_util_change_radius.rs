use crate::{
    misc::PValue,
    sol::{RevProjs, SolarSystem},
    svc::Svc,
    ud::{UData, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn util_update_item_radius_in_projs(
        u_data: &mut UData,
        rev_projs: &RevProjs,
        svc: &mut Svc,
        item_uid: UItemId,
    ) {
        let u_item = u_data.items.get_mut(item_uid);
        // Outgoing projections - service change should be handled in calling method
        let item_radius = u_item.get_axt().map(|v| v.radius).unwrap_or(PValue::ZERO);
        for u_proj_data in u_item.get_projs_mut().unwrap().iter_datas_mut() {
            u_proj_data.update_src_radius(item_radius);
        }
        // Incoming projections
        for projector_uid in rev_projs.iter_projectors(&item_uid) {
            let projector_u_item = u_data.items.get_mut(projector_uid);
            if let Some(u_proj_data) = projector_u_item.get_projs_mut().unwrap().get_proj_data_mut(&item_uid)
                && u_proj_data.update_tgt_radius(item_radius)
            {
                let u_proj_data = Some(*u_proj_data);
                SolarSystem::util_change_item_proj_data(u_data, svc, projector_uid, item_uid, u_proj_data);
            }
        }
    }
}
