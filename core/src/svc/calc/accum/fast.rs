//! This is attribute calculator designed to be used for attribute calculation.
//!
//! It has more bloated brother, which is built to calculate attribute value, and provide info about
//! what went into it. Since they duplicate each other, when doing any changes, MAKE SURE TO APPLY
//! THEM TO BOTH.

use std::{cmp::Ordering, collections::hash_map::Entry};

use super::shared::{PENALTY_MULTS, diminish_mul, is_penal, normalize_div, normalize_noop, normalize_perc};
use crate::{
    ad::AItemCatId,
    num::{PValue, Value},
    svc::calc::{AggrKey, AggrMode, CalcOp},
    util::RMap,
};

pub(in crate::svc::calc) struct ModAccumFast {
    pre_assign: ModAccumAssign,
    pre_mul: AttrStack,
    pre_div: AttrStack,
    add: ModAccumAdd,
    sub: ModAccumSub,
    post_mul: AttrStack,
    post_div: AttrStack,
    post_perc: AttrStack,
    post_assign: ModAccumAssign,
    extra_add: ModAccumAdd,
    extra_mul: AttrAggr,
    reuse_pen_chains: PenChains,
}
impl ModAccumFast {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
            pre_assign: ModAccumAssign::new(),
            pre_mul: AttrStack::new(),
            pre_div: AttrStack::new(),
            add: ModAccumAdd::new(),
            sub: ModAccumSub::new(),
            post_mul: AttrStack::new(),
            post_div: AttrStack::new(),
            post_perc: AttrStack::new(),
            post_assign: ModAccumAssign::new(),
            extra_add: ModAccumAdd::new(),
            extra_mul: AttrAggr::new(),
            reuse_pen_chains: PenChains::new(),
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
        match op {
            CalcOp::PreAssign => self.pre_assign.add_val(val, proj_mult, res_mult, aggr_mode, attr_hig),
            CalcOp::PreMul => self.pre_mul.add_val(
                val,
                proj_mult,
                res_mult,
                normalize_noop,
                diminish_mul,
                is_penal(attr_pen, &item_cat),
                &aggr_mode,
            ),
            CalcOp::PreDiv => self.pre_div.add_val(
                val,
                proj_mult,
                res_mult,
                normalize_div,
                diminish_mul,
                is_penal(attr_pen, &item_cat),
                &aggr_mode,
            ),
            CalcOp::Add => self.add.add_val(val, proj_mult, res_mult, aggr_mode),
            CalcOp::Sub => self.sub.add_val(val, proj_mult, res_mult, aggr_mode),
            CalcOp::PostMul => self.post_mul.add_val(
                val,
                proj_mult,
                res_mult,
                normalize_noop,
                diminish_mul,
                is_penal(attr_pen, &item_cat),
                &aggr_mode,
            ),
            CalcOp::PostMulImmune => self.post_mul.add_val(
                val,
                proj_mult,
                res_mult,
                normalize_noop,
                diminish_mul,
                false,
                &aggr_mode,
            ),
            CalcOp::PostDiv => self.post_div.add_val(
                val,
                proj_mult,
                res_mult,
                normalize_div,
                diminish_mul,
                is_penal(attr_pen, &item_cat),
                &aggr_mode,
            ),
            CalcOp::PostPerc => self.post_perc.add_val(
                val,
                proj_mult,
                res_mult,
                normalize_perc,
                diminish_mul,
                is_penal(attr_pen, &item_cat),
                &aggr_mode,
            ),
            CalcOp::PostPercImmune => self.post_perc.add_val(
                val,
                proj_mult,
                res_mult,
                normalize_perc,
                diminish_mul,
                false,
                &aggr_mode,
            ),
            CalcOp::PostAssign => self.post_assign.add_val(val, proj_mult, res_mult, aggr_mode, attr_hig),
            CalcOp::ExtraAdd => self.extra_add.add_val(val, proj_mult, res_mult, aggr_mode),
            CalcOp::ExtraMul => {
                self.extra_mul
                    .add_val(val, proj_mult, res_mult, normalize_noop, diminish_mul, &aggr_mode)
            }
        };
    }
    pub(in crate::svc::calc) fn apply_dogma_mods(&mut self, base_val: Value, attr_hig: bool) -> Value {
        let val = self.pre_assign.calc_val(base_val, attr_hig);
        let val = apply_mul(
            val,
            self.pre_mul
                .get_comb_val(combine_muls, combine_muls_pen, attr_hig, &mut self.reuse_pen_chains),
        );
        let val = apply_mul(
            val,
            self.pre_div
                .get_comb_val(combine_muls, combine_muls_pen, attr_hig, &mut self.reuse_pen_chains),
        );
        let val = self.add.calc_val(val);
        let val = self.sub.calc_val(val);
        let val = apply_mul(
            val,
            self.post_mul
                .get_comb_val(combine_muls, combine_muls_pen, attr_hig, &mut self.reuse_pen_chains),
        );
        let val = apply_mul(
            val,
            self.post_div
                .get_comb_val(combine_muls, combine_muls_pen, attr_hig, &mut self.reuse_pen_chains),
        );
        let val = apply_mul(
            val,
            self.post_perc
                .get_comb_val(combine_muls, combine_muls_pen, attr_hig, &mut self.reuse_pen_chains),
        );
        self.post_assign.calc_val(val, attr_hig)
    }
    pub(in crate::svc::calc) fn apply_extra_mods(&mut self, val: Value, hig: bool) -> Value {
        let val = self.extra_add.calc_val(val);
        apply_mul(
            val,
            self.extra_mul
                .get_comb_val(combine_muls, hig, &mut self.reuse_pen_chains),
        )
    }
}

