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
    pub(in crate::sol::api) fn add_fighter_autocharges(
        svc: &mut Svc,
        uad: &mut Uad,
        proj_tracker: &mut ProjTracker,
        fighter_item_key: ItemKey,
    ) {
        // Process autocharges - start with collecting some data about fighter itself
        let uad_fighter = uad.items.get(fighter_item_key).get_fighter().unwrap();
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
                let uad_autocharge = UadAutocharge::new(
                    &uad.src,
                    autocharge_id,
                    autocharge_a_item_id,
                    fit_key,
                    fighter_item_key,
                    a_effect_id,
                    item_a_state,
                    false,
                );
                // Don't add an autocharge if it can't be loaded
                if !uad_autocharge.is_loaded() {
                    continue;
                }
                // Add autocharge item to user data and fill info vec
                let ac_uad_item = UadItem::Autocharge(uad_autocharge);
                let ac_key = uad.items.add(ac_uad_item);
                new_autocharges.push((a_effect_id, ac_key));
            }
        }
        if !new_autocharges.is_empty() {
            for (_, autocharge_key) in new_autocharges.iter() {
                // Add autocharges without projections to services
                let autocharge_uad_item = uad.items.get(*autocharge_key);
                svc.add_item(uad, *autocharge_key, autocharge_uad_item);
                if !projections.is_empty() {
                    // Transfer fighter projections to autocharge, and fill projection tracker
                    let uad_autocharge = uad.items.get_mut(*autocharge_key).get_autocharge_mut().unwrap();
                    for (projectee_key, range) in projections.iter() {
                        uad_autocharge.get_projs_mut().add(*projectee_key, *range);
                        proj_tracker.reg_projectee(*autocharge_key, *projectee_key);
                    }
                    // Register projections in services
                    let autocharge_uad_item = uad.items.get(*autocharge_key);
                    for (projectee_key, range) in projections.iter() {
                        let projectee_uad_item = uad.items.get(*projectee_key);
                        svc.add_item_projection(
                            uad,
                            *autocharge_key,
                            autocharge_uad_item,
                            *projectee_key,
                            projectee_uad_item,
                            *range,
                        )
                    }
                }
            }
            // Update on-fighter autocharge info
            let item_acs = uad
                .items
                .get_mut(fighter_item_key)
                .get_fighter_mut()
                .unwrap()
                .get_autocharges_mut();
            for (a_effect_id, autocharge_key) in new_autocharges.iter() {
                item_acs.set(*a_effect_id, *autocharge_key);
            }
        };
    }
    pub(in crate::sol::api) fn remove_fighter_autocharges(
        svc: &mut Svc,
        uad: &mut Uad,
        proj_tracker: &mut ProjTracker,
        fighter_item_key: ItemKey,
    ) {
        let autocharge_keys = uad
            .items
            .get(fighter_item_key)
            .get_fighter()
            .unwrap()
            .get_autocharges()
            .values()
            .copied()
            .collect_vec();
        for &autocharge_key in autocharge_keys.iter() {
            let autocharge_uad_item = uad.items.get(autocharge_key);
            let uad_autocharge = autocharge_uad_item.get_autocharge().unwrap();
            for &projectee_item_key in uad_autocharge.get_projs().iter_projectee_item_keys() {
                let projectee_uad_item = uad.items.get(projectee_item_key);
                // Remove projections from services
                svc.remove_item_projection(
                    uad,
                    autocharge_key,
                    autocharge_uad_item,
                    projectee_item_key,
                    projectee_uad_item,
                );
                // Update projection tracker (just because it's convenient to do it here)
                proj_tracker.unreg_projectee(&autocharge_key, &projectee_item_key);
            }
            // Clear up on-autocharge projections, so that they don't get processed during item
            // removal from services - they already were processed before
            uad.items
                .get_mut(autocharge_key)
                .get_autocharge_mut()
                .unwrap()
                .get_projs_mut()
                .clear();
            // Remove from services
            let autocharge_uad_item = uad.items.get(autocharge_key);
            svc.remove_item(uad, autocharge_key, autocharge_uad_item);
        }
        // Update items
        uad.items
            .get_mut(fighter_item_key)
            .get_fighter_mut()
            .unwrap()
            .get_autocharges_mut()
            .clear();
        for autocharge_key in autocharge_keys.into_iter() {
            uad.items.remove(autocharge_key);
        }
    }
}
