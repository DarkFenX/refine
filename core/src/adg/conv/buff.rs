use itertools::Itertools;

use crate::{
    ad,
    adg::GData,
    defs::EBuffId,
    shr::{ModAggrMode, ModOp},
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
        let aggr_mode = match conv_buff_aggr_mode(&e_buff.aggregate_mode, e_buff.id) {
            Ok(am) => am,
            Err(e) => {
                let msg = format!("{}: {}", e_buff, e.msg);
                tracing::warn!("{msg}");
                continue;
            }
        };
        let mut a_mods = Vec::new();
        for e_item_mod in e_buff.item_mods.iter() {
            a_mods.push(ad::ABuffAttrMod::new(
                ad::AModTgtFilter::Direct(ad::AModDomain::Ship),
                e_item_mod.attr_id,
            ));
        }
        for e_loc_mod in e_buff.loc_mods.iter() {
            a_mods.push(ad::ABuffAttrMod::new(
                ad::AModTgtFilter::Loc(ad::AModDomain::Ship),
                e_loc_mod.attr_id,
            ));
        }
        for e_locgroup_mod in e_buff.locgroup_mods.iter() {
            a_mods.push(ad::ABuffAttrMod::new(
                ad::AModTgtFilter::LocGrp(ad::AModDomain::Ship, e_locgroup_mod.group_id),
                e_locgroup_mod.attr_id,
            ));
        }
        for e_locsrq_mod in e_buff.locsrq_mods.iter() {
            a_mods.push(ad::ABuffAttrMod::new(
                ad::AModTgtFilter::LocSrq(ad::AModDomain::Ship, ad::AModSrq::ItemId(e_locsrq_mod.skill_id)),
                e_locsrq_mod.attr_id,
            ));
        }
        let a_buff = ad::ABuff::new(e_buff.id, aggr_mode, op, a_mods);
        a_buffs.push(a_buff);
    }
    a_buffs
}

fn conv_buff_aggr_mode(aggr_mode: &str, key: EBuffId) -> IntResult<ModAggrMode> {
    match aggr_mode {
        "Minimum" => Ok(ModAggrMode::Min(key)),
        "Maximum" => Ok(ModAggrMode::Max(key)),
        _ => Err(IntError::new(format!("unexpected aggregate mode \"{aggr_mode}\""))),
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
        _ => Err(IntError::new(format!("unexpected operation \"{operation}\""))),
    }
}
