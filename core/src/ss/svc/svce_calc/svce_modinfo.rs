//! Methods here largely reimplement attribute calculation counterparts to provide extended info
//! while not bloating calculation part, which is supposed to be used much more often.

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{
    ad,
    defs::{EAttrId, SsItemId},
    ss::{
        item::SsItem,
        svc::{
            svce_calc::{
                misc::ModKey,
                mod_info::{ModOpInfo, ModSrcInfo, ModSrcValInfo},
            },
            SsSvcs,
        },
        SsView,
    },
    util::Result,
    ModAggrMode,
};

use super::{mod_info::ModInfo, svce_attr::is_penalizable};

impl SsSvcs {
    // Query methods
    pub(in crate::ss) fn calc_get_item_mods(
        &mut self,
        ss_view: &SsView,
        item_id: &SsItemId,
    ) -> Result<HashMap<EAttrId, Vec<ModInfo>>> {
        let item = ss_view.items.get_item(item_id)?;
        let mut info_map = HashMap::new();
        for attr_id in self.calc_get_item_attr_ids(ss_view, item_id)? {
            let attr = match ss_view.src.get_a_attr(&attr_id) {
                Some(attr) => attr,
                None => continue,
            };
            let mut infos = self.calc_get_item_attr_mods(ss_view, item, &attr);
            filter_useless(&attr_id, &mut infos, ss_view);
            if !infos.is_empty() {
                info_map.insert(attr_id, infos);
            }
        }
        Ok(info_map)
    }
    // Private methods
    fn calc_get_item_attr_ids(&self, ss_view: &SsView, item_id: &SsItemId) -> Result<HashSet<EAttrId>> {
        let mut attr_ids = HashSet::new();
        for attr_id in ss_view.items.get_item(item_id)?.get_orig_attrs()?.keys() {
            attr_ids.insert(*attr_id);
        }
        for attr_id in self.calc_data.attrs.get_item_attrs(item_id)?.keys() {
            attr_ids.insert(*attr_id);
        }
        Ok(attr_ids)
    }
    fn calc_get_item_attr_mods(&mut self, ss_view: &SsView, item: &SsItem, attr: &ad::AAttr) -> Vec<ModInfo> {
        let mut mod_map = HashMap::new();
        for modifier in self
            .calc_data
            .mods
            .get_mods_for_tgt(item, &attr.id, ss_view.fits)
            .iter()
        {
            let val = match modifier.get_mod_val(self, ss_view) {
                Ok(v) => v,
                _ => continue,
            };
            let src_item = match ss_view.items.get_item(&modifier.src_item_id) {
                Ok(i) => i,
                _ => continue,
            };
            let src_item_cat_id = match src_item.get_category_id() {
                Ok(src_item_cat_id) => src_item_cat_id,
                _ => continue,
            };
            let penalizable = is_penalizable(attr, &src_item_cat_id, &modifier.op);
            let srcs = modifier
                .get_srcs(ss_view)
                .into_iter()
                .map(|(i, a)| ModSrcInfo::new(i, ModSrcValInfo::AttrId(a)))
                .collect();
            let mod_key = ModKey::from(modifier);
            let mod_info = ModInfo::new(val, (&modifier.op).into(), penalizable, modifier.aggr_mode, srcs);
            mod_map.insert(mod_key, mod_info);
        }
        let mut mod_vec = mod_map.into_values().collect_vec();
        if let Some(max_attr_id) = attr.max_attr_id {
            if let Ok(val) = self.calc_get_item_attr_val(ss_view, &item.get_id(), &max_attr_id) {
                let mod_info = ModInfo::new(
                    val.dogma,
                    ModOpInfo::Limit,
                    false,
                    ModAggrMode::Stack,
                    vec![ModSrcInfo::new(item.get_id(), ModSrcValInfo::AttrId(max_attr_id))],
                );
                mod_vec.push(mod_info);
            }
        }
        mod_vec
    }
}

fn filter_useless(attr_id: &EAttrId, mods: &mut Vec<ModInfo>, ss_view: &SsView) {
    // Filter out modifications which get overridden by post-assigment
    filter_pre_postassign(mods);
    // Filter out modifications where right hand operand doesn't do anything because of its value
    filter_neutral_invalid_operands(mods);
    // Since only one of assignment operations is effective, include only that one
    if let Some(attr) = ss_view.src.get_a_attr(attr_id) {
        filter_ineffective_assigns(mods, &attr, ModOpInfo::PreAssign);
        filter_ineffective_assigns(mods, &attr, ModOpInfo::PostAssign);
    }
}

fn filter_pre_postassign(mods: &mut Vec<ModInfo>) {
    if mods.iter().any(|v| matches!(v.op, ModOpInfo::PostAssign)) {
        mods.retain(|m| match m.op {
            // Only those 2 modifications are processed after post-assignment
            ModOpInfo::PostAssign | ModOpInfo::Limit | ModOpInfo::ExtraMul => true,
            _ => false,
        });
    };
}

fn filter_neutral_invalid_operands(mods: &mut Vec<ModInfo>) {
    mods.retain(|m| match m.op {
        ModOpInfo::PreMul | ModOpInfo::PostMul | ModOpInfo::ExtraMul => m.val != 1.0,
        ModOpInfo::PreDiv | ModOpInfo::PostDiv => m.val != 1.0 && m.val != 0.0,
        ModOpInfo::Add | ModOpInfo::Sub | ModOpInfo::PostPerc => m.val != 0.0,
        _ => true,
    });
}

fn filter_ineffective_assigns(mods: &mut Vec<ModInfo>, attr: &ad::AAttr, op: ModOpInfo) {
    let assign_mods = mods.extract_if(|m| op == m.op).collect_vec();
    if !assign_mods.is_empty() {
        let effective_mod = match attr.hig {
            true => assign_mods.into_iter().max_by(|a, b| a.val.total_cmp(&b.val)).unwrap(),
            false => assign_mods.into_iter().min_by(|a, b| a.val.total_cmp(&b.val)).unwrap(),
        };
        mods.push(effective_mod);
    }
}
