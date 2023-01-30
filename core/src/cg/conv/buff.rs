use itertools::Itertools;

use crate::{
    consts::{ModAfeeFilter, ModAggrMode, ModDomain, ModOp},
    ct, dh,
    util::Named,
    IntError, IntResult, ReeInt,
};

use super::CgData;

pub(super) fn conv_buffs(cg_data: &CgData, warns: &mut Vec<String>) -> Vec<ct::Buff> {
    let mut converted = Vec::new();
    for buff_data in cg_data.buffs.iter().sorted_by_key(|v| v.id) {
        let op = match conv_buff_op(&buff_data.operation) {
            Ok(op) => op,
            Err(e) => {
                let msg = format!("{} {}: {}", dh::Buff::get_name(), buff_data.id, e.msg);
                log::warn!("{}", &msg);
                warns.push(msg);
                continue;
            }
        };
        let aggr_mode = match conv_buff_aggr_mode(&buff_data.aggregate_mode, buff_data.id) {
            Ok(am) => am,
            Err(e) => {
                let msg = format!("{} {}: {}", dh::Buff::get_name(), buff_data.id, e.msg);
                log::warn!("{}", &msg);
                warns.push(msg);
                continue;
            }
        };
        let mut mods = Vec::new();
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

fn conv_buff_aggr_mode(aggr_mode: &str, key: ReeInt) -> IntResult<ModAggrMode> {
    match aggr_mode {
        "Maximum" => Ok(ModAggrMode::Max(key)),
        "Minimum" => Ok(ModAggrMode::Min(key)),
        _ => Err(IntError::new(format!("unexpected aggregate mode \"{}\"", aggr_mode))),
    }
}

fn conv_buff_op(operation: &str) -> IntResult<ModOp> {
    match operation {
        "PreAssignment" => Ok(ModOp::PreAssign),
        "PreMul" => Ok(ModOp::PreMul),
        "PreDiv" => Ok(ModOp::PreDiv),
        "ModAdd" => Ok(ModOp::Add),
        "ModSub" => Ok(ModOp::Sub),
        "PostMul" => Ok(ModOp::PostMul),
        "PostDiv" => Ok(ModOp::PostDiv),
        "PostPercent" => Ok(ModOp::PostPerc),
        "PostAssignment" => Ok(ModOp::PostAssign),
        _ => Err(IntError::new(format!("unexpected operation \"{}\"", operation))),
    }
}
