use itertools::Itertools;

use crate::{
    rd::REffectId,
    sol::{RevProjs, SolarSystem},
    svc::Svc,
    ud::{UAutocharge, UData, UEffectUpdates, UItem, UItemId},
};

struct AutochargeData {
    effect_rid: REffectId,
    autocharge_uid: UItemId,
}

impl SolarSystem {
    pub(in crate::api::item) fn util_add_item_autocharges(
        u_data: &mut UData,
        svc: &mut Svc,
        rev_projs: &mut RevProjs,
        item_uid: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // Process autocharges - start with collecting some data about parent item
        let u_item = u_data.items.get(item_uid);
        if u_item.get_autocharges().is_none() || !u_item.is_loaded() {
            return;
        }
        // Add autocharge items themselves, and record which have been added
        let Some(fit_uid) = u_item.get_fit_uid() else {
            return;
        };
        let effects_with_ac_type_aids = u_item
            .get_r_item_base()
            .unwrap()
            .effects
            .iter()
            .filter_map(|(effect_rid, effect_data)| {
                effect_data.autocharge_aid.map(|ac_type_aid| (*effect_rid, ac_type_aid))
            })
            .collect_vec();
        if effects_with_ac_type_aids.is_empty() {
            return;
        }
        let projections = match u_item.get_projs() {
            Some(projs) => projs.iter().collect(),
            None => Vec::new(),
        };
        let ac_datas = effects_with_ac_type_aids
            .into_iter()
            .filter_map(|(effect_rid, ac_type_aid)| {
                let autocharge_item_id = u_data.items.alloc_id();
                // Autocharge is activated only if effect controlling it is running, and activates
                // charges
                let activated = u_data.r_data.get_effect_by_rid(effect_rid).activates_autocharge()
                    && u_data
                        .items
                        .get(item_uid)
                        .get_reffs()
                        .is_some_and(|v| v.contains(&effect_rid));
                let u_autocharge = UAutocharge::new(
                    autocharge_item_id,
                    ac_type_aid,
                    fit_uid,
                    item_uid,
                    effect_rid,
                    activated,
                    false,
                    &u_data.r_data,
                );
                // Don't add an autocharge if it can't be loaded
                if !u_autocharge.is_loaded() {
                    return None;
                }
                let autocharge_u_item = UItem::Autocharge(u_autocharge);
                let autocharge_uid = u_data.items.add(autocharge_u_item);
                Some(AutochargeData {
                    effect_rid,
                    autocharge_uid,
                })
            })
            .collect_vec();
        if ac_datas.is_empty() {
            return;
        }
        for ac_data in ac_datas.iter() {
            let autocharge_u_item = u_data.items.get_mut(ac_data.autocharge_uid);
            autocharge_u_item.update_reffs(reuse_eupdates, &u_data.r_data);
            SolarSystem::util_add_item(u_data, svc, ac_data.autocharge_uid, reuse_eupdates);
            if !projections.is_empty() {
                let u_autocharge = u_data
                    .items
                    .get_mut(ac_data.autocharge_uid)
                    .dc_autocharge_mut()
                    .unwrap();
                for (projectee_uid, range) in projections.iter() {
                    u_autocharge.get_projs_mut().add(*projectee_uid, *range);
                    rev_projs.reg_projectee(ac_data.autocharge_uid, *projectee_uid);
                }
                let u_autocharge = u_data.items.get(ac_data.autocharge_uid).dc_autocharge().unwrap();
                for (projectee_uid, range) in u_autocharge.get_projs().iter() {
                    SolarSystem::util_add_item_projection(u_data, svc, ac_data.autocharge_uid, projectee_uid, range);
                }
            }
        }
        // Update autocharge info on parent item
        let cont_u_item = u_data.items.get_mut(item_uid);
        let cont_acs = cont_u_item.get_autocharges_mut().unwrap();
        for ac_data in ac_datas.into_iter() {
            cont_acs.set(ac_data.effect_rid, ac_data.autocharge_uid);
        }
    }
    pub(in crate::api::item) fn util_remove_item_autocharges(
        u_data: &mut UData,
        svc: &mut Svc,
        rev_projs: &mut RevProjs,
        item_uid: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        SolarSystem::util_detach_item_autocharges(u_data, svc, rev_projs, item_uid, reuse_eupdates);
        // Update items
        let cont_u_item = u_data.items.get_mut(item_uid);
        let Some(cont_acs) = cont_u_item.get_autocharges_mut() else {
            return;
        };
        let autocharge_uids = cont_acs.values().collect_vec();
        cont_acs.clear();
        for ac_uid in autocharge_uids.into_iter() {
            u_data.items.remove(ac_uid);
        }
    }
    pub(in crate::api::item) fn util_detach_item_autocharges(
        u_data: &mut UData,
        svc: &mut Svc,
        rev_projs: &mut RevProjs,
        item_uid: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let autocharge_uids = match u_data.items.get(item_uid).get_autocharges() {
            Some(cont_acs) => cont_acs.values().collect_vec(),
            None => return,
        };
        for autocharge_uid in autocharge_uids.into_iter() {
            SolarSystem::util_unreg_item_projs(u_data, svc, rev_projs, autocharge_uid);
            let autocharge_u_item = u_data.items.get_mut(autocharge_uid);
            autocharge_u_item.stop_all_reffs(reuse_eupdates, &u_data.r_data);
            SolarSystem::util_remove_item(u_data, svc, autocharge_uid, reuse_eupdates);
        }
    }
    /// Adds autocharges an item has in user data to services and to reverse projection tracker.
    pub(in crate::api::item) fn util_reattach_item_autocharges(
        u_data: &mut UData,
        svc: &mut Svc,
        rev_projs: &mut RevProjs,
        item_uid: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let autocharge_uids = match u_data.items.get(item_uid).get_autocharges() {
            Some(cont_acs) => cont_acs.values().collect_vec(),
            None => return,
        };
        for autocharge_uid in autocharge_uids.into_iter() {
            SolarSystem::util_reattach_autocharge(u_data, svc, rev_projs, autocharge_uid, reuse_eupdates);
        }
    }
    pub(super) fn util_reattach_autocharge(
        u_data: &mut UData,
        svc: &mut Svc,
        rev_projs: &mut RevProjs,
        autocharge_uid: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let autocharge_u_item = u_data.items.get_mut(autocharge_uid);
        autocharge_u_item.update_reffs(reuse_eupdates, &u_data.r_data);
        SolarSystem::util_add_item(u_data, svc, autocharge_uid, reuse_eupdates);
        SolarSystem::util_reg_item_projs(u_data, svc, rev_projs, autocharge_uid);
    }
}
