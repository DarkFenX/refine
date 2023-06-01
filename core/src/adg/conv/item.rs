use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    adt,
    consts::{attrs, effects, get_abil_effect, itemcats, itemgrps, ItemType},
    defs::ReeInt,
    edt,
    util::Named,
};

use super::super::{data::Support, Data};

pub(super) fn conv_items(erg_data: &Data, supp: &Support) -> Vec<adt::Item> {
    // Auxiliary maps
    let defeff_map = erg_data
        .item_effects
        .iter()
        .filter(|v| v.is_default)
        .map(|v| (v.item_id, v.effect_id))
        .collect::<HashMap<ReeInt, ReeInt>>();
    let mut item_map = HashMap::new();
    for item_data in erg_data.items.iter() {
        // Item category ID
        let cat_id = match supp.grp_cat_map.get(&item_data.group_id) {
            Some(&cid) => cid,
            None => {
                let msg = format!(
                    "unable to find category ID for {} {}",
                    edt::Item::get_name(),
                    item_data.id
                );
                log::warn!("{}", msg);
                continue;
            }
        };
        // Item default effect
        let defeff_id = match defeff_map.get(&item_data.id) {
            Some(&deff) => Some(deff),
            None => None,
        };
        // Item construction
        let item = adt::Item::new(
            item_data.id,
            None,
            item_data.group_id,
            cat_id,
            HashMap::new(),
            HashMap::new(),
            defeff_id,
            HashMap::new(),
        );
        item_map.insert(item.id, item);
    }
    // Item attributes
    for item_attr in erg_data.item_attrs.iter() {
        item_map
            .get_mut(&item_attr.item_id)
            .and_then(|v| v.attr_vals.insert(item_attr.attr_id, item_attr.value));
    }
    // Item effects & extended effect data from abilities
    for item_effect in erg_data.item_effects.iter() {
        item_map.get_mut(&item_effect.item_id).and_then(|v| {
            v.effect_datas
                .insert(item_effect.effect_id, adt::ItemEffData::new(None, None, None))
        });
    }
    for item_abil in erg_data.item_abils.iter() {
        match item_map.get_mut(&item_abil.item_id) {
            None => continue,
            Some(item) => match get_abil_effect(item_abil.abil_id) {
                None => continue,
                Some(eid) => match item.effect_datas.get_mut(&eid) {
                    None => continue,
                    Some(edata) => {
                        edata.cd = item_abil.cooldown;
                        edata.charges = item_abil.charge_count;
                        edata.charge_reload_time = item_abil.charge_rearm_time;
                    }
                },
            },
        }
    }
    // Item skill requirements
    for item_srq in erg_data.item_srqs.iter() {
        item_map
            .get_mut(&item_srq.item_id)
            .and_then(|v| v.srqs.insert(item_srq.skill_id, item_srq.level));
    }
    // Item type
    let mut items = Vec::new();
    for mut item in item_map.into_iter().map(|(_, v)| v).sorted_by_key(|v| v.id) {
        let mut item_types = get_item_types(&item);
        match item_types.len() {
            0 => {
                items.push(item);
            }
            1 => {
                item.itype = Some(item_types.pop().unwrap());
                items.push(item);
            }
            _ => {
                let msg = format!(
                    "{} {} is eligible for {} item types",
                    adt::Item::get_name(),
                    item.id,
                    item_types.len()
                );
                log::warn!("{}", msg);
                continue;
            }
        }
    }
    items
}

fn get_item_types(item: &adt::Item) -> Vec<ItemType> {
    let mut types = Vec::new();
    if item.cat_id == itemcats::IMPLANT && item.attr_vals.contains_key(&attrs::BOOSTERNESS) {
        types.push(ItemType::Booster);
    };
    if item.grp_id == itemgrps::CHARACTER {
        types.push(ItemType::Character);
    };
    if item.cat_id == itemcats::CHARGE {
        types.push(ItemType::Charge);
    };
    if item.cat_id == itemcats::DRONE {
        types.push(ItemType::Drone);
    };
    if item.grp_id == itemgrps::EFFECT_BEACON {
        types.push(ItemType::EffectBeacon);
    };
    if item.cat_id == itemcats::FIGHTER
        && (item.attr_vals.contains_key(&attrs::FTR_SQ_IS_HEAVY)
            || item.attr_vals.contains_key(&attrs::FTR_SQ_IS_LIGHT)
            || item.attr_vals.contains_key(&attrs::FTR_SQ_IS_SUPPORT))
    {
        types.push(ItemType::FighterSquad);
    };
    if item.cat_id == itemcats::IMPLANT && item.attr_vals.contains_key(&attrs::IMPLANTNESS) {
        types.push(ItemType::Implant);
    };
    if item.cat_id == itemcats::MODULE && item.effect_datas.contains_key(&effects::HI_POWER) {
        types.push(ItemType::ModHigh);
    };
    if item.cat_id == itemcats::MODULE && item.effect_datas.contains_key(&effects::LO_POWER) {
        types.push(ItemType::ModLow);
    };
    if item.cat_id == itemcats::MODULE && item.effect_datas.contains_key(&effects::MED_POWER) {
        types.push(ItemType::ModMid);
    };
    if item.cat_id == itemcats::MODULE && item.effect_datas.contains_key(&effects::RIG_SLOT) {
        types.push(ItemType::Rig);
    };
    if item.grp_id == itemgrps::MUTAPLASMID {
        types.push(ItemType::Mutaplasmid);
    };
    if item.cat_id == itemcats::SHIP {
        types.push(ItemType::Ship);
    };
    if item.cat_id == itemcats::SKILL {
        types.push(ItemType::Skill);
    };
    if item.grp_id == itemgrps::SHIP_MOD {
        types.push(ItemType::Stance);
    };
    if item.cat_id == itemcats::SUBSYSTEM && item.effect_datas.contains_key(&effects::SUBSYSTEM) {
        types.push(ItemType::Subsystem);
    };
    types
}
