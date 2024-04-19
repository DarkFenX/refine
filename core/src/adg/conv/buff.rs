use itertools::Itertools;

use crate::{
    ad,
    adg::GData,
    defs::AggrKey,
    shr::ModAggrMode,
    util::{IntError, IntResult},
};

pub(in crate::adg::conv) fn conv_buffs(g_data: &GData) -> Vec<ad::ABuff> {
    let mut a_buffs = Vec::new();
    for e_buff in g_data.buffs.iter().sorted_by_key(|v| v.id) {
        let op = match conv_buff_op(&e_buff.operation) {
            Ok(op) => op,
            Err(e) => {
                let msg = format!("{}: {}", e_buff, e.msg);
                tracing::warn!("{msg}");
                continue;
            }
        };
        let aggr_mode = match conv_buff_aggr_mode(&e_buff.aggregate_mode, e_buff.id as AggrKey) {
            Ok(am) => am,
            Err(e) => {
                let msg = format!("{}: {}", e_buff, e.msg);
                tracing::warn!("{msg}");
                continue;
            }
        };
        let mut a_mods = Vec::new();
        for e_item_mod in e_buff.item_mods.iter() {
            a_mods.push(ad::ABuffAttrMod::new(ad::ABuffTgtFilter::Direct, e_item_mod.attr_id));
        }
        for e_loc_mod in e_buff.loc_mods.iter() {
            a_mods.push(ad::ABuffAttrMod::new(ad::ABuffTgtFilter::Loc, e_loc_mod.attr_id));
        }
        for e_locgroup_mod in e_buff.locgroup_mods.iter() {
            a_mods.push(ad::ABuffAttrMod::new(
                ad::ABuffTgtFilter::LocGrp(e_locgroup_mod.group_id),
                e_locgroup_mod.attr_id,
            ));
        }
        for e_locsrq_mod in e_buff.locsrq_mods.iter() {
            a_mods.push(ad::ABuffAttrMod::new(
                ad::ABuffTgtFilter::LocSrq(ad::AModSrq::ItemId(e_locsrq_mod.skill_id)),
                e_locsrq_mod.attr_id,
            ));
        }
        let a_buff = ad::ABuff::new(e_buff.id, aggr_mode, op, a_mods);
        a_buffs.push(a_buff);
    }
    a_buffs
}

fn conv_buff_aggr_mode(aggr_mode: &str, key: AggrKey) -> IntResult<ModAggrMode> {
    match aggr_mode {
        "Minimum" => Ok(ModAggrMode::Min(key)),
        "Maximum" => Ok(ModAggrMode::Max(key)),
        _ => Err(IntError::new(format!("unexpected aggregate mode \"{aggr_mode}\""))),
    }
}

fn conv_buff_op(operation: &str) -> IntResult<ad::AModOp> {
    match operation {
        "PreAssignment" => Ok(ad::AModOp::PreAssign),
        "PreMul" => Ok(ad::AModOp::PreMul),
        "PreDiv" => Ok(ad::AModOp::PreDiv),
        "ModAdd" => Ok(ad::AModOp::Add),
        "ModSub" => Ok(ad::AModOp::Sub),
        "PostMul" => Ok(ad::AModOp::PostMul),
        "PostDiv" => Ok(ad::AModOp::PostDiv),
        "PostPercent" => Ok(ad::AModOp::PostPerc),
        "PostAssignment" => Ok(ad::AModOp::PostAssign),
        _ => Err(IntError::new(format!("unexpected operation \"{operation}\""))),
    }
}
