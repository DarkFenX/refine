use itertools::Itertools;

use crate::{
    sol::{RevProjs, SolarSystem},
    svc::Svc,
    ud::{UData, UItemId, UPhysics},
};

impl SolarSystem {
    pub(in crate::api) fn util_update_physics_for_incoming(
        u_data: &mut UData,
        rev_projs: &RevProjs,
        svc: &mut Svc,
        item_uid: UItemId,
        physics: UPhysics,
    ) {
        let projector_uids = rev_projs.iter_projectors(&item_uid);
        if projector_uids.len() > 0 {
            let projector_uids = projector_uids.collect_vec();
            for projector_uid in projector_uids {
                let projector_u_item = u_data.items.get_mut(projector_uid);
                if let Some(proj_data) = projector_u_item.get_projs_mut().unwrap().get_proj_data_mut(&item_uid) {
                    proj_data.update_tgt_physics(physics);
                    let proj_data = *proj_data;
                    SolarSystem::util_change_item_proj_data(u_data, svc, projector_uid, item_uid, Some(proj_data));
                }
            }
        }
    }
}
