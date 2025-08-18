use itertools::Itertools;

use crate::{
    sol::{SolarSystem, rev_projs::RevProjs},
    svc::Svc,
    ud::{UData, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_update_fighter_position(
        u_data: &mut UData,
        rev_projs: &RevProjs,
        svc: &mut Svc,
        fighter_key: UItemKey,
    ) {
        let u_fighter = u_data.items.get_mut(fighter_key).get_fighter_mut().unwrap();
        let u_position = *u_fighter.get_position();
        if !u_fighter.get_projs_mut().is_empty() {
            // Handle outgoing projections for fighter itself
            for u_proj_data in u_fighter.get_projs_mut().iter_datas_mut() {
                u_proj_data.update_src_pos(u_position);
            }
            let u_fighter = u_data.items.get(fighter_key).get_fighter().unwrap();
            for (projectee_key, u_proj_data) in u_fighter.get_projs().iter_projectees_and_datas() {
                SolarSystem::util_change_item_proj_data(u_data, svc, fighter_key, projectee_key, Some(u_proj_data));
            }
            // Handle outgoing projections for autocharges itself
            let autocharge_keys = u_fighter.get_autocharges().values().collect_vec();
            for autocharge_key in autocharge_keys {
                let u_autocharge = u_data.items.get_mut(autocharge_key).get_autocharge_mut().unwrap();
                for u_proj_data in u_autocharge.get_projs_mut().iter_datas_mut() {
                    u_proj_data.update_src_pos(u_position);
                }
                let u_autocharge = u_data.items.get(fighter_key).get_autocharge().unwrap();
                for (projectee_key, u_proj_data) in u_autocharge.get_projs().iter_projectees_and_datas() {
                    SolarSystem::util_change_item_proj_data(u_data, svc, fighter_key, projectee_key, Some(u_proj_data));
                }
            }
        }
        // Handle incoming projections
        SolarSystem::util_update_position_for_incoming(u_data, rev_projs, svc, fighter_key, u_position);
    }
}
