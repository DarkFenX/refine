use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    ad,
    consts::{attrs, itemcats, ModAggrMode, ModOp, TgtMode},
    defs::{ReeFloat, ReeId, ReeInt},
    ss::{
        item::SsItem,
        svc::{
            calc::support::{AffectorSpec, SsAttrVal},
            SsSvcs,
        },
        SsView,
    },
    util::{Error, ErrorKind, Result},
};

use super::support::Modification;

const PENALTY_IMMUNE_CATS: [ReeInt; 5] = [
    itemcats::SHIP,
    itemcats::CHARGE,
    itemcats::SKILL,
    itemcats::IMPLANT,
    itemcats::SUBSYSTEM,
];
const PENALIZABLE_OPS: [ModOp; 5] = [
    ModOp::PreMul,
    ModOp::PreDiv,
    ModOp::PostMul,
    ModOp::PostDiv,
    ModOp::PostPerc,
];
const OP_ORDER: [ModOp; 9] = [
    ModOp::PreAssign,
    ModOp::PreMul,
    ModOp::PreDiv,
    ModOp::Add,
    ModOp::Sub,
    ModOp::PostMul,
    ModOp::PostDiv,
    ModOp::PostPerc,
    ModOp::PostAssign,
];
const LIMITED_PRECISION_ATTR_IDS: [ReeInt; 4] = [attrs::CPU, attrs::POWER, attrs::CPU_OUTPUT, attrs::POWER_OUTPUT];
// Source expression: 1 / e^((1 / 2.67)^2)
const PENALTY_BASE: ReeFloat = 0.86911998080039742919922218788997270166873931884765625;

