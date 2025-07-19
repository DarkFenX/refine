use itertools::Itertools;

use crate::{
    ad,
    def::ItemKey,
    sol::{SolarSystem, rprojs::RProjs},
    svc::Svc,
    uad::{Uad, UadAutocharge, UadEffectUpdates, UadItem},
};

struct AutochargeData {
    a_effect_id: ad::AEffectId,
    item_key: ItemKey,
    eupdates: UadEffectUpdates,
}

impl SolarSystem {
    pub(in crate::sol::api) fn add_fighter_autocharges(
        uad: &mut Uad,
        svc: &mut Svc,
        rprojs: &mut RProjs,
        fighter_key: ItemKey,
    ) {
        // Process autocharges - start with collecting some data about fighter itself
        let uad_fighter = uad.items.get(fighter_key).get_fighter().unwrap();
        // Add autocharge items themselves, and record which have been added
        if !uad_fighter.is_loaded() {
            return;
        }
        let fit_key = uad_fighter.get_fit_key();
        let fighter_a_state = uad_fighter.get_a_state();
        let effects_with_ac_a_item_ids = uad_fighter
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
        let projections = uad_fighter.get_projs().iter().collect_vec();
        let mut ac_datas = effects_with_ac_a_item_ids
            .into_iter()
            .filter_map(|(a_effect_id, ac_a_item_id)| {
                let ac_item_id = uad.items.alloc_id();
                let mut ac_eupdates = UadEffectUpdates::new();
                let mut uad_ac = UadAutocharge::new(
                    ac_item_id,
                    ac_a_item_id,
                    fit_key,
                    fighter_key,
                    a_effect_id,
                    fighter_a_state,
                    false,
                    &uad.src,
                    &mut ac_eupdates,
                );
                // Don't add an autocharge if it can't be loaded
                if !uad_ac.is_loaded() {
                    return None;
                }
                // Set projections right away, they will be ignored during autocharge addition to
                // services anyway
                for (projectee_key, range) in projections.iter() {
                    uad_ac.get_projs_mut().add(*projectee_key, *range);
                }
                // Add autocharge item to user data and fill info vec
                let ac_uad_item = UadItem::Autocharge(uad_ac);
                let ac_key = uad.items.add(ac_uad_item);
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
            let ac_uad_item = uad.items.get(ac_data.item_key);
            SolarSystem::util_add_item_without_projs(uad, svc, ac_data.item_key, ac_uad_item, &ac_data.eupdates);
            for (projectee_key, range) in projections.iter() {
                let projectee_uad_item = uad.items.get(*projectee_key);
                SolarSystem::util_add_item_projection(
                    uad,
                    svc,
                    ac_data.item_key,
                    ac_uad_item,
                    *projectee_key,
                    projectee_uad_item,
                    *range,
                );
                rprojs.reg_projectee(ac_data.item_key, *projectee_key);
            }
        }
        // Update on-fighter autocharge info
        let fighter_acs = uad
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
        uad: &mut Uad,
        svc: &mut Svc,
        rprojs: &mut RProjs,
        fighter_key: ItemKey,
        clear_fighter_acs: bool,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        let ac_keys = uad
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
            let ac_uad_item = uad.items.get(ac_key);
            let uad_ac = ac_uad_item.get_autocharge().unwrap();
            for projectee_key in uad_ac.get_projs().iter_projectees() {
                // Remove projections from services
                let projectee_uad_item = uad.items.get(projectee_key);
                SolarSystem::util_remove_item_projection(
                    uad,
                    svc,
                    ac_key,
                    ac_uad_item,
                    projectee_key,
                    projectee_uad_item,
                );
                // Update reverse projections (just because it's convenient to do it here)
                rprojs.unreg_projectee(&ac_key, &projectee_key);
            }
            // Remove from services
            SolarSystem::util_remove_item_without_projs(uad, svc, ac_key, ac_uad_item, reuse_eupdates);
        }
        // Update items
        if clear_fighter_acs {
            uad.items
                .get_mut(fighter_key)
                .get_fighter_mut()
                .unwrap()
                .get_autocharges_mut()
                .clear();
        }
        for ac_key in ac_keys.into_iter() {
            uad.items.remove(ac_key);
        }
    }
}