struct AttrStack {
    stacked: AttrAggr,
    penalized: AttrAggr,
}
impl AttrStack {
    fn new() -> Self {
        Self {
            stacked: AttrAggr::new(),
            penalized: AttrAggr::new(),
        }
    }
    fn add_val<N, D>(
        &mut self,
        val: Value,
        proj_mult: Option<PValue>,
        res_mult: Option<PValue>,
        normalize_func: N,
        diminish_func: D,
        penalizable: bool,
        aggr_mode: &AggrMode,
    ) where
        N: Fn(Value) -> Option<Value>,
        D: Fn(Value, Option<PValue>, Option<PValue>) -> Value,
    {
        let attr_aggr = match penalizable {
            true => &mut self.penalized,
            false => &mut self.stacked,
        };
        attr_aggr.add_val(val, proj_mult, res_mult, normalize_func, diminish_func, aggr_mode)
    }
    fn get_comb_val<F1, F2>(
        &mut self,
        comb_func: F1,
        pen_func: F2,
        hig: bool,
        reuse_pen_chains: &mut PenChains,
    ) -> Option<Value>
    where
        F1: Fn(&[Value], bool, &mut PenChains) -> Option<Value>,
        F2: Fn(&[Value], bool, &mut PenChains) -> Option<Value>,
    {
        if let Some(val) = self.penalized.get_comb_val(pen_func, hig, reuse_pen_chains) {
            self.stacked.add_processed_val(val, &AggrMode::Stack);
        }
        self.stacked.get_comb_val(comb_func, hig, reuse_pen_chains)
    }
}