impl SsSvcs {
    // Query methods
    pub(in crate::ss) fn calc_get_item_attr_val(
        &mut self,
        ss_view: &SsView,
        item_id: &ReeId,
        attr_id: &ReeInt,
    ) -> Result<SsAttrVal> {
        // Try accessing cached value
        match self.calc_get_item_dogma_attr_map(item_id)?.get(attr_id) {
            Some(v) => return Ok(*v),
            _ => (),
        };
        // If it is not cached, calculate and cache it
        let val = self.calc_calc_item_attr_val(ss_view, item_id, attr_id)?;
        self.calc_get_item_dogma_attrs_mut(item_id)?.insert(*attr_id, val);
        Ok(val)
    }
    pub(in crate::ss) fn calc_get_item_attr_vals(
        &mut self,
        ss_view: &SsView,
        item_id: &ReeId,
    ) -> Result<HashMap<ReeInt, SsAttrVal>> {
        // ssi::Item can have attributes which are not defined on the original EVE item. This happens when
        // something requested an attr value and it was calculated using base attribute value. Here,
        // we get already calculated attributes, which includes attributes absent on the EVE item
        let mut vals = self.calc_get_item_dogma_attr_map(item_id)?.clone();
        // Calculate & store attributes which are not calculated yet,
        // but are defined on the EVE item
        for attr_id in ss_view.items.get_item(item_id)?.get_orig_attrs()?.keys() {
            match self.calc_get_item_attr_val(ss_view, item_id, attr_id) {
                Ok(v) => vals.entry(*attr_id).or_insert(v),
                _ => continue,
            };
        }
        Ok(vals)
    }
    // Maintenance methods
    pub(in crate::ss) fn calc_item_loaded(&mut self, item: &SsItem) {
        self.calc_data.attrs.insert(item.get_id(), HashMap::new());
        self.calc_data.affections.reg_afee(item);
    }
    pub(in crate::ss) fn calc_item_unloaded(&mut self, item: &SsItem) {
        self.calc_data.affections.unreg_afee(item);
        self.calc_data.attrs.remove(&item.get_id());
    }
    pub(in crate::ss) fn calc_effects_started(
        &mut self,
        ss_view: &SsView,
        item: &SsItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
        let afor_specs = generate_local_afor_specs(item, effects);
        self.calc_data
            .affections
            .reg_local_afor_specs(item.get_fit_id(), afor_specs.clone());
        for afor_spec in afor_specs {
            for item_id in self
                .calc_data
                .affections
                .get_local_afee_items(&afor_spec, ss_view.items)
            {
                self.calc_force_recalc(ss_view, &item_id, &afor_spec.modifier.afee_attr_id);
            }
        }
    }
    pub(in crate::ss) fn calc_effects_stopped(
        &mut self,
        ss_view: &SsView,
        item: &SsItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
        let afor_specs = generate_local_afor_specs(item, effects);
        for afor_spec in afor_specs.iter() {
            for item_id in self
                .calc_data
                .affections
                .get_local_afee_items(&afor_spec, ss_view.items)
            {
                self.calc_force_recalc(ss_view, &item_id, &afor_spec.modifier.afee_attr_id);
            }
        }
        self.calc_data
            .affections
            .unreg_local_afor_specs(item.get_fit_id(), afor_specs);
    }
    pub(in crate::ss) fn calc_attr_value_changed(&mut self, ss_view: &SsView, item_id: &ReeId, attr_id: &ReeInt) {
        // Clear up attribute values which rely on passed attribute as an upper cap
        let capped_attr_ids = self
            .calc_data
            .caps
            .get_l2(item_id, attr_id)
            .map(|v| v.iter().map(|v| *v).collect_vec());
        if let Some(capped_attr_ids) = capped_attr_ids {
            for capped_attr_id in capped_attr_ids.iter() {
                self.calc_force_recalc(ss_view, item_id, capped_attr_id);
            }
        };
        for afor_spec in self.calc_data.affections.get_afor_specs_by_afor(item_id) {
            if afor_spec.modifier.afor_attr_id != *attr_id {
                continue;
            }
            for afee_item_id in self
                .calc_data
                .affections
                .get_local_afee_items(&afor_spec, ss_view.items)
            {
                self.calc_force_recalc(ss_view, &afee_item_id, &afor_spec.modifier.afee_attr_id);
            }
        }
    }
    // Private methods
    fn calc_calc_item_attr_val(&mut self, ss_view: &SsView, item_id: &ReeId, attr_id: &ReeInt) -> Result<SsAttrVal> {
        let item = ss_view.items.get_item(item_id)?;
        let attr = match ss_view.src.get_a_attr(attr_id) {
            Some(attr) => attr,
            None => return Err(Error::new(ErrorKind::AAttrNotFound(*attr_id))),
        };
        // Get base value; use on-item original attributes, or, if not specified, default attribute value.
        // If both can't be fetched, consider it a failure
        let base_val = match item.get_orig_attrs()?.get(attr_id) {
            Some(orig_val) => *orig_val,
            None => match attr.def_val {
                Some(def_val) => def_val,
                None => return Err(Error::new(ErrorKind::NoAttrBaseValue(*attr_id, item.get_a_item_id()))),
            },
        };
        match (attr_id, item) {
            (280, SsItem::Skill(s)) => return Ok(SsAttrVal::new(base_val, s.level as ReeFloat, s.level as ReeFloat)),
            _ => (),
        }
        let mut stacked = HashMap::new();
        let mut stacked_penalized = HashMap::new();
        // let aggregate_min = Vec::new();
        // let aggregate_max = Vec::new();
        for modification in self.calc_get_modifications(ss_view, item, attr_id).iter() {
            let penalize =
                attr.penalizable && !modification.afor_pen_immune && PENALIZABLE_OPS.contains(&modification.op);
            let mod_val = match modification.op {
                ModOp::PreAssign => modification.val,
                ModOp::PreMul => modification.val,
                ModOp::PreDiv => 1.0 / modification.val,
                ModOp::Add => modification.val,
                ModOp::Sub => -modification.val,
                ModOp::PostMul => modification.val,
                ModOp::PostDiv => 1.0 / modification.val,
                ModOp::PostPerc => 1.0 + modification.val / 100.0,
                ModOp::PostAssign => modification.val,
            };
            match modification.aggr_mode {
                ModAggrMode::Stack if penalize => stacked_penalized
                    .entry(modification.op)
                    .or_insert_with(|| Vec::new())
                    .push(mod_val),
                ModAggrMode::Stack if !penalize => stacked
                    .entry(modification.op)
                    .or_insert_with(|| Vec::new())
                    .push(mod_val),
                // TODO: add support for remaining aggregation modes
                _ => (),
            }
        }
        for (op, vals) in stacked_penalized.into_iter() {
            let penalized_val = penalize_vals(vals);
            stacked.entry(op).or_insert_with(|| Vec::new()).push(penalized_val);
        }
        let mut dogma_val = base_val;
        for op in OP_ORDER.iter() {
            match stacked.get(op) {
                Some(vals) => match op {
                    ModOp::PreAssign => {
                        dogma_val = process_assigns(vals, &attr);
                    }
                    ModOp::PreMul => dogma_val *= process_mults(vals),
                    ModOp::PreDiv => dogma_val *= process_mults(vals),
                    ModOp::Add => dogma_val += process_adds(vals),
                    ModOp::Sub => dogma_val += process_adds(vals),
                    ModOp::PostMul => dogma_val *= process_mults(vals),
                    ModOp::PostDiv => dogma_val *= process_mults(vals),
                    ModOp::PostPerc => dogma_val *= process_mults(vals),
                    ModOp::PostAssign => {
                        dogma_val = process_assigns(vals, &attr);
                    }
                },
                _ => (),
            }
        }
        // Upper cap for the attribute value being calculated
        let mut dogma_val = match attr.max_attr_id {
            Some(capping_attr_id) => match self.calc_get_item_attr_val(ss_view, item_id, &capping_attr_id) {
                Ok(capping_vals) => {
                    self.calc_data.caps.add(*item_id, capping_attr_id, *attr_id);
                    ReeFloat::min(dogma_val, capping_vals.dogma)
                }
                Err(_) => dogma_val,
            },
            None => dogma_val,
        };
        if LIMITED_PRECISION_ATTR_IDS.contains(attr_id) {
            dogma_val = (dogma_val * 100.0).round() / 100.0
        }
        Ok(SsAttrVal::new(base_val, dogma_val, dogma_val))
    }
    fn calc_force_recalc(&mut self, ss_view: &SsView, item_id: &ReeId, attr_id: &ReeInt) {
        match self.calc_get_item_dogma_attrs_mut(item_id) {
            Ok(item_attrs) => {
                if item_attrs.remove(attr_id).is_some() {
                    self.notify_attr_val_changed(ss_view, item_id, attr_id);
                }
            }
            _ => return,
        }
    }
    fn calc_get_modifications(&mut self, ss_view: &SsView, item: &SsItem, attr_id: &ReeInt) -> Vec<Modification> {
        // TODO: optimize to pass attr ID to affector getter, and allocate vector with capacity
        let mut mods = Vec::new();
        for afor_spec in self.calc_data.affections.get_afor_specs_by_afee(item).iter() {
            if &afor_spec.modifier.afee_attr_id != attr_id {
                continue;
            }
            let val = match self.calc_get_item_attr_val(ss_view, &afor_spec.item_id, &afor_spec.modifier.afor_attr_id) {
                Ok(v) => v,
                _ => continue,
            };
            let afor_item = match ss_view.items.get_item(&afor_spec.item_id) {
                Ok(i) => i,
                _ => continue,
            };
            let pen_immune = match afor_item.get_category_id() {
                Ok(cid) => PENALTY_IMMUNE_CATS.contains(&cid),
                _ => continue,
            };
            // TODO: implement resistance support
            let modification = Modification::new(afor_spec.modifier.op, val.dogma, 1.0, ModAggrMode::Stack, pen_immune);
            mods.push(modification);
        }
        mods
    }
    fn calc_get_item_dogma_attr_map(&self, item_id: &ReeId) -> Result<&HashMap<ReeInt, SsAttrVal>> {
        // All items known to calculator are in this map, so consider absence an error
        self.calc_data
            .attrs
            .get(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemIdNotFound(*item_id)))
    }
    fn calc_get_item_dogma_attrs_mut(&mut self, item_id: &ReeId) -> Result<&mut HashMap<ReeInt, SsAttrVal>> {
        // All items known to calculator are in this map, so consider absence an error
        self.calc_data
            .attrs
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemIdNotFound(*item_id)))
    }
}

