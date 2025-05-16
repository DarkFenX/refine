use itertools::Itertools;

use crate::{
    ad,
    sol::{
        ItemKey, SolarSystem,
        proj_tracker::ProjTracker,
        svc::Svc,
        uad::{
            Uad,
            item::{UadAutocharge, UadItem},
        },
    },
};

impl SolarSystem {
    pub(in crate::sol::api) fn load_fighter(
        svc: &mut Svc,
        uad: &mut Uad,
        proj_tracker: &mut ProjTracker,
        item_key: ItemKey,
    ) {
        // Process fighter itself
        let uad_item = uad.items.get(item_key);
        let uad_fighter = uad_item.get_fighter().unwrap();
        svc.load_item(uad, item_key, uad_item);
        // Process autocharges - start with collecting some data about fighter itself
        let item_a_state = uad_fighter.get_a_state();
        let projections = uad_fighter
            .get_projs()
            .iter()
            .map(|(&projectee_item_key, &range)| (projectee_item_key, range))
            .collect_vec();
        // Add autocharge items themselves, and record which have been added
        let mut new_autocharges = Vec::new();
        if uad_fighter.is_loaded() {
            let fit_key = uad_fighter.get_fit_key();
            let effects_with_autocharges = uad_fighter
                .get_a_effect_datas()
                .unwrap()
                .keys()
                .filter_map(|&a_effect_id| {
                    let a_effect = uad.src.get_a_effect(&a_effect_id)?;
                    let autocharge_a_item_id = match a_effect.charge {
                        Some(ad::AEffectChargeInfo::Attr(charge_a_attr_id)) => uad_fighter
                            .get_a_attrs()
                            .unwrap()
                            .get(&charge_a_attr_id)?
                            .into_inner()
                            .round()
                            as ad::AItemId,
                        _ => return None,
                    };
                    Some((a_effect_id, autocharge_a_item_id))
                })
                .collect_vec();
            for (a_effect_id, autocharge_a_item_id) in effects_with_autocharges.into_iter() {
                let autocharge_id = uad.items.alloc_id();
                let mut autocharge = UadAutocharge::new(
                    &uad.src,
                    autocharge_id,
                    autocharge_a_item_id,
                    fit_key,
                    item_key,
                    a_effect_id,
                    item_a_state,
                    false,
                );
                // Don't add an autocharge if it can't be loaded
                if !autocharge.is_loaded() {
                    continue;
                }
                // Transfer parent item projections to autocharge
                for (projectee_key_id, range) in projections.iter() {
                    autocharge.get_projs_mut().add(*projectee_key_id, *range);
                }
                // Add autocharge item to user data and fill info vec
                let ac_item = UadItem::Autocharge(autocharge);
                let ac_key = uad.items.add(ac_item);
                new_autocharges.push((a_effect_id, ac_key));
            }
        }
        if !new_autocharges.is_empty() {
            // On=fighter container and projection tracker
            let item_acs = uad
                .items
                .get_mut(item_key)
                .get_fighter_mut()
                .unwrap()
                .get_autocharges_mut();
            for (a_effect_id, autocharge_key) in new_autocharges.iter() {
                item_acs.set(*a_effect_id, *autocharge_key);
                for (projectee_key_id, _) in projections.iter() {
                    proj_tracker.reg_projectee(*autocharge_key, *projectee_key_id);
                }
            }
            // Register autocharges in services
            for (_, autocharge_key) in new_autocharges.into_iter() {
                let ac_uad_item = uad.items.get(autocharge_key);
                svc.add_item(uad, autocharge_key, ac_uad_item);
            }
        };
    }
    pub(in crate::sol::api) fn unload_fighter(
        svc: &mut Svc,
        uad: &mut Uad,
        proj_tracker: &mut ProjTracker,
        item_key: ItemKey,
    ) {
        let autocharges = uad.items.get(item_key).get_fighter().unwrap().get_autocharges();
        let mut autocharge_keys = Vec::with_capacity(autocharges.len());
        for &autocharge_key in autocharges.values() {
            let autocharge_uad_item = uad.items.get(autocharge_key);
            let uad_autocharge = autocharge_uad_item.get_autocharge().unwrap();
            for &projectee_item_key in uad_autocharge.get_projs().iter_projectee_item_keys() {
                let projectee_uad_item = uad.items.get(projectee_item_key);
                // Update services
                svc.remove_item_projection(
                    &uad,
                    autocharge_key,
                    autocharge_uad_item,
                    projectee_item_key,
                    projectee_uad_item,
                );
                // Update user data for autocharge - don't touch data on charge itself, since charge
                // will be removed later anyway
                proj_tracker.unreg_projectee(&autocharge_key, &projectee_item_key);
            }
            // Remove from services
            svc.remove_item(&uad, autocharge_key, autocharge_uad_item);
            autocharge_keys.push(autocharge_key);
        }
        let fighter_uad_item = uad.items.get(item_key);
        svc.unload_item(uad, item_key, fighter_uad_item);
        // Update items
        uad.items
            .get_mut(item_key)
            .get_fighter_mut()
            .unwrap()
            .get_autocharges_mut()
            .clear();
        for autocharge_key in autocharge_keys {
            uad.items.remove(autocharge_key);
        }
    }
}
