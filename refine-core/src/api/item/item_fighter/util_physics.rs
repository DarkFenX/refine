use itertools::Itertools;

use crate::{
    sol::{RevProjs, SolarSystem},
    svc::Svc,
    ud::{UData, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn util_update_fighter_physics(
        u_data: &mut UData,
        rev_projs: &RevProjs,
        svc: &mut Svc,
        fighter_uid: UItemId,
    ) {
        let u_fighter = u_data.items.get_mut(fighter_uid).dc_fighter_mut().unwrap();
        let u_physics = *u_fighter.get_physics();
        if !u_fighter.get_projs_mut().is_empty() {
            // Handle outgoing projections for fighter itself
            for u_proj_data in u_fighter.get_projs_mut().iter_datas_mut() {
                u_proj_data.update_src_physics(u_physics);
            }
            let u_fighter = u_data.items.get(fighter_uid).dc_fighter().unwrap();
            for (projectee_uid, u_proj_data) in u_fighter.get_projs().iter_projectees_and_datas() {
                SolarSystem::util_change_item_proj_data(u_data, svc, fighter_uid, projectee_uid, Some(u_proj_data));
            }
            // Handle outgoing projections for autocharges itself
            let autocharge_uids = u_fighter.get_autocharges().values().collect_vec();
            for autocharge_uid in autocharge_uids {
                let u_autocharge = u_data.items.get_mut(autocharge_uid).dc_autocharge_mut().unwrap();
                for u_proj_data in u_autocharge.get_projs_mut().iter_datas_mut() {
                    u_proj_data.update_src_physics(u_physics);
                }
                let u_autocharge = u_data.items.get(fighter_uid).dc_autocharge().unwrap();
                for (projectee_uid, u_proj_data) in u_autocharge.get_projs().iter_projectees_and_datas() {
                    SolarSystem::util_change_item_proj_data(u_data, svc, fighter_uid, projectee_uid, Some(u_proj_data));
                }
            }
        }
        // Handle incoming projections
        SolarSystem::util_update_physics_for_incoming(u_data, rev_projs, svc, fighter_uid, u_physics);
    }
}
