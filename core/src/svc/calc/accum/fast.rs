//! This is attribute calculator designed to be used for attribute calculation.
//!
//! It has more bloated brother, which is built to calculate attribute value, and provide info about
//! what went into it. Since they duplicate each other, when doing any changes, MAKE SURE TO APPLY
//! THEM TO BOTH.

use std::{cmp::Ordering, collections::hash_map::Entry};

use super::shared::{PENALTY_MULTS, is_penal};
use crate::{
    ad::AItemCatId,
    num::{PValue, Value},
    svc::calc::{AggrKey, AggrMode, CalcOp},
    util::RMap,
};

pub(in crate::svc::calc) struct ModAccumFast {
    pre_assign: ModAccumAssign,
    pre_mul: ModAccumMul,
    pre_div: ModAccumDiv,
    add: ModAccumAdd,
    sub: ModAccumSub,
    post_mul: ModAccumMul,
    post_div: ModAccumDiv,
    post_perc: ModAccumPerc,
    post_assign: ModAccumAssign,
    extra_add: ModAccumAdd,
    extra_mul: ModAccumMul,
}
impl ModAccumFast {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
            pre_assign: ModAccumAssign::new(),
            pre_mul: ModAccumMul::new(),
            pre_div: ModAccumDiv::new(),
            add: ModAccumAdd::new(),
            sub: ModAccumSub::new(),
            post_mul: ModAccumMul::new(),
            post_div: ModAccumDiv::new(),
            post_perc: ModAccumPerc::new(),
            post_assign: ModAccumAssign::new(),
            extra_add: ModAccumAdd::new(),
            extra_mul: ModAccumMul::new(),
        }
    }
    pub(in crate::svc::calc) fn add_val(
        &mut self,
        val: Value,
        op: CalcOp,
        proj_mult: Option<PValue>,
        res_mult: Option<PValue>,
        attr_pen: bool,
        item_cat: AItemCatId,
        aggr_mode: AggrMode,
        attr_hig: bool,
    ) {
        let comb_mult = proj_mult.reduce(res_mult, |x, y| x * y);
        match op {
            CalcOp::PreAssign => self.pre_assign.add_val(val, comb_mult, aggr_mode, attr_hig),
            CalcOp::PreMul => self
                .pre_mul
                .add_val(val, comb_mult, aggr_mode, is_penal(attr_pen, &item_cat)),
            CalcOp::PreDiv => self
                .pre_div
                .add_val(val, comb_mult, aggr_mode, is_penal(attr_pen, &item_cat)),
            CalcOp::Add => self.add.add_val(val, comb_mult, aggr_mode),
            CalcOp::Sub => self.sub.add_val(val, comb_mult, aggr_mode),
            CalcOp::PostMul => self
                .post_mul
                .add_val(val, comb_mult, aggr_mode, is_penal(attr_pen, &item_cat)),
            CalcOp::PostMulImmune => self.post_mul.add_val(val, comb_mult, aggr_mode, false),
            CalcOp::PostDiv => self
                .post_div
                .add_val(val, comb_mult, aggr_mode, is_penal(attr_pen, &item_cat)),
            CalcOp::PostPerc => self
                .post_perc
                .add_val(val, comb_mult, aggr_mode, is_penal(attr_pen, &item_cat)),
            CalcOp::PostPercImmune => self.post_perc.add_val(val, comb_mult, aggr_mode, false),
            CalcOp::PostAssign => self.post_assign.add_val(val, comb_mult, aggr_mode, attr_hig),
            CalcOp::ExtraAdd => self.extra_add.add_val(val, comb_mult, aggr_mode),
            CalcOp::ExtraMul => self
                .extra_mul
                .add_val(val, comb_mult, aggr_mode, is_penal(attr_pen, &item_cat)),
        };
    }
    pub(in crate::svc::calc) fn apply_dogma_mods(&mut self, base_val: Value, attr_hig: bool) -> Value {
        let val = self.pre_assign.calc_val(base_val, attr_hig);
        let val = self.pre_mul.calc_val(val);
        let val = self.pre_div.calc_val(val);
        let val = self.add.calc_val(val);
        let val = self.sub.calc_val(val);
        let val = self.post_mul.calc_val(val);
        let val = self.post_div.calc_val(val);
        let val = self.post_perc.calc_val(val);
        self.post_assign.calc_val(val, attr_hig)
    }
    pub(in crate::svc::calc) fn apply_extra_mods(&mut self, val: Value) -> Value {
        let val = self.extra_add.calc_val(val);
        self.extra_mul.calc_val(val)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Assignment
////////////////////////////////////////////////////////////////////////////////////////////////////
struct ModAccumAssign {
    // Best seen non-aggregable assignment
    main: Option<Value>,
    // Aggregable assignments
    aggr_min: AggrMin,
    aggr_max: AggrMax,
}
impl ModAccumAssign {
    fn new() -> Self {
        Self {
            main: None,
            aggr_min: AggrMin::new(),
            aggr_max: AggrMax::new(),
        }
    }
    fn add_val(&mut self, val: Value, comb_mult: Option<PValue>, aggr_mode: AggrMode, attr_hig: bool) {
        // Projection/resist multipliers affect assign operations differently: if any of multipliers
        // is 0.0, then modification is not applied altogether, otherwise it is applied fully. There
        // are no such modifiers in EVE, but the lib makes it to work this way.
        if comb_mult == Some(PValue::ZERO) {
            return;
        };
        match aggr_mode {
            // Overwrite main value only if there is no stored assignment yet, or if passed
            // assignment is "better", according to the high-is-good flag
            AggrMode::Stack => match &self.main {
                Some(main) => {
                    if let (Ordering::Greater, true) | (Ordering::Less, false) = (val.cmp(main), attr_hig) {
                        self.main = Some(val)
                    }
                }
                None => self.main = Some(val),
            },
            // Store asignment in its original format for aggregation
            AggrMode::Min(key) => self.aggr_min.add_val(key, val),
            AggrMode::Max(key) => self.aggr_max.add_val(key, val),
        }
    }
    fn calc_val(&mut self, mut val: Value, attr_hig: bool) -> Value {
        let iter_main = self.main.into_iter();
        let iter_min = self.aggr_min.drain_values();
        let iter_max = self.aggr_max.drain_values();
        let chain = iter_main.chain(iter_min).chain(iter_max);
        // Pick best assignment across all containers, and use it
        if let Some(best_value) = match attr_hig {
            true => chain.max(),
            false => chain.min(),
        } {
            val = best_value;
        }
        val
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Addition/subtraction
////////////////////////////////////////////////////////////////////////////////////////////////////
struct ModAccumAdd {
    // Non-aggregable, folded sum of increases
    main: Value,
    // Aggregable, increases
    aggr_min: AggrMin,
    aggr_max: AggrMax,
}
impl ModAccumAdd {
    fn new() -> Self {
        Self {
            main: Value::ZERO,
            aggr_min: AggrMin::new(),
            aggr_max: AggrMax::new(),
        }
    }
    fn add_val(&mut self, mut val: Value, comb_mult: Option<PValue>, aggr_mode: AggrMode) {
        if let Some(comb_mult) = comb_mult {
            val *= comb_mult;
        }
        match aggr_mode {
            AggrMode::Stack => self.main += val,
            // Store addition in its original format for aggregation
            AggrMode::Min(key) => self.aggr_min.add_val(key, val),
            AggrMode::Max(key) => self.aggr_max.add_val(key, val),
        }
    }
    fn calc_val(&mut self, val: Value) -> Value {
        val + self.main + self.aggr_min.drain_values().sum() + self.aggr_max.drain_values().sum()
    }
}

struct ModAccumSub {
    // Non-aggregable, folded sum of decreases
    main: Value,
    // Aggregable, decreases
    aggr_min: AggrMin,
    aggr_max: AggrMax,
}
impl ModAccumSub {
    fn new() -> Self {
        Self {
            main: Value::ZERO,
            aggr_min: AggrMin::new(),
            aggr_max: AggrMax::new(),
        }
    }
    fn add_val(&mut self, mut val: Value, comb_mult: Option<PValue>, aggr_mode: AggrMode) {
        if let Some(comb_mult) = comb_mult {
            val *= comb_mult;
        }
        match aggr_mode {
            AggrMode::Stack => self.main += val,
            // Store decrease in its original format for aggregation
            AggrMode::Min(key) => self.aggr_min.add_val(key, val),
            AggrMode::Max(key) => self.aggr_max.add_val(key, val),
        }
    }
    fn calc_val(&mut self, val: Value) -> Value {
        // Since all the values were stored/folded in original format of decreases/subtractions,
        // subtract their sum from passed value
        val - (self.main + self.aggr_min.drain_values().sum() + self.aggr_max.drain_values().sum())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Multiplication/division
////////////////////////////////////////////////////////////////////////////////////////////////////
struct ModAccumMul {
    // Non-aggregable non-penalizable, folded multiplier
    main: Value,
    // Non-aggregable penalizable values, multiplier - 1
    pen_pos: Vec<Value>,
    pen_neg: Vec<Value>,
    // Aggregable values, multiplier
    aggr_min: AggrPenMin,
    aggr_max: AggrPenMax,
}
impl ModAccumMul {
    fn new() -> Self {
        Self {
            main: Value::ONE,
            pen_pos: Vec::new(),
            pen_neg: Vec::new(),
            aggr_min: AggrPenMin::new(),
            aggr_max: AggrPenMax::new(),
        }
    }
    fn add_val(&mut self, mut val: Value, comb_mult: Option<PValue>, aggr_mode: AggrMode, pen: bool) {
        if let Some(comb_mult) = comb_mult {
            // Convert multiplier into increase relatively zero before applying multiplier, and then
            // convert back
            val = (val - Value::ONE).mul_add(comb_mult.into_value(), Value::ONE);
        }
        match aggr_mode {
            AggrMode::Stack => match pen {
                true => {
                    // Convert mult into mult - 1 to store in penalizable value containers
                    val -= Value::ONE;
                    match val.cmp(&Value::ZERO) {
                        Ordering::Greater => self.pen_pos.push(val),
                        Ordering::Equal => (),
                        Ordering::Less => self.pen_neg.push(val),
                    }
                }
                // Store in main multiplier in case of stacked & unpenalized modification
                false => self.main *= val,
            },
            // Store multiplier in its original format for aggregation
            AggrMode::Min(key) => self.aggr_min.add_val(key, PenEntry { val, pen }),
            AggrMode::Max(key) => self.aggr_max.add_val(key, PenEntry { val, pen }),
        }
    }
    fn calc_val(&mut self, val: Value) -> Value {
        // Distribute aggregable values first
        let iter_min = self.aggr_min.drain_values();
        let iter_max = self.aggr_max.drain_values();
        for aggr_entry in iter_min.chain(iter_max) {
            match aggr_entry.pen {
                true => {
                    // Convert mult into mult - 1 to store in penalizable value containers
                    let aggr_val = aggr_entry.val - Value::ONE;
                    match aggr_val.cmp(&Value::ZERO) {
                        Ordering::Greater => self.pen_pos.push(aggr_val),
                        Ordering::Equal => (),
                        Ordering::Less => self.pen_neg.push(aggr_val),
                    }
                }
                // Fold into main value in case value coming from aggregators is not penalizable
                false => self.main *= aggr_entry.val,
            }
        }
        // Resolve penalization chains & calculate final value
        self.pen_pos.sort_unstable_by_key(|&v| -v);
        self.pen_neg.sort_unstable();
        val * self.main * get_penalty_chain_mult(&self.pen_pos) * get_penalty_chain_mult(&self.pen_neg)
    }
}

struct ModAccumDiv {
    // Non-aggregable non-penalizable, folded divisor
    main: Value,
    // Non-aggregable penalizable values, multiplier - 1
    pen_pos: Vec<Value>,
    pen_neg: Vec<Value>,
    // Aggregable values, divisor
    aggr_min: AggrPenMin,
    aggr_max: AggrPenMax,
}
impl ModAccumDiv {
    fn new() -> Self {
        Self {
            main: Value::ONE,
            pen_pos: Vec::new(),
            pen_neg: Vec::new(),
            aggr_min: AggrPenMin::new(),
            aggr_max: AggrPenMax::new(),
        }
    }
    fn add_val(&mut self, mut val: Value, comb_mult: Option<PValue>, aggr_mode: AggrMode, pen: bool) {
        // Ignore division by zero early
        if val == Value::ZERO {
            return;
        }
        if let Some(comb_mult) = comb_mult {
            // Convert divisor into increase relatively zero before applying multiplier, and then
            // convert back
            val = Value::ONE / (Value::ONE / val - Value::ONE).mul_add(comb_mult.into_value(), Value::ONE);
        }
        match aggr_mode {
            AggrMode::Stack => match pen {
                true => {
                    // Convert divisor into mult - 1 to store in penalizable value containers
                    val = Value::ONE / val - Value::ONE;
                    match val.cmp(&Value::ZERO) {
                        Ordering::Greater => self.pen_pos.push(val),
                        Ordering::Equal => (),
                        Ordering::Less => self.pen_neg.push(val),
                    }
                }
                // Store in main divisor in case of stacked & unpenalized modification
                false => self.main *= val,
            },
            // Store divisor in its original format for aggregation
            AggrMode::Min(key) => self.aggr_min.add_val(key, PenEntry { val, pen }),
            AggrMode::Max(key) => self.aggr_max.add_val(key, PenEntry { val, pen }),
        }
    }
    fn calc_val(&mut self, val: Value) -> Value {
        // Distribute aggregable values first
        let iter_min = self.aggr_min.drain_values();
        let iter_max = self.aggr_max.drain_values();
        for aggr_entry in iter_min.chain(iter_max) {
            match aggr_entry.pen {
                true => {
                    // Convert divisor into mult - 1 to store in penalizable value containers
                    let aggr_val = Value::ONE / aggr_entry.val - Value::ONE;
                    match aggr_val.cmp(&Value::ZERO) {
                        Ordering::Greater => self.pen_pos.push(aggr_val),
                        Ordering::Equal => (),
                        Ordering::Less => self.pen_neg.push(aggr_val),
                    }
                }
                // Unpenalizable divisors are folded into main value
                false => self.main *= aggr_entry.val,
            }
        }
        // Resolve penalization chains & calculate final value
        self.pen_pos.sort_unstable_by_key(|&v| -v);
        self.pen_neg.sort_unstable();
        val / self.main * get_penalty_chain_mult(&self.pen_pos) * get_penalty_chain_mult(&self.pen_neg)
    }
}

struct ModAccumPerc {
    // Non-aggregable non-penalizable, folded multiplier
    main: Value,
    // Non-aggregable penalizable values, multiplier - 1
    pen_pos: Vec<Value>,
    pen_neg: Vec<Value>,
    // Aggregable values, percent change
    aggr_min: AggrPenMin,
    aggr_max: AggrPenMax,
}
impl ModAccumPerc {
    fn new() -> Self {
        Self {
            main: Value::ONE,
            pen_pos: Vec::new(),
            pen_neg: Vec::new(),
            aggr_min: AggrPenMin::new(),
            aggr_max: AggrPenMax::new(),
        }
    }
    fn add_val(&mut self, mut val: Value, comb_mult: Option<PValue>, aggr_mode: AggrMode, pen: bool) {
        if let Some(comb_mult) = comb_mult {
            val *= comb_mult;
        }
        match aggr_mode {
            AggrMode::Stack => match pen {
                true => {
                    // Convert percent change into mult - 1 to store in penalizable value containers
                    val *= Value::HUNDREDTH;
                    match val.cmp(&Value::ZERO) {
                        Ordering::Greater => self.pen_pos.push(val),
                        Ordering::Equal => (),
                        Ordering::Less => self.pen_neg.push(val),
                    }
                }
                false => self.main *= val.mul_add(Value::HUNDREDTH, Value::ONE),
            },
            // Store percent change in its original format for aggregation
            AggrMode::Min(key) => self.aggr_min.add_val(key, PenEntry { val, pen }),
            AggrMode::Max(key) => self.aggr_max.add_val(key, PenEntry { val, pen }),
        }
    }
    fn calc_val(&mut self, val: Value) -> Value {
        // Distribute aggregable values first
        let iter_min = self.aggr_min.drain_values();
        let iter_max = self.aggr_max.drain_values();
        for aggr_entry in iter_min.chain(iter_max) {
            match aggr_entry.pen {
                true => {
                    // Convert percent change into mult - 1 to store in penalizable value containers
                    let aggr_val = aggr_entry.val * Value::HUNDREDTH;
                    match aggr_val.cmp(&Value::ZERO) {
                        Ordering::Greater => self.pen_pos.push(aggr_val),
                        Ordering::Equal => (),
                        Ordering::Less => self.pen_neg.push(aggr_val),
                    }
                }
                false => self.main *= aggr_entry.val.mul_add(Value::HUNDREDTH, Value::ONE),
            }
        }
        // Resolve penalization chains & calculate final value
        self.pen_pos.sort_unstable_by_key(|&v| -v);
        self.pen_neg.sort_unstable();
        val * self.main * get_penalty_chain_mult(&self.pen_pos) * get_penalty_chain_mult(&self.pen_neg)
    }
}

// Take a slice of mult - 1's, return mult
fn get_penalty_chain_mult(vals: &[Value]) -> Value {
    let mut val = Value::ONE;
    for (&mod_val, &mult) in std::iter::zip(vals.iter(), PENALTY_MULTS.iter()) {
        val *= mod_val.mul_add(mult.into_value(), Value::ONE);
    }
    val
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Key-based aggregation maps
////////////////////////////////////////////////////////////////////////////////////////////////////
type AggrMin = AggrStore<Value, AggrArbiterValMin>;
type AggrMax = AggrStore<Value, AggrArbiterValMax>;
type AggrPenMin = AggrStore<PenEntry, AggrArbiterPenMin>;
type AggrPenMax = AggrStore<PenEntry, AggrArbiterPenMax>;

struct PenEntry {
    val: Value,
    pen: bool,
}

struct AggrStore<T, R> {
    data: RMap<AggrKey, T>,
    arbiter: std::marker::PhantomData<R>,
}
impl<T, R> AggrStore<T, R> {
    fn new() -> Self {
        Self {
            data: RMap::new(),
            arbiter: std::marker::PhantomData,
        }
    }
    fn add_val(&mut self, aggr_key: AggrKey, added: T)
    where
        R: AggrArbiter<Item = T>,
    {
        match self.data.entry(aggr_key) {
            Entry::Occupied(mut entry) => {
                if R::is_added_better(entry.get(), &added) {
                    entry.insert(added);
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(added);
            }
        }
    }
    fn drain_values(&mut self) -> impl ExactSizeIterator<Item = T> {
        self.data.drain().map(|v| v.1)
    }
}

trait AggrArbiter {
    type Item;
    fn is_added_better(stored: &Self::Item, added: &Self::Item) -> bool;
}

struct AggrArbiterValMin;
impl AggrArbiter for AggrArbiterValMin {
    type Item = Value;
    fn is_added_better(stored: &Self::Item, added: &Self::Item) -> bool {
        added < stored
    }
}
struct AggrArbiterValMax;
impl AggrArbiter for AggrArbiterValMax {
    type Item = Value;
    fn is_added_better(stored: &Self::Item, added: &Self::Item) -> bool {
        added > stored
    }
}
struct AggrArbiterPenMin;
impl AggrArbiter for AggrArbiterPenMin {
    type Item = PenEntry;
    fn is_added_better(stored: &Self::Item, added: &Self::Item) -> bool {
        match added.val.cmp(&stored.val) {
            Ordering::Greater => false,
            Ordering::Equal => !added.pen && stored.pen,
            Ordering::Less => true,
        }
    }
}
struct AggrArbiterPenMax;
impl AggrArbiter for AggrArbiterPenMax {
    type Item = PenEntry;
    fn is_added_better(stored: &Self::Item, added: &Self::Item) -> bool {
        match added.val.cmp(&stored.val) {
            Ordering::Greater => true,
            Ordering::Equal => !added.pen && stored.pen,
            Ordering::Less => false,
        }
    }
}