struct AttrAggr {
    stack: Vec<Value>,
    aggr_min: RMap<AggrKey, Vec<Value>>,
    aggr_max: RMap<AggrKey, Vec<Value>>,
}
impl AttrAggr {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            aggr_min: RMap::new(),
            aggr_max: RMap::new(),
        }
    }
    fn add_val<N, D>(
        &mut self,
        val: Value,
        proj_mult: Option<PValue>,
        res_mult: Option<PValue>,
        normalize_func: N,
        diminish_func: D,
        aggr_mode: &AggrMode,
    ) where
        N: Fn(Value) -> Option<Value>,
        D: Fn(Value, Option<PValue>, Option<PValue>) -> Value,
    {
        let Some(mut val) = normalize_func(val) else {
            return;
        };
        val = diminish_func(val, proj_mult, res_mult);
        self.add_processed_val(val, aggr_mode);
    }
    fn add_processed_val(&mut self, val: Value, aggr_mode: &AggrMode) {
        match aggr_mode {
            AggrMode::Stack => self.stack.push(val),
            AggrMode::Min(key) => self.aggr_min.entry(*key).or_default().push(val),
            AggrMode::Max(key) => self.aggr_max.entry(*key).or_default().push(val),
        }
    }
    fn get_comb_val<F>(&mut self, comb_func: F, attr_hig: bool, reuse_pen_chains: &mut PenChains) -> Option<Value>
    where
        F: Fn(&[Value], bool, &mut PenChains) -> Option<Value>,
    {
        // Resolve aggregations
        for vals in self.aggr_min.values() {
            if let Some(val) = get_min(vals) {
                self.stack.push(val);
            }
        }
        for vals in self.aggr_max.values() {
            if let Some(val) = get_max(vals) {
                self.stack.push(val);
            }
        }
        comb_func(&self.stack, attr_hig, reuse_pen_chains)
    }
}

struct PenChains {
    positive: Vec<Value>,
    negative: Vec<Value>,
}
impl PenChains {
    fn new() -> Self {
        Self {
            positive: Vec::new(),
            negative: Vec::new(),
        }
    }
    fn clear(&mut self) {
        self.positive.clear();
        self.negative.clear();
    }
    fn is_empty(&self) -> bool {
        self.positive.is_empty() && self.negative.is_empty()
    }
}

// Application functions
fn apply_mul(base_val: Value, other_val: Option<Value>) -> Value {
    match other_val {
        Some(other_val) => base_val * other_val,
        None => base_val,
    }
}

// Regular combination functions
fn combine_muls(vals: &[Value], _high_is_good: bool, _reuse_pen_chains: &mut PenChains) -> Option<Value> {
    if vals.is_empty() {
        return None;
    }
    Some(vals.iter().product())
}

// Penalized combination functions
fn combine_muls_pen(vals: &[Value], _high_is_good: bool, reuse_pen_chains: &mut PenChains) -> Option<Value> {
    // Gather positive multipliers into one chain, negative into another, with stronger
    // modifications being first
    reuse_pen_chains.clear();
    for val in vals.iter() {
        if *val > Value::ONE {
            reuse_pen_chains.positive.push(*val);
        } else if *val < Value::ONE {
            reuse_pen_chains.negative.push(*val);
        }
    }
    if reuse_pen_chains.is_empty() {
        return None;
    }
    reuse_pen_chains.positive.sort_unstable_by_key(|&v| -v);
    reuse_pen_chains.negative.sort_unstable();
    Some(get_chain_val(&reuse_pen_chains.positive) * get_chain_val(&reuse_pen_chains.negative))
}
fn get_chain_val(vals: &[Value]) -> Value {
    let mut val = Value::ONE;
    for (&mod_val, &mult) in std::iter::zip(vals.iter(), PENALTY_MULTS.iter()) {
        val *= (mod_val - Value::ONE).mul_add(mult.into_value(), Value::ONE);
    }
    val
}

