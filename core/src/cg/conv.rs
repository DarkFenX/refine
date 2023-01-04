use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    consts::{ModAfeeFilter, ModAggrMode, ModDomain, ModOp},
    ct,
    defines::ReeInt,
    util::{Error, Result}
};

use super::Data;

// Convert data handler-provided entities into cacheable types.
pub(super) fn convert(data: &Data, warns: &mut Vec<String>) {
    let attrs = conv_attrs(data);
    let mutas = conv_mutas(data);
    let buffs = conv_buffs(data, warns);
}

// fn conv_items(data: &Data) -> Vec<ct::Item> {
// }

fn conv_attrs(data: &Data) -> Vec<ct::Attr> {
    data.attrs
        .iter()
        .map(|v| ct::Attr::new(
            v.id,
            !v.stackable,
            v.high_is_good,
            v.default_value,
            v.max_attr_id
        ))
        .collect()
}

// fn conv_effects(data: &Data) -> Vec<ct::Effect> {
//     data.effects
//         .iter()
//         .map(|v| ct::Effect::new(
//             v.id,
//             State::Active,
//             TgtMode::None,
//             v.is_assistance,
//             v.is_offensive,
//             Some(True),
//             Some(True),
//             v.discharge_attr_id,
//             v.duration_attr_id,
//             v.range_attr_id,
//             v.falloff_attr_id,
//             v.tracking_attr_id,
//             v.usage_chance_attr_id,
//             v.resist_attr_id,
//             vec![],
//             vec![],
//         ))
//         .collect()
// }

fn conv_mutas(data: &Data) -> Vec<ct::Muta> {
    let mut composed = HashMap::new();
    for item_data in data.muta_items.iter() {
        let muta = composed
            .entry(item_data.muta_id)
            .or_insert_with(|| ct::Muta::new(item_data.muta_id));
        muta.item_map.insert(item_data.in_item_id, item_data.out_item_id);
    }
    for attr_data in data.muta_attrs.iter() {
        // We are interested in attribute modifiers only for mutaplasmids which have in-out item
        // definitions
        if let Some(muta) = composed.get_mut(&attr_data.muta_id) {
            muta.attr_mods.insert(
                attr_data.attr_id,
                ct::MutaAttrRange::new(attr_data.min_attr_mult, attr_data.max_attr_mult),
            );
        }
    }
    composed.into_iter().map(|(_, v)| v).sorted_by_key(|v| v.id).collect()
}

fn conv_buffs(data: &Data, warns: &mut Vec<String>) -> Vec<ct::Buff> {
    let mut converted = vec![];
    for buff_data in data.buffs.iter().sorted_by_key(|v| v.id) {
        let op = match conv_op(&buff_data.operation) {
            Ok(op) => op,
            Err(e) => {
                let msg = format!("buff {}: {}", buff_data.id, e.msg);
                log::warn!("{}", &msg);
                warns.push(msg);
                continue
            }
        };
        let aggr_mode = match conv_aggr_mode(&buff_data.aggregate_mode, buff_data.id) {
            Ok(am) => am,
            Err(e) => {
                let msg = format!("buff {}: {}", buff_data.id, e.msg);
                log::warn!("{}", &msg);
                warns.push(msg);
                continue
            }
        };
        let mut mods = vec![];
        for item_mod in buff_data.item_mods.iter() {
            mods.push(ct::BuffAttrMod::new(
                ModAfeeFilter::Direct(ModDomain::Ship),
                item_mod.attr_id,
            ));
        }
        for loc_mod in buff_data.loc_mods.iter() {
            mods.push(ct::BuffAttrMod::new(
                ModAfeeFilter::Loc(ModDomain::Ship),
                loc_mod.attr_id,
            ));
        }
        for locgroup_mod in buff_data.locgroup_mods.iter() {
            mods.push(ct::BuffAttrMod::new(
                ModAfeeFilter::LocGrp(ModDomain::Ship, locgroup_mod.group_id),
                locgroup_mod.attr_id,
            ));
        }
        for locsrq_mod in buff_data.locsrq_mods.iter() {
            mods.push(ct::BuffAttrMod::new(
                ModAfeeFilter::LocSrq(ModDomain::Ship, locsrq_mod.skill_id),
                locsrq_mod.attr_id,
            ));
        }
        let buff = ct::Buff::new(buff_data.id, aggr_mode, op, mods);
        converted.push(buff);
    }
    converted
}

fn conv_aggr_mode(aggr_mode: &str, key: ReeInt) -> Result<ModAggrMode> {
    match aggr_mode {
        "Maximum" => Ok(ModAggrMode::Max(key)),
        "Minimum" => Ok(ModAggrMode::Min(key)),
        _ => Err(Error::new(format!("unexpected aggregate mode \"{}\"", aggr_mode)))
    }
}

fn conv_op(operation: &str) -> Result<ModOp> {
    match operation {
        "ModAdd" => Ok(ModOp::Add),
        "PostMul" => Ok(ModOp::PostMul),
        "PostPercent" => Ok(ModOp::PostPerc),
        "PostAssignment" => Ok(ModOp::PostAssign),
        _ => Err(Error::new(format!("unexpected operation \"{}\"", operation)))
    }
}
