use crate::{
    ad::{ABuff, ABuffAffecteeFilter, ABuffAggrMode, ABuffId, ABuffModifier, AModifierSrq, AOp},
    ed::EData,
    util::{RMap, StrMsgError},
};

pub(in crate::adg::flow::conv_pre) fn conv_buffs(e_data: &EData) -> RMap<ABuffId, ABuff> {
    let mut a_buffs = RMap::new();
    for e_buff in e_data.buffs.data.iter() {
        let op = match conv_buff_op(&e_buff.operation) {
            Ok(op) => op,
            Err(e) => {
                let msg = format!("{e_buff}: {e}");
                tracing::warn!("{msg}");
                continue;
            }
        };
        let aggr_mode = match conv_buff_aggr_mode(&e_buff.aggregate_mode) {
            Ok(am) => am,
            Err(e) => {
                let msg = format!("{e_buff}: {e}");
                tracing::warn!("{msg}");
                continue;
            }
        };
        let mut a_mods = Vec::new();
        for e_item_mod in e_buff.item_mods.iter() {
            a_mods.push(ABuffModifier {
                affectee_filter: ABuffAffecteeFilter::Direct,
                affectee_attr_id: e_item_mod.attr_id,
            });
        }
        for e_loc_mod in e_buff.loc_mods.iter() {
            a_mods.push(ABuffModifier {
                affectee_filter: ABuffAffecteeFilter::Loc,
                affectee_attr_id: e_loc_mod.attr_id,
            });
        }
        for e_locgroup_mod in e_buff.locgroup_mods.iter() {
            a_mods.push(ABuffModifier {
                affectee_filter: ABuffAffecteeFilter::LocGrp(e_locgroup_mod.group_id),
                affectee_attr_id: e_locgroup_mod.attr_id,
            });
        }
        for e_locsrq_mod in e_buff.locsrq_mods.iter() {
            a_mods.push(ABuffModifier {
                affectee_filter: ABuffAffecteeFilter::LocSrq(AModifierSrq::ItemId(e_locsrq_mod.skill_id)),
                affectee_attr_id: e_locsrq_mod.attr_id,
            });
        }
        let a_buff = ABuff {
            id: e_buff.id,
            aggr_mode,
            op,
            mods: a_mods,
        };
        a_buffs.insert(a_buff.id, a_buff);
    }
    a_buffs
}

fn conv_buff_aggr_mode(aggr_mode: &str) -> Result<ABuffAggrMode, StrMsgError> {
    match aggr_mode {
        "Minimum" => Ok(ABuffAggrMode::Min),
        "Maximum" => Ok(ABuffAggrMode::Max),
        _ => Err(StrMsgError {
            msg: format!("unexpected aggregate mode \"{aggr_mode}\""),
        }),
    }
}

fn conv_buff_op(operation: &str) -> Result<AOp, StrMsgError> {
    match operation {
        "PreAssignment" => Ok(AOp::PreAssign),
        "PreMul" => Ok(AOp::PreMul),
        "PreDiv" => Ok(AOp::PreDiv),
        "ModAdd" => Ok(AOp::Add),
        "ModSub" => Ok(AOp::Sub),
        "PostMul" => Ok(AOp::PostMul),
        "PostDiv" => Ok(AOp::PostDiv),
        "PostPercent" => Ok(AOp::PostPerc),
        "PostAssignment" => Ok(AOp::PostAssign),
        _ => Err(StrMsgError {
            msg: format!("unexpected operation \"{operation}\""),
        }),
    }
}
