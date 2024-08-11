use itertools::Itertools;

use crate::{ad, adg::GData, util::StrMsgError};

pub(in crate::adg::conv) fn conv_buffs(g_data: &GData) -> Vec<ad::ABuff> {
    let mut a_buffs = Vec::new();
    for e_buff in g_data.buffs.iter().sorted_by_key(|v| v.id) {
        let op = match conv_buff_op(&e_buff.operation) {
            Ok(op) => op,
            Err(e) => {
                let msg = format!("{}: {}", e_buff, e);
                tracing::warn!("{msg}");
                continue;
            }
        };
        let aggr_mode = match conv_buff_aggr_mode(&e_buff.aggregate_mode) {
            Ok(am) => am,
            Err(e) => {
                let msg = format!("{}: {}", e_buff, e);
                tracing::warn!("{msg}");
                continue;
            }
        };
        let mut a_mods = Vec::new();
        for e_item_mod in e_buff.item_mods.iter() {
            a_mods.push(ad::ABuffModifier::new(
                ad::ABuffAffecteeFilter::Direct,
                e_item_mod.attr_id,
            ));
        }
        for e_loc_mod in e_buff.loc_mods.iter() {
            a_mods.push(ad::ABuffModifier::new(ad::ABuffAffecteeFilter::Loc, e_loc_mod.attr_id));
        }
        for e_locgroup_mod in e_buff.locgroup_mods.iter() {
            a_mods.push(ad::ABuffModifier::new(
                ad::ABuffAffecteeFilter::LocGrp(e_locgroup_mod.group_id),
                e_locgroup_mod.attr_id,
            ));
        }
        for e_locsrq_mod in e_buff.locsrq_mods.iter() {
            a_mods.push(ad::ABuffModifier::new(
                ad::ABuffAffecteeFilter::LocSrq(ad::AModifierSrq::ItemId(e_locsrq_mod.skill_id)),
                e_locsrq_mod.attr_id,
            ));
        }
        let a_buff = ad::ABuff::new(e_buff.id, aggr_mode, op, a_mods);
        a_buffs.push(a_buff);
    }
    a_buffs
}

fn conv_buff_aggr_mode(aggr_mode: &str) -> Result<ad::ABuffAggrMode, StrMsgError> {
    match aggr_mode {
        "Minimum" => Ok(ad::ABuffAggrMode::Min),
        "Maximum" => Ok(ad::ABuffAggrMode::Max),
        _ => Err(StrMsgError::new(format!("unexpected aggregate mode \"{aggr_mode}\""))),
    }
}

fn conv_buff_op(operation: &str) -> Result<ad::AOp, StrMsgError> {
    match operation {
        "PreAssignment" => Ok(ad::AOp::PreAssign),
        "PreMul" => Ok(ad::AOp::PreMul),
        "PreDiv" => Ok(ad::AOp::PreDiv),
        "ModAdd" => Ok(ad::AOp::Add),
        "ModSub" => Ok(ad::AOp::Sub),
        "PostMul" => Ok(ad::AOp::PostMul),
        "PostDiv" => Ok(ad::AOp::PostDiv),
        "PostPercent" => Ok(ad::AOp::PostPerc),
        "PostAssignment" => Ok(ad::AOp::PostAssign),
        _ => Err(StrMsgError::new(format!("unexpected operation \"{operation}\""))),
    }
}
