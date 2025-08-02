use itertools::Itertools;

use crate::{
    rd::REffectKey,
    sol::{SolarSystem, rev_projs::RevProjs},
    svc::Svc,
    ud::{UAutocharge, UData, UEffectUpdates, UItem, UItemKey},
};

struct AutochargeData {
    effect_key: REffectKey,
    item_key: UItemKey,
}

impl SolarSystem {
    pub(in crate::sol::api) fn add_item_autocharges(
        u_data: &mut UData,
        svc: &mut Svc,
        rev_projs: &mut RevProjs,
        item_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // Process autocharges - start with collecting some data about parent item
        let u_item = u_data.items.get(item_key);
        if u_item.get_autocharges().is_none() || !u_item.is_loaded() {
            return;
        }
        // Add autocharge items themselves, and record which have been added
        let fit_key = match u_item.get_fit_key() {
            Some(fit_key) => fit_key,
            None => return,
        };
        let effects_with_ac_type_ids = u_item
            .get_effect_datas()
            .unwrap()
            .iter()
            .filter_map(|(effect_key, effect_data)| effect_data.autocharge.map(|ac_type_id| (*effect_key, ac_type_id)))
            .collect_vec();
        if effects_with_ac_type_ids.is_empty() {
            return;
        }
        let projections = match u_item.get_projs() {
            Some(projs) => projs.iter().collect(),
            None => Vec::new(),
        };
        let ac_datas = effects_with_ac_type_ids
            .into_iter()
            .filter_map(|(effect_key, ac_type_id)| {
                let ac_item_id = u_data.items.alloc_id();
                // Autocharge is activated only if effect controlling it is running, and activates
                // charges
                let activated = u_data.src.get_effect(effect_key).activates_autocharge()
                    && u_data
                        .items
                        .get(item_key)
                        .get_reffs()
                        .is_some_and(|v| v.contains(&effect_key));
                let mut u_ac = UAutocharge::new(
                    ac_item_id,
                    ac_type_id,
                    fit_key,
                    item_key,
                    effect_key,
                    activated,
                    false,
                    &u_data.src,
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
                    effect_key,
                })
            })
            .collect_vec();
        if ac_datas.is_empty() {
            return;
        }
        for ac_data in ac_datas.iter() {
            let ac_u_item = u_data.items.get_mut(ac_data.item_key);
            ac_u_item.update_reffs(reuse_eupdates, &u_data.src);
            SolarSystem::util_add_item_without_projs(u_data, svc, ac_data.item_key, reuse_eupdates);
            if !projections.is_empty() {
                let ac_u_item = u_data.items.get(ac_data.item_key);
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
        }
        // Update autocharge info on parent item
        let cont_u_item = u_data.items.get_mut(item_key);
        let cont_acs = cont_u_item.get_autocharges_mut().unwrap();
        for ac_data in ac_datas.into_iter() {
            cont_acs.set(ac_data.effect_key, ac_data.item_key);
        }
    }
    pub(in crate::sol::api) fn remove_item_autocharges(
        u_data: &mut UData,
        svc: &mut Svc,
        rev_projs: &mut RevProjs,
        item_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let cont_u_item = u_data.items.get(item_key);
        let ac_keys = match cont_u_item.get_autocharges() {
            Some(cont_acs) => cont_acs.values().copied().collect_vec(),
            None => return,
        };
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
            let ac_u_item = u_data.items.get_mut(ac_key);
            ac_u_item.stop_all_reffs(reuse_eupdates, &u_data.src);
            SolarSystem::util_remove_item_without_projs(u_data, svc, ac_key, reuse_eupdates);
        }
        // Update items
        let cont_u_item = u_data.items.get_mut(item_key);
        cont_u_item.get_autocharges_mut().unwrap().clear();
        for ac_key in ac_keys.into_iter() {
            u_data.items.remove(ac_key);
        }
    }
}
