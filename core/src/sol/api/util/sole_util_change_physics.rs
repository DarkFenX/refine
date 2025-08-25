use itertools::Itertools;

use crate::{
    sol::{RevProjs, SolarSystem},
    svc::Svc,
    ud::{UData, UItemKey, UPhysics},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_update_physics_for_incoming(
        u_data: &mut UData,
        rev_projs: &RevProjs,
        svc: &mut Svc,
        item_key: UItemKey,
        physics: UPhysics,
    ) {
        let projector_keys = rev_projs.iter_projectors(&item_key);
        if !projector_keys.is_empty() {
            let projector_keys = projector_keys.collect_vec();
            for projector_key in projector_keys {
                let projector_u_item = u_data.items.get_mut(projector_key);
                if let Some(proj_data) = projector_u_item.get_projs_mut().unwrap().get_proj_data_mut(&item_key) {
                    proj_data.update_tgt_physics(physics);
                    let proj_data = *proj_data;
                    SolarSystem::util_change_item_proj_data(u_data, svc, projector_key, item_key, Some(proj_data));
                }
            }
        }
    }
}
