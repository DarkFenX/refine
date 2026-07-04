use crate::{
    ad::{
        AAttrId, ABuff, ABuffAffecteeFilter, ABuffAggrMode, ABuffId, ABuffModifier, ABuffs, ADataGenerator, AItemGrpId,
        AItemId, AModifierSrq, AOp,
    },
    util::RMap,
};

impl ADataGenerator {
    pub(super) fn conv_buffs(&mut self) {
        let mut a_buffs = RMap::new();
        for e_buff in self.e_data.buffs.data.iter() {
            let op = match conv_buff_op(&e_buff.operation) {
                Ok(op) => op,
                Err(e) => {
                    let warning = format!("{e_buff}: {e}");
                    self.a_data.warnings.conversion_main.push(warning);
                    continue;
                }
            };
            let aggr_mode = match conv_buff_aggr_mode(&e_buff.aggregate_mode) {
                Ok(am) => am,
                Err(e) => {
                    let warning = format!("{e_buff}: {e}");
                    self.a_data.warnings.conversion_main.push(warning);
                    continue;
                }
            };
            let mut a_mods = Vec::new();
            for e_item_mod in e_buff.item_mods.iter() {
                a_mods.push(ABuffModifier {
                    affectee_filter: ABuffAffecteeFilter::Direct,
                    affectee_attr_id: AAttrId::from_eid(e_item_mod.attr_id),
                });
            }
            for e_loc_mod in e_buff.loc_mods.iter() {
                a_mods.push(ABuffModifier {
                    affectee_filter: ABuffAffecteeFilter::Loc,
                    affectee_attr_id: AAttrId::from_eid(e_loc_mod.attr_id),
                });
            }
            for e_locgroup_mod in e_buff.locgroup_mods.iter() {
                a_mods.push(ABuffModifier {
                    affectee_filter: ABuffAffecteeFilter::LocGrp(AItemGrpId::from_eid(e_locgroup_mod.group_id)),
                    affectee_attr_id: AAttrId::from_eid(e_locgroup_mod.attr_id),
                });
            }
            for e_locsrq_mod in e_buff.locsrq_mods.iter() {
                a_mods.push(ABuffModifier {
                    affectee_filter: ABuffAffecteeFilter::LocSrq(AModifierSrq::ItemId(AItemId::from_eid(
                        e_locsrq_mod.skill_id,
                    ))),
                    affectee_attr_id: AAttrId::from_eid(e_locsrq_mod.attr_id),
                });
            }
            let a_buff = ABuff {
                id: ABuffId::from_eid(e_buff.id),
                aggr_mode,
                op,
                mods: a_mods.into_iter().collect(),
            };
            a_buffs.insert(a_buff.id, a_buff);
        }
        self.a_data.buffs = ABuffs { data: a_buffs };
    }
}

fn conv_buff_aggr_mode(aggr_mode: &str) -> Result<ABuffAggrMode, String> {
    match aggr_mode {
        "Minimum" => Ok(ABuffAggrMode::Min),
        "Maximum" => Ok(ABuffAggrMode::Max),
        _ => Err(format!("unexpected aggregate mode \"{aggr_mode}\"")),
    }
}

fn conv_buff_op(operation: &str) -> Result<AOp, String> {
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
        _ => Err(format!("unexpected operation \"{operation}\"")),
    }
}
