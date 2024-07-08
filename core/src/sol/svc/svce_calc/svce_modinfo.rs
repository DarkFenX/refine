//! Methods here reimplement attribute calculation counterparts to some extent, to provide extended
//! info while not bloating calculation part (since calculation is supposed to be used much more
//! often than modification info fetching).

use itertools::Itertools;

use crate::{
    ad,
    defs::{EAttrId, EItemCatId, SolItemId},
    sol::{
        item::SolItem,
        svc::{
            svce_calc::{
                SolAffectorInfo, SolAffectorValueInfo, SolModificationInfo, SolModificationKey, SolOp, SolOpInfo,
            },
            SolSvcs,
        },
        SolView,
    },
    util::{Result, StMap, StMapVecL1, StSet},
};

const PENALIZABLE_OPS: [SolOp; 5] = [
    SolOp::PreMul,
    SolOp::PreDiv,
    SolOp::PostMul,
    SolOp::PostDiv,
    SolOp::PostPerc,
];

impl SolSvcs {
    // Query methods
    pub(in crate::sol) fn calc_iter_item_mods(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
    ) -> Result<impl ExactSizeIterator<Item = (EAttrId, Vec<SolModificationInfo>)>> {
        let item = sol_view.items.get_item(item_id)?;
        let mut info_map = StMapVecL1::new();
        for attr_id in self.calc_get_item_attr_ids(sol_view, item_id)? {
            let attr = match sol_view.src.get_a_attr(&attr_id) {
                Some(attr) => attr,
                None => continue,
            };
            let mut infos = self.calc_get_item_attr_mods(sol_view, item, &attr);
            // filter_useless(&attr_id, &mut infos, sol_view);
            if !infos.is_empty() {
                info_map.extend_entries(attr_id, infos.into_iter());
            }
        }
        Ok(info_map.into_iter())
    }
    // Private methods
    fn calc_get_item_attr_ids(
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
    fn calc_get_item_attr_mods(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        attr: &ad::AAttr,
    ) -> Vec<SolModificationInfo> {
        // let mut mod_map = StMap::new();
        // for modifier in self
        //     .calc_data
        //     .std
        //     .get_mods_for_affectee(item, &attr.id, sol_view.fits)
        //     .iter()
        // {
        //     let val = match modifier.raw.get_mod_val(self, sol_view) {
        //         Ok(v) => v,
        //         _ => continue,
        //     };
        //     let affector_item = match sol_view.items.get_item(&modifier.raw.affector_item_id) {
        //         Ok(i) => i,
        //         _ => continue,
        //     };
        //     let affector_item_cat_id = match affector_item.get_category_id() {
        //         Ok(affector_item_cat_id) => affector_item_cat_id,
        //         _ => continue,
        //     };
        //     let penalizable = is_penalizable(attr, &affector_item_cat_id, &modifier.raw.op);
        //     let affectors = modifier
        //         .raw
        //         .get_affectors(sol_view)
        //         .into_iter()
        //         .map(|(i, a)| SolAffectorInfo::new(i, SolAffectorValueInfo::AttrId(a)))
        //         .collect();
        //     let mod_key = SolModificationKey::from(modifier);
        //     let mod_info = SolModificationInfo::new(val, (&modifier.raw.op).into(), penalizable,
        // affectors);     mod_map.insert(mod_key, mod_info);
        // }
        // let mut mod_vec = mod_map.into_values().collect_vec();
        // // Expose limit modification only if attribute value matches value of limiting attributed
        // if let Some(max_attr_id) = attr.max_attr_id {
        //     if let Ok(cap_val) = self.calc_get_item_attr_val(sol_view, &item.get_id(), &max_attr_id) {
        //         if let Ok(capped_val) = self.calc_get_item_attr_val(sol_view, &item.get_id(), &attr.id) {
        //             if cap_val.dogma == capped_val.dogma {
        //                 let mod_info = SolModificationInfo::new(
        //                     cap_val.dogma,
        //                     SolOpInfo::MaxLimit,
        //                     false,
        //                     vec![SolAffectorInfo::new(
        //                         item.get_id(),
        //                         SolAffectorValueInfo::AttrId(max_attr_id),
        //                     )],
        //                 );
        //                 mod_vec.push(mod_info);
        //             }
        //         }
        //     }
        // }
        // mod_vec
        Vec::new()
    }
}

// fn is_penalizable(attr: &ad::AAttr, affector_item_cat_id: &EItemCatId, op: &SolOp) -> bool {
//     attr::is_penal(attr.penalizable, affector_item_cat_id) && PENALIZABLE_OPS.contains(op)
// }
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