// Misc functions
fn get_min(vals: &[Value]) -> Option<Value> {
    vals.iter().min().copied()
}
fn get_max(vals: &[Value]) -> Option<Value> {
    vals.iter().max().copied()
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// NEW IMPL AHEAD
// TODO: check if iterating over values + copying is better than draining
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
// Operation-specific containers
////////////////////////////////////////////////////////////////////////////////////////////////////
struct ModAccumAssign {
    main: Option<Value>,
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
    fn add_val(
        &mut self,
        val: Value,
        proj_mult: Option<PValue>,
        res_mult: Option<PValue>,
        aggr_mode: AggrMode,
        attr_hig: bool,
    ) {
        // Multipliers affect assign operations differently: if any of multipliers is 0.0, then
        // modification is not applied altogether, otherwise it is applied fully. There are no such
        // modifiers in EVE, but the lib makes it to work this way.
        if proj_mult == Some(PValue::ZERO) || res_mult == Some(PValue::ZERO) {
            return;
        };
        match aggr_mode {
            // Overwrite main value only if there is no value yet, or if passed value is "better",
            // according to high-is-good flag
            AggrMode::Stack => match &self.main {
                Some(main) => {
                    if let (Ordering::Greater, true) | (Ordering::Less, false) = (val.cmp(main), attr_hig) {
                        self.main = Some(val)
                    }
                }
                None => self.main = Some(val),
            },
            AggrMode::Min(key) => self.aggr_min.add_val(key, val),
            AggrMode::Max(key) => self.aggr_max.add_val(key, val),
        }
    }
    fn calc_val(&mut self, mut val: Value, attr_hig: bool) -> Value {
        let iter_main = self.main.into_iter();
        let iter_min = self.aggr_min.drain_values();
        let iter_max = self.aggr_max.drain_values();
        let chain = iter_main.chain(iter_min).chain(iter_max);
        // Pick best value across all containers, and use it
        if let Some(best_assignment) = match attr_hig {
            true => chain.max(),
            false => chain.min(),
        } {
            val = best_assignment;
        }
        val
    }
}

struct ModAccumAdd {
    main: Value,
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
    fn add_val(&mut self, mut val: Value, proj_mult: Option<PValue>, res_mult: Option<PValue>, aggr_mode: AggrMode) {
        if let Some(proj_mult) = proj_mult {
            val *= proj_mult;
        }
        if let Some(res_mult) = res_mult {
            val *= res_mult;
        }
        match aggr_mode {
            AggrMode::Stack => self.main += val,
            AggrMode::Min(key) => self.aggr_min.add_val(key, val),
            AggrMode::Max(key) => self.aggr_max.add_val(key, val),
        }
    }
    fn calc_val(&mut self, val: Value) -> Value {
        val + self.main + self.aggr_min.drain_values().sum() + self.aggr_max.drain_values().sum()
    }
}

struct ModAccumSub {
    main: Value,
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
    fn add_val(&mut self, mut val: Value, proj_mult: Option<PValue>, res_mult: Option<PValue>, aggr_mode: AggrMode) {
        if let Some(proj_mult) = proj_mult {
            val *= proj_mult;
        }
        if let Some(res_mult) = res_mult {
            val *= res_mult;
        }
        match aggr_mode {
            AggrMode::Stack => self.main += val,
            AggrMode::Min(key) => self.aggr_min.add_val(key, val),
            AggrMode::Max(key) => self.aggr_max.add_val(key, val),
        }
    }
    fn calc_val(&mut self, val: Value) -> Value {
        val - (self.main + self.aggr_min.drain_values().sum() + self.aggr_max.drain_values().sum())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Key-based aggregation maps
////////////////////////////////////////////////////////////////////////////////////////////////////
struct AggrMin {
    data: RMap<AggrKey, Value>,
}
impl AggrMin {
    fn new() -> Self {
        Self { data: RMap::new() }
    }
    fn add_val(&mut self, aggr_key: AggrKey, val: Value) {
        match self.data.entry(aggr_key) {
            Entry::Occupied(mut entry) => {
                if val < *entry.get() {
                    entry.insert(val);
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(val);
            }
        }
    }
    fn drain_values(&mut self) -> impl ExactSizeIterator<Item = Value> {
        self.data.drain().map(|v| v.1)
    }
}

struct AggrMax {
    data: RMap<AggrKey, Value>,
}
impl AggrMax {
    fn new() -> Self {
        Self { data: RMap::new() }
    }
    fn add_val(&mut self, aggr_key: AggrKey, val: Value) {
        match self.data.entry(aggr_key) {
            Entry::Occupied(mut entry) => {
                if val > *entry.get() {
                    entry.insert(val);
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(val);
            }
        }
    }
    fn drain_values(&mut self) -> impl ExactSizeIterator<Item = Value> {
        self.data.drain().map(|v| v.1)
    }
}
