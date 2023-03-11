use std::{cmp::Ordering, collections::HashMap, sync::Arc};

use itertools::Itertools;

use crate::{
    consts::{attrs, itemcats, ModAggrMode, ModOp, TgtMode},
    ct,
    ss::item::Item,
    ReeFloat, ReeId, ReeInt, Src,
};

use super::{affection_reg::AffectionRegister, modification::Modification};

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

pub(in crate::ss) struct CalcSvc {
    attrs: HashMap<ReeId, HashMap<ReeInt, ReeFloat>>,
    affection: AffectionRegister,
}
impl CalcSvc {
    pub(in crate::ss) fn new() -> CalcSvc {
        CalcSvc {
            attrs: HashMap::new(),
            affection: AffectionRegister::new(),
        }
    }
    // Query methods
    pub(in crate::ss) fn get_attr_val(
        &mut self,
        item_id: &ReeId,
        attr_id: &ReeInt,
        src: &Src,
        items: &HashMap<ReeId, Item>,
    ) -> Option<ReeFloat> {
        match self.attrs.get(item_id) {
            Some(attrs) => match attrs.get(attr_id) {
                Some(v) => return Some(*v),
                _ => (),
            },
            _ => (),
        };
        let val = match self.calc_attr(item_id, attr_id, src, items) {
            Some(v) => v,
            _ => return None,
        };
        match self.attrs.get_mut(item_id) {
            Some(attrs) => {
                attrs.insert(*attr_id, val);
                ();
            }
            _ => (),
        };
        Some(val)
    }
    // Maintenance methods
    pub(in crate::ss) fn item_loaded(&mut self, item: &Item) {
        self.attrs.insert(item.get_id(), HashMap::new());
        self.affection.reg_afee(item);
    }
    pub(in crate::ss) fn item_unloaded(&mut self, item: &Item) {
        self.affection.unreg_afee(item);
        self.attrs.remove(&item.get_id());
    }
    pub(in crate::ss) fn effects_started(&mut self, item: &Item, effects: &Vec<Arc<ct::Effect>>) {
        for effect in effects.iter().filter(|e| matches!(&e.tgt_mode, TgtMode::None)) {
            self.affection.reg_local_effect(item, effect);
        }
    }
    pub(in crate::ss) fn effects_stopped(&mut self, item: &Item, effects: &Vec<Arc<ct::Effect>>) {
        for effect in effects.iter().filter(|e| matches!(&e.tgt_mode, TgtMode::None)) {
            self.affection.unreg_local_effect(item, effect);
        }
    }
    // Private methods
    fn calc_attr(
        &mut self,
        item_id: &ReeId,
        attr_id: &ReeInt,
        src: &Src,
        items: &HashMap<ReeId, Item>,
    ) -> Option<ReeFloat> {
        let item = match items.get(item_id) {
            Some(i) => i,
            None => return None,
        };
        match (attr_id, item) {
            (280, Item::Skill(s)) => return Some(s.level as ReeFloat),
            _ => (),
        }
        let attr = match src.cache_handler.get_attr(attr_id) {
            Some(attr) => attr,
            None => return None,
        };
        // Get base value; use on-iteme original attributes, or, if not specified, default attribute value.
        // If both can't be fetched, consider it a failure
        let mut val = match item.get_orig_attrs() {
            Some(orig_attrs) => match orig_attrs.get(attr_id) {
                Some(orig_val) => *orig_val,
                None => match attr.def_val {
                    Some(def_val) => def_val,
                    None => return None,
                },
            },
            None => return None,
        };
        let mut stacked = HashMap::new();
        let mut stacked_penalized = HashMap::new();
        // let aggregate_min = Vec::new();
        // let aggregate_max = Vec::new();
        for modification in self.get_modifications(item, attr_id, src, items).iter() {
            let penalize =
                attr.penalizable && !modification.afor_pen_immune && PENALIZABLE_OPS.contains(&modification.op);
            let mod_val = match modification.op {
                ModOp::PreAssign => modification.val,
                ModOp::PreMul => modification.val - 1.0,
                ModOp::PreDiv => 1.0 / modification.val - 1.0,
                ModOp::Add => modification.val,
                ModOp::Sub => -modification.val,
                ModOp::PostMul => modification.val - 1.0,
                ModOp::PostDiv => 1.0 / modification.val - 1.0,
                ModOp::PostPerc => modification.val / 100.0,
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
        for op in OP_ORDER.iter() {
            match stacked.get(op) {
                Some(vals) => match op {
                    ModOp::PreAssign => {
                        val = process_assigns(vals, &attr);
                    }
                    ModOp::PreMul => val *= process_mults(vals),
                    ModOp::PreDiv => val *= process_mults(vals),
                    ModOp::Add => val += process_adds(vals),
                    ModOp::Sub => val += process_adds(vals),
                    ModOp::PostMul => val *= process_mults(vals),
                    ModOp::PostDiv => val *= process_mults(vals),
                    ModOp::PostPerc => val *= process_mults(vals),
                    ModOp::PostAssign => {
                        val = process_assigns(vals, &attr);
                    }
                },
                _ => (),
            }
        }
        // TODO: implement upper cap
        if LIMITED_PRECISION_ATTR_IDS.contains(attr_id) {
            val = (val * 100.0).round() / 100.0
        }
        Some(val)
    }
    fn get_modifications(
        &mut self,
        item: &Item,
        attr_id: &ReeInt,
        src: &Src,
        items: &HashMap<ReeId, Item>,
    ) -> Vec<Modification> {
        // TODO: optimize to pass attr ID to affector getter, and allocate vector with capacity
        let mut mods = Vec::new();
        for afor_spec in self.affection.get_afor_specs(item).iter() {
            let afor_mod = match afor_spec.get_modifier() {
                Some(m) => m,
                None => continue,
            };
            if &afor_mod.afee_attr_id != attr_id {
                continue;
            }
            let val = match self.get_attr_val(&afor_spec.item_id, &afor_mod.afor_attr_id, src, items) {
                Some(v) => v,
                None => continue,
            };
            let afor_item = match items.get(&afor_spec.item_id) {
                Some(i) => i,
                None => continue,
            };
            let pen_immune = match afor_item.get_category_id() {
                Some(cid) => PENALTY_IMMUNE_CATS.contains(&cid),
                None => continue,
            };
            // TODO: implement resistance support
            let modification = Modification::new(afor_mod.op, val, 1.0, ModAggrMode::Stack, pen_immune);
            mods.push(modification);
        }
        mods
    }
}

fn penalize_vals(mut vals: Vec<ReeFloat>) -> ReeFloat {
    // Gather positive multipliers into one chain, negative into another, with stronger modifications
    // being first
    let positive = vals
        .drain_filter(|v| *v > 0.0)
        .into_iter()
        .sorted_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less))
        .collect_vec();
    let negative = vals
        .into_iter()
        .sorted_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Less))
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
        val *= 1.0 + mod_val * PENALTY_BASE.powi((i as i32).pow(2));
    }
    val
}

fn process_assigns(assigns: &Vec<ReeFloat>, attr: &ct::Attr) -> ReeFloat {
    match attr.hig {
        true => *assigns.iter().max_by(|a, b| a.total_cmp(b)).unwrap(),
        false => *assigns.iter().min_by(|a, b| a.total_cmp(b)).unwrap(),
    }
}
fn process_mults(mults: &Vec<ReeFloat>) -> ReeFloat {
    let mut val = 1.0;
    mults.iter().for_each(|v| val *= (1.0 + v));
    val
}
fn process_adds(adds: &Vec<ReeFloat>) -> ReeFloat {
    let mut val = 0.0;
    adds.iter().for_each(|v| val += v);
    val
}
