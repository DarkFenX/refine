use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    ad,
    adg::{GData, GSupport},
    consts::{attrs, effects, get_abil_effect, itemcats, itemgrps, ItemType},
    defs::ReeInt,
    ed,
    util::Named,
};

pub(in crate::adg::conv) fn conv_items(g_data: &GData, g_supp: &GSupport) -> Vec<ad::AItem> {
    // Auxiliary maps
    let defeff_map = g_data
        .item_effects
        .iter()
        .filter(|v| v.is_default)
        .map(|v| (v.item_id, v.effect_id))
        .collect::<HashMap<ReeInt, ReeInt>>();
    let mut a_item_map = HashMap::new();
    for e_item in g_data.items.iter() {
        // Item category ID
        let cat_id = match g_supp.grp_cat_map.get(&e_item.group_id) {
            Some(&cid) => cid,
            None => {
                let msg = format!("unable to find category ID for {} {}", ed::EItem::get_name(), e_item.id);
                log::warn!("{msg}");
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
            Some(a_item) => match get_abil_effect(e_item_abil.abil_id) {
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
                let msg = format!(
                    "{} {} is eligible for {} item types",
                    ad::AItem::get_name(),
                    a_item.id,
                    item_types.len()
                );
                log::warn!("{msg}");
                continue;
            }
        }
    }
    a_items
}

fn get_item_types(a_item: &ad::AItem) -> Vec<ItemType> {
    let mut types = Vec::new();
    if a_item.cat_id == itemcats::IMPLANT && a_item.attr_vals.contains_key(&attrs::BOOSTERNESS) {
        types.push(ItemType::Booster);
    };
    if a_item.grp_id == itemgrps::CHARACTER {
        types.push(ItemType::Character);
    };
    if a_item.cat_id == itemcats::CHARGE {
        types.push(ItemType::Charge);
    };
    if a_item.cat_id == itemcats::DRONE {
        types.push(ItemType::Drone);
    };
    if a_item.grp_id == itemgrps::EFFECT_BEACON {
        types.push(ItemType::EffectBeacon);
    };
    if a_item.cat_id == itemcats::FIGHTER
        && (a_item.attr_vals.contains_key(&attrs::FTR_SQ_IS_HEAVY)
            || a_item.attr_vals.contains_key(&attrs::FTR_SQ_IS_LIGHT)
            || a_item.attr_vals.contains_key(&attrs::FTR_SQ_IS_SUPPORT))
    {
        types.push(ItemType::FighterSquad);
    };
    if a_item.cat_id == itemcats::IMPLANT && a_item.attr_vals.contains_key(&attrs::IMPLANTNESS) {
        types.push(ItemType::Implant);
    };
    if a_item.cat_id == itemcats::MODULE && a_item.effect_datas.contains_key(&effects::HI_POWER) {
        types.push(ItemType::ModHigh);
    };
    if a_item.cat_id == itemcats::MODULE && a_item.effect_datas.contains_key(&effects::LO_POWER) {
        types.push(ItemType::ModLow);
    };
    if a_item.cat_id == itemcats::MODULE && a_item.effect_datas.contains_key(&effects::MED_POWER) {
        types.push(ItemType::ModMid);
    };
    if a_item.cat_id == itemcats::MODULE && a_item.effect_datas.contains_key(&effects::RIG_SLOT) {
        types.push(ItemType::Rig);
    };
    if a_item.grp_id == itemgrps::MUTAPLASMID {
        types.push(ItemType::Mutaplasmid);
    };
    if a_item.cat_id == itemcats::SHIP {
        types.push(ItemType::Ship);
    };
    if a_item.cat_id == itemcats::SKILL {
        types.push(ItemType::Skill);
    };
    if a_item.grp_id == itemgrps::SHIP_MOD {
        types.push(ItemType::Stance);
    };
    if a_item.cat_id == itemcats::SUBSYSTEM && a_item.effect_datas.contains_key(&effects::SUBSYSTEM) {
        types.push(ItemType::Subsystem);
    };
    types
}
