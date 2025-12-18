use itertools::Itertools;

use crate::{
    rd::REffectKey,
    sol::{RevProjs, SolarSystem},
    svc::Svc,
    ud::{UAutocharge, UData, UEffectUpdates, UItem, UItemKey},
};

struct AutochargeData {
    effect_key: REffectKey,
    autocharge_key: UItemKey,
}

impl SolarSystem {
    pub(in crate::api) fn add_item_autocharges(
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
                let autocharge_item_id = u_data.items.alloc_id();
                // Autocharge is activated only if effect controlling it is running, and activates
                // charges
                let activated = u_data.src.get_effect(effect_key).activates_autocharge()
                    && u_data
                        .items
                        .get(item_key)
                        .get_reffs()
                        .is_some_and(|v| v.contains(&effect_key));
                let u_autocharge = UAutocharge::new(
                    autocharge_item_id,
                    ac_type_id,
                    fit_key,
                    item_key,
                    effect_key,
                    activated,
                    false,
                    &u_data.src,
                );
                // Don't add an autocharge if it can't be loaded
                if !u_autocharge.is_loaded() {
                    return None;
                }
                let autocharge_u_item = UItem::Autocharge(u_autocharge);
                let autocharge_key = u_data.items.add(autocharge_u_item);
                Some(AutochargeData {
                    effect_key,
                    autocharge_key,
                })
            })
            .collect_vec();
        if ac_datas.is_empty() {
            return;
        }
        for ac_data in ac_datas.iter() {
            let autocharge_u_item = u_data.items.get_mut(ac_data.autocharge_key);
            autocharge_u_item.update_reffs(reuse_eupdates, &u_data.src);
            SolarSystem::util_add_item(u_data, svc, ac_data.autocharge_key, reuse_eupdates);
            if !projections.is_empty() {
                let u_autocharge = u_data
                    .items
                    .get_mut(ac_data.autocharge_key)
                    .dc_autocharge_mut()
                    .unwrap();
                for (projectee_key, range) in projections.iter() {
                    u_autocharge.get_projs_mut().add(*projectee_key, *range);
                    rev_projs.reg_projectee(ac_data.autocharge_key, *projectee_key);
                }
                let u_autocharge = u_data.items.get(ac_data.autocharge_key).dc_autocharge().unwrap();
                for (projectee_key, range) in u_autocharge.get_projs().iter() {
                    SolarSystem::util_add_item_projection(u_data, svc, ac_data.autocharge_key, projectee_key, range);
                }
            }
        }
        // Update autocharge info on parent item
        let cont_u_item = u_data.items.get_mut(item_key);
        let cont_acs = cont_u_item.get_autocharges_mut().unwrap();
        for ac_data in ac_datas.into_iter() {
            cont_acs.set(ac_data.effect_key, ac_data.autocharge_key);
        }
    }
    pub(in crate::api) fn remove_item_autocharges(
        u_data: &mut UData,
        svc: &mut Svc,
        rev_projs: &mut RevProjs,
        item_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let cont_u_item = u_data.items.get(item_key);
        let autocharge_keys = match cont_u_item.get_autocharges() {
            Some(cont_acs) => cont_acs.values().collect_vec(),
            None => return,
        };
        if autocharge_keys.is_empty() {
            return;
        }
        for &autocharge_key in autocharge_keys.iter() {
            let u_autocharge = u_data.items.get(autocharge_key).dc_autocharge().unwrap();
            if !u_autocharge.get_projs().is_empty() {
                for projectee_key in u_autocharge.get_projs().iter_projectees() {
                    // Remove projections from services
                    SolarSystem::util_remove_item_projection(u_data, svc, autocharge_key, projectee_key);
                    // Update reverse projections (just because it's convenient to do it here)
                    rev_projs.unreg_projectee(&autocharge_key, projectee_key);
                }
                let u_autocharge = u_data.items.get_mut(autocharge_key).dc_autocharge_mut().unwrap();
                u_autocharge.get_projs_mut().clear();
            }
            // Remove from services
            let autocharge_u_item = u_data.items.get_mut(autocharge_key);
            autocharge_u_item.stop_all_reffs(reuse_eupdates, &u_data.src);
            SolarSystem::util_remove_item(u_data, svc, autocharge_key, reuse_eupdates);
        }
        // Update items
        let cont_u_item = u_data.items.get_mut(item_key);
        cont_u_item.get_autocharges_mut().unwrap().clear();
        for ac_key in autocharge_keys.into_iter() {
            u_data.items.remove(ac_key);
        }
    }
}