// Calculation-related functions
fn penalize_vals(mut vals: Vec<ReeFloat>) -> ReeFloat {
    // Gather positive multipliers into one chain, negative into another, with stronger modifications
    // being first
    let positive = vals
        .drain_filter(|v| *v > 1.0)
        .into_iter()
        .sorted_by(|a, b| b.partial_cmp(a).unwrap())
        .collect_vec();
    let negative = vals
        .into_iter()
        .sorted_by(|a, b| a.partial_cmp(b).unwrap())
        .collect_vec();
    get_chain_val(positive) * get_chain_val(negative)
}

fn get_chain_val(vals: Vec<ReeFloat>) -> ReeFloat {
    let mut val = 1.0;
    for (i, mod_val) in vals.iter().enumerate() {
        // Ignore 12th modification and further as non-significant
        if i > 10 {
            break;
        }
        val *= 1.0 + (mod_val - 1.0) * PENALTY_BASE.powi((i as i32).pow(2));
    }
    val
}

fn process_assigns(assigns: &Vec<ReeFloat>, attr: &ad::AAttr) -> ReeFloat {
    match attr.hig {
        true => *assigns.iter().max_by(|a, b| a.total_cmp(b)).unwrap(),
        false => *assigns.iter().min_by(|a, b| a.total_cmp(b)).unwrap(),
    }
}
fn process_mults(mults: &Vec<ReeFloat>) -> ReeFloat {
    let mut val = 1.0;
    mults.iter().for_each(|v| val *= v);
    val
}
fn process_adds(adds: &Vec<ReeFloat>) -> ReeFloat {
    let mut val = 0.0;
    adds.iter().for_each(|v| val += v);
    val
}

// Maintenance- and query-related functions
fn generate_local_afor_specs(afor_item: &SsItem, effects: &Vec<ad::ArcEffect>) -> Vec<AffectorSpec> {
    let mut specs = Vec::new();
    for effect in effects.iter().filter(|e| matches!(&e.tgt_mode, TgtMode::None)) {
        for afor_mod in effect.mods.iter() {
            let afor_item_id = afor_item.get_id();
            let afor_spec = AffectorSpec::new(afor_item_id, effect.clone(), *afor_mod);
            specs.push(afor_spec);
        }
    }
    specs
}
