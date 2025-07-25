use itertools::Itertools;

use crate::{
    ad,
    sol::{SolarSystem, rev_projs::RevProjs},
    svc::Svc,
    ud::{UAutocharge, UData, UEffectUpdates, UItem, UItemKey},
};

struct AutochargeData {
    a_effect_id: ad::AEffectId,
    item_key: UItemKey,
    eupdates: UEffectUpdates,
}

impl SolarSystem {
    pub(in crate::sol::api) fn add_fighter_autocharges(
        u_data: &mut UData,
        svc: &mut Svc,
        rev_projs: &mut RevProjs,
        fighter_key: UItemKey,
    ) {
        // Process autocharges - start with collecting some data about fighter itself
        let u_fighter = u_data.items.get(fighter_key).get_fighter().unwrap();
        // Add autocharge items themselves, and record which have been added
        if !u_fighter.is_loaded() {
            return;
        }
        let fit_key = u_fighter.get_fit_key();
        let fighter_a_state = u_fighter.get_a_state();
        let effects_with_ac_a_item_ids = u_fighter
            .get_a_effect_datas()
            .unwrap()
            .iter()
            .filter_map(|(a_effect_id, a_effect_data)| {
                a_effect_data
                    .autocharge
                    .map(|ac_a_item_id| (*a_effect_id, ac_a_item_id))
            })
            .collect_vec();
        if effects_with_ac_a_item_ids.is_empty() {
            return;
        }
        let projections = u_fighter.get_projs().iter().collect_vec();
        let mut ac_datas = effects_with_ac_a_item_ids
            .into_iter()
            .filter_map(|(a_effect_id, ac_a_item_id)| {
                let ac_item_id = u_data.items.alloc_id();
                let mut ac_eupdates = UEffectUpdates::new();
                let mut u_ac = UAutocharge::new(
                    ac_item_id,
                    ac_a_item_id,
                    fit_key,
                    fighter_key,
                    a_effect_id,
                    fighter_a_state,
                    false,
                    &u_data.src,
                    &mut ac_eupdates,
                );
                // Don't add an autocharge if it can't be loaded
                if !u_ac.is_loaded() {
                    return None;
                }
                // Set projections right away, they will be ignored during autocharge addition to
                // services anyway
                for (projectee_key, range) in projections.iter() {
                    u_ac.get_projs_mut().add(*projectee_key, *range);
                }
                // Add autocharge item to user data and fill info vec
                let ac_u_item = UItem::Autocharge(u_ac);
                let ac_key = u_data.items.add(ac_u_item);
                Some(AutochargeData {
                    item_key: ac_key,
                    a_effect_id,
                    eupdates: ac_eupdates,
                })
            })
            .collect_vec();
        if ac_datas.is_empty() {
            return;
        }
        for ac_data in ac_datas.iter_mut() {
            let ac_u_item = u_data.items.get(ac_data.item_key);
            SolarSystem::util_add_item_without_projs(u_data, svc, ac_data.item_key, ac_u_item, &ac_data.eupdates);
            for (projectee_key, range) in projections.iter() {
                let projectee_u_item = u_data.items.get(*projectee_key);
                SolarSystem::util_add_item_projection(
                    u_data,
                    svc,
                    ac_data.item_key,
                    ac_u_item,
                    *projectee_key,
                    projectee_u_item,
                    *range,
                );
                rev_projs.reg_projectee(ac_data.item_key, *projectee_key);
            }
        }
        // Update on-fighter autocharge info
        let fighter_acs = u_data
            .items
            .get_mut(fighter_key)
            .get_fighter_mut()
            .unwrap()
            .get_autocharges_mut();
        for ac_data in ac_datas.into_iter() {
            fighter_acs.set(ac_data.a_effect_id, ac_data.item_key);
        }
    }
    pub(in crate::sol::api) fn remove_fighter_autocharges(
        u_data: &mut UData,
        svc: &mut Svc,
        rev_projs: &mut RevProjs,
        fighter_key: UItemKey,
        clear_fighter_acs: bool,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let ac_keys = u_data
            .items
            .get(fighter_key)
            .get_fighter()
            .unwrap()
            .get_autocharges()
            .values()
            .copied()
            .collect_vec();
        if ac_keys.is_empty() {
            return;
        }
        for &ac_key in ac_keys.iter() {
            let ac_u_item = u_data.items.get(ac_key);
            let u_ac = ac_u_item.get_autocharge().unwrap();
            for projectee_key in u_ac.get_projs().iter_projectees() {
                // Remove projections from services
                let projectee_u_item = u_data.items.get(projectee_key);
                SolarSystem::util_remove_item_projection(
                    u_data,
                    svc,
                    ac_key,
                    ac_u_item,
                    projectee_key,
                    projectee_u_item,
                );
                // Update reverse projections (just because it's convenient to do it here)
                rev_projs.unreg_projectee(&ac_key, &projectee_key);
            }
            // Remove from services
            SolarSystem::util_remove_item_without_projs(u_data, svc, ac_key, ac_u_item, reuse_eupdates);
        }
        // Update items
        if clear_fighter_acs {
            u_data
                .items
                .get_mut(fighter_key)
                .get_fighter_mut()
                .unwrap()
                .get_autocharges_mut()
                .clear();
        }
        for ac_key in ac_keys.into_iter() {
            u_data.items.remove(ac_key);
        }
    }
}
