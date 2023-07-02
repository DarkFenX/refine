use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    ad,
    adg::{GData, GSupport},
    defs::{EEffectId, EItemId},
    ec, ed,
    util::Named,
};

pub(in crate::adg::conv) fn conv_items(g_data: &GData, g_supp: &GSupport) -> Vec<ad::AItem> {
    // Auxiliary maps
    let defeff_map = g_data
        .item_effects
        .iter()
        .filter(|v| v.is_default)
        .map(|v| (v.item_id, v.effect_id))
        .collect::<HashMap<EItemId, EEffectId>>();
    let mut a_item_map = HashMap::new();
    for e_item in g_data.items.iter() {
        // Item category ID
        let cat_id = match g_supp.grp_cat_map.get(&e_item.group_id) {
            Some(&cid) => cid,
            None => {
                let msg = format!("unable to find category ID for {}", e_item);
                tracing::warn!("{msg}");
                continue;
            }
        };
        // Item default effect
        let defeff_id = defeff_map.get(&e_item.id).map(|v| *v);
        // Item construction
        let a_item = ad::AItem::new(
            e_item.id,
            None,
            e_item.group_id,
            cat_id,
            HashMap::new(),
            HashMap::new(),
            defeff_id,
            HashMap::new(),
        );
        a_item_map.insert(a_item.id, a_item);
    }
    // Item attributes
    for e_item_attr in g_data.item_attrs.iter() {
        a_item_map
            .get_mut(&e_item_attr.item_id)
            .and_then(|v| v.attr_vals.insert(e_item_attr.attr_id, e_item_attr.value));
    }
    // Item effects & extended effect data from abilities
    for e_item_effect in g_data.item_effects.iter() {
        a_item_map.get_mut(&e_item_effect.item_id).and_then(|v| {
            v.effect_datas
                .insert(e_item_effect.effect_id, ad::AItemEffData::new(None, None, None))
        });
    }
    for e_item_abil in g_data.item_abils.iter() {
        match a_item_map.get_mut(&e_item_abil.item_id) {
            None => continue,
            Some(a_item) => match ec::abils::get_abil_effect(e_item_abil.abil_id) {
                None => continue,
                Some(effect_id) => match a_item.effect_datas.get_mut(&effect_id) {
                    None => continue,
                    Some(a_item_eff_data) => {
                        a_item_eff_data.cd = e_item_abil.cooldown;
                        a_item_eff_data.charge_amount = e_item_abil.charge_count;
                        a_item_eff_data.charge_reload_time = e_item_abil.charge_rearm_time;
                    }
                },
            },
        }
    }
    // Item skill requirements
    for e_item_srq in g_data.item_srqs.iter() {
        a_item_map
            .get_mut(&e_item_srq.item_id)
            .and_then(|v| v.srqs.insert(e_item_srq.skill_id, e_item_srq.level));
    }
    // Item type
    let mut a_items = Vec::new();
    for mut a_item in a_item_map.into_iter().map(|(_, v)| v).sorted_by_key(|v| v.id) {
        let mut item_types = get_item_types(&a_item);
        match item_types.len() {
            0 => {
                a_items.push(a_item);
            }
            1 => {
                a_item.itype = Some(item_types.pop().unwrap());
                a_items.push(a_item);
            }
            _ => {
                let msg = format!("{} is eligible for {} item types", a_item, item_types.len());
                tracing::warn!("{msg}");
                continue;
            }
        }
    }
    a_items
}

fn get_item_types(a_item: &ad::AItem) -> Vec<ad::AItemType> {
    let mut types = Vec::new();
    if a_item.cat_id == ec::itemcats::IMPLANT && a_item.attr_vals.contains_key(&ec::attrs::BOOSTERNESS) {
        types.push(ad::AItemType::Booster);
    };
    if a_item.grp_id == ec::itemgrps::CHARACTER {
        types.push(ad::AItemType::Character);
    };
    if a_item.cat_id == ec::itemcats::CHARGE {
        types.push(ad::AItemType::Charge);
    };
    if a_item.cat_id == ec::itemcats::DRONE {
        types.push(ad::AItemType::Drone);
    };
    if a_item.grp_id == ec::itemgrps::EFFECT_BEACON {
        types.push(ad::AItemType::EffectBeacon);
    };
    if a_item.cat_id == ec::itemcats::FIGHTER
        && (a_item.attr_vals.contains_key(&ec::attrs::FTR_SQ_IS_HEAVY)
            || a_item.attr_vals.contains_key(&ec::attrs::FTR_SQ_IS_LIGHT)
            || a_item.attr_vals.contains_key(&ec::attrs::FTR_SQ_IS_SUPPORT))
    {
        types.push(ad::AItemType::FighterSquad);
    };
    if a_item.cat_id == ec::itemcats::IMPLANT && a_item.attr_vals.contains_key(&ec::attrs::IMPLANTNESS) {
        types.push(ad::AItemType::Implant);
    };
    if a_item.cat_id == ec::itemcats::MODULE && a_item.effect_datas.contains_key(&ec::effects::HI_POWER) {
        types.push(ad::AItemType::ModHigh);
    };
    if a_item.cat_id == ec::itemcats::MODULE && a_item.effect_datas.contains_key(&ec::effects::LO_POWER) {
        types.push(ad::AItemType::ModLow);
    };
    if a_item.cat_id == ec::itemcats::MODULE && a_item.effect_datas.contains_key(&ec::effects::MED_POWER) {
        types.push(ad::AItemType::ModMid);
    };
    if a_item.cat_id == ec::itemcats::MODULE && a_item.effect_datas.contains_key(&ec::effects::RIG_SLOT) {
        types.push(ad::AItemType::Rig);
    };
    if a_item.grp_id == ec::itemgrps::MUTAPLASMID {
        types.push(ad::AItemType::Mutaplasmid);
    };
    if a_item.cat_id == ec::itemcats::SHIP {
        types.push(ad::AItemType::Ship);
    };
    if a_item.cat_id == ec::itemcats::SKILL {
        types.push(ad::AItemType::Skill);
    };
    if a_item.grp_id == ec::itemgrps::SHIP_MOD {
        types.push(ad::AItemType::Stance);
    };
    if a_item.cat_id == ec::itemcats::SUBSYSTEM && a_item.effect_datas.contains_key(&ec::effects::SUBSYSTEM) {
        types.push(ad::AItemType::Subsystem);
    };
    types
}
