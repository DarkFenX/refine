use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    ad,
    defs::{AttrVal, EAttrId, EItemCatId, SsItemId},
    ec,
    shr::{ModAggrMode, ModOp},
    ss::{
        item::SsItem,
        svc::{calc::support::SsAttrVal, SsSvcs},
        SsView,
    },
    util::{Error, ErrorKind, Result},
};

pub(in crate::ss::svc::calc) const PENALTY_IMMUNE_CATS: [EItemCatId; 5] = [
    ec::itemcats::SHIP,
    ec::itemcats::CHARGE,
    ec::itemcats::SKILL,
    ec::itemcats::IMPLANT,
    ec::itemcats::SUBSYSTEM,
];
const PENALIZABLE_OPS: [ModOp; 5] = [
    ModOp::PreMul,
    ModOp::PreDiv,
    ModOp::PostMul,
    ModOp::PostDiv,
    ModOp::PostPerc,
];
const OP_ORDER: [ModOp; 10] = [
    ModOp::PreAssign,
    ModOp::PreMul,
    ModOp::PreDiv,
    ModOp::Add,
    ModOp::Sub,
    ModOp::PostMul,
    ModOp::PostMulImmune,
    ModOp::PostDiv,
    ModOp::PostPerc,
    ModOp::PostAssign,
];
const LIMITED_PRECISION_ATTR_IDS: [EAttrId; 4] = [
    ec::attrs::CPU,
    ec::attrs::POWER,
    ec::attrs::CPU_OUTPUT,
    ec::attrs::POWER_OUTPUT,
];
// Source expression: 1 / e^((1 / 2.67)^2)
const PENALTY_BASE: f64 = 0.86911998080039742919922218788997270166873931884765625;

impl SsSvcs {
    pub(in crate::ss::svc::calc) fn calc_calc_item_attr_val(
        &mut self,
        ss_view: &SsView,
        item_id: &SsItemId,
        attr_id: &EAttrId,
    ) -> Result<SsAttrVal> {
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
            (&ec::attrs::SKILL_LEVEL, SsItem::Skill(s)) => {
                return Ok(SsAttrVal::new(base_val, s.level as AttrVal, s.level as AttrVal))
            }
            _ => (),
        }
        let mut stacked = HashMap::new();
        let mut stacked_penalized = HashMap::new();
        // let aggregate_min = Vec::new();
        // let aggregate_max = Vec::new();
        for modification in self.calc_get_modifications(ss_view, item, attr_id).values() {
            let penalize =
                attr.penalizable && !modification.src_pen_immune && PENALIZABLE_OPS.contains(&modification.op);
            let mod_val = match modification.op {
                ModOp::PreAssign => modification.val,
                ModOp::PreMul => modification.val,
                ModOp::PreDiv => 1.0 / modification.val,
                ModOp::Add => modification.val,
                ModOp::Sub => -modification.val,
                ModOp::PostMul => modification.val,
                ModOp::PostMulImmune => modification.val,
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
                    ModOp::PostMulImmune => dogma_val *= process_mults(vals),
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
                    self.calc_data.caps.add_cap(*item_id, capping_attr_id, *attr_id);
                    AttrVal::min(dogma_val, capping_vals.dogma)
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
}

fn penalize_vals(mut vals: Vec<AttrVal>) -> AttrVal {
    // Gather positive multipliers into one chain, negative into another, with stronger modifications
    // being first
    let positive = vals
        .extract_if(|v| *v > 1.0)
        .into_iter()
        .sorted_by(|a, b| b.partial_cmp(a).unwrap())
        .collect();
    let negative = vals.into_iter().sorted_by(|a, b| a.partial_cmp(b).unwrap()).collect();
    get_chain_val(positive) * get_chain_val(negative)
}

fn get_chain_val(vals: Vec<AttrVal>) -> AttrVal {
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

fn process_assigns(assigns: &Vec<AttrVal>, attr: &ad::AAttr) -> AttrVal {
    match attr.hig {
        true => *assigns.iter().max_by(|a, b| a.total_cmp(b)).unwrap(),
        false => *assigns.iter().min_by(|a, b| a.total_cmp(b)).unwrap(),
    }
}
fn process_mults(mults: &Vec<AttrVal>) -> AttrVal {
    let mut val = 1.0;
    mults.iter().for_each(|v| val *= v);
    val
}
fn process_adds(adds: &Vec<AttrVal>) -> AttrVal {
    let mut val = 0.0;
    adds.iter().for_each(|v| val += v);
    val
}
