//! Methods here reimplement attribute calculation counterparts to some extent, to provide extended
//! info while not bloating calculation part (since calculation is supposed to be used much more
//! often than modification info fetching).

use crate::{
    defs::{AttrVal, EAttrId, SolItemId},
    ec,
    sol::{
        item::SolItem,
        svc::{
            svce_calc::{
                SolAffectorInfo, SolAffectorValueInfo, SolAttrValInfo, SolModAccumInfo, SolModificationInfo, SolOpInfo,
            },
            SolSvcs,
        },
        SolView,
    },
    util::{Error, ErrorKind, Result, StMapVecL1, StSet},
};

const LIMITED_PRECISION_ATTR_IDS: [EAttrId; 4] = [
    ec::attrs::CPU,
    ec::attrs::POWER,
    ec::attrs::CPU_OUTPUT,
    ec::attrs::POWER_OUTPUT,
];

impl SolSvcs {
    // Query methods
    pub(in crate::sol) fn calc_iter_item_mods(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
    ) -> Result<impl ExactSizeIterator<Item = (EAttrId, Vec<SolModificationInfo>)>> {
        let mut info_map = StMapVecL1::new();
        for attr_id in self.calc_iter_item_attr_ids(sol_view, item_id)? {
            let mut attr_info = match self.calc_calc_item_attr_info(sol_view, item_id, &attr_id) {
                Ok(attr_info) => attr_info,
                _ => continue,
            };
            let mut info_vec = Vec::new();
            info_vec.extend(attr_info.effective_infos.extract_if(|_| true));
            info_vec.extend(attr_info.filtered_infos.extract_if(|_| true));
            if !info_vec.is_empty() {
                info_map.extend_entries(attr_id, info_vec.into_iter());
            }
        }
        Ok(info_map.into_iter())
    }
    // Private methods
    fn calc_iter_item_attr_ids(
        &self,
        sol_view: &SolView,
        item_id: &SolItemId,
    ) -> Result<impl ExactSizeIterator<Item = EAttrId>> {
        let mut attr_ids = StSet::new();
        for attr_id in sol_view.items.get_item(item_id)?.get_orig_attrs()?.keys() {
            attr_ids.insert(*attr_id);
        }
        for attr_id in self.calc_data.attrs.get_item_attrs(item_id)?.keys() {
            attr_ids.insert(*attr_id);
        }
        Ok(attr_ids.into_iter())
    }
    fn calc_calc_item_attr_info(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) -> Result<SolAttrValInfo> {
        let item = sol_view.items.get_item(item_id)?;
        let attr = match sol_view.src.get_a_attr(attr_id) {
            Some(attr) => attr,
            None => return Err(Error::new(ErrorKind::AAttrNotFound(*attr_id))),
        };
        // Get base value; use on-item original attributes, or, if not specified, default attribute value.
        // If both can't be fetched, consider it a failure
        let base_val = match item.get_orig_attrs()?.get(attr_id) {
            Some(orig_val) => *orig_val,
            None => attr.def_val,
        };
        match (attr_id, item) {
            (&ec::attrs::SKILL_LEVEL, SolItem::Skill(s)) => return Ok(SolAttrValInfo::new(s.level as AttrVal)),
            _ => (),
        }
        let mut accumulator = SolModAccumInfo::new();
        for modification in self.calc_iter_modifications(sol_view, item, attr_id) {
            accumulator.add_val(
                modification.val,
                modification.res_mult,
                modification.proj_mult,
                &modification.op,
                attr.penalizable,
                &modification.affector_item_cat_id,
                &modification.aggr_mode,
            );
        }
        let mut dogma_attr_info = accumulator.apply_dogma_mods(base_val, attr.hig);
        // Upper cap for the attribute value being calculated
        match attr.max_attr_id {
            Some(capping_attr_id) => match self.calc_get_item_attr_val(sol_view, item_id, &capping_attr_id) {
                Ok(capping_vals) => {
                    self.calc_data
                        .deps
                        .add_direct_local(*item_id, capping_attr_id, *attr_id);
                    if capping_vals.dogma < dogma_attr_info.value {
                        dogma_attr_info.value = capping_vals.dogma;
                        dogma_attr_info.effective_infos.push(SolModificationInfo::new(
                            capping_vals.dogma,
                            None,
                            None,
                            capping_vals.dogma,
                            None,
                            Some(capping_vals.dogma),
                            SolOpInfo::MaxLimit,
                            vec![SolAffectorInfo::new(
                                *item_id,
                                SolAffectorValueInfo::AttrId(capping_attr_id),
                            )],
                        ))
                    }
                }
                Err(_) => (),
            },
            None => (),
        };
        if LIMITED_PRECISION_ATTR_IDS.contains(attr_id) {
            dogma_attr_info.value = (dogma_attr_info.value * 100.0).round() / 100.0
        }
        // Post-dogma calculations
        let extra_attr_info = accumulator.apply_extra_mods(dogma_attr_info, attr.hig);
        Ok(extra_attr_info)
    }
}

//
// fn filter_useless(attr_id: &EAttrId, mods: &mut Vec<SolModificationInfo>, sol_view: &SolView) {
//     // Filter out modifications which get overridden by post-assigment
//     filter_pre_postassign(mods);
//     // Filter out modifications where right hand operand doesn't do anything because of its value
//     filter_neutral_invalid_operands(mods);
//     // Since only one of assignment operations is effective, include only that one
//     if let Some(attr) = sol_view.src.get_a_attr(attr_id) {
//         filter_ineffective_assigns(mods, &attr, SolOpInfo::PreAssign);
//         filter_ineffective_assigns(mods, &attr, SolOpInfo::PostAssign);
//     }
// }
//
// fn filter_pre_postassign(mods: &mut Vec<SolModificationInfo>) {
//     if mods.iter().any(|v| matches!(v.op, SolOpInfo::PostAssign)) {
//         mods.retain(|m| match m.op {
//             // Only those 2 modifications are processed after post-assignment
//             SolOpInfo::PostAssign | SolOpInfo::MaxLimit | SolOpInfo::ExtraMul => true,
//             _ => false,
//         });
//     };
// }
//
// fn filter_neutral_invalid_operands(mods: &mut Vec<SolModificationInfo>) {
//     mods.retain(|m| match m.op {
//         SolOpInfo::PreMul | SolOpInfo::PostMul | SolOpInfo::ExtraMul => m.val != 1.0,
//         SolOpInfo::PreDiv | SolOpInfo::PostDiv => m.val != 1.0 && m.val != 0.0,
//         SolOpInfo::Add | SolOpInfo::Sub | SolOpInfo::PostPerc => m.val != 0.0,
//         _ => true,
//     });
// }
//
// fn filter_ineffective_assigns(mods: &mut Vec<SolModificationInfo>, attr: &ad::AAttr, op:
// SolOpInfo) {     let assign_mods = mods.extract_if(|m| op == m.op).collect_vec();
//     if !assign_mods.is_empty() {
//         let effective_mod = match attr.hig {
//             true => assign_mods.into_iter().max_by(|a, b| a.val.total_cmp(&b.val)).unwrap(),
//             false => assign_mods.into_iter().min_by(|a, b| a.val.total_cmp(&b.val)).unwrap(),
//         };
//         mods.push(effective_mod);
//     }
// }
