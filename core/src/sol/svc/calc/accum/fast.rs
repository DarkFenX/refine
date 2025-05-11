//! This is attribute calculator designed to be used for attribute calculation.
//!
//! It has more bloated brother, which is built to calculate attribute value, and provide info about
//! what went into it. Since they duplicate each other, when doing any changes, MAKE SURE TO APPLY
//! THEM TO BOTH.

use ordered_float::OrderedFloat as OF;

use super::shared::{
    PENALTY_BASE, PENALTY_SIGNIFICANT_MODIFICATIONS, diminish_basic, diminish_mul, diminish_noop, is_penal,
    normalize_div, normalize_noop, normalize_perc, normalize_sub,
};
use crate::{
    ad,
    sol::{
        AttrVal,
        svc::calc::{AggrKey, AggrMode, Op},
    },
    util::RMap,
};

pub(in crate::sol::svc::calc) struct ModAccumFast {
    pre_assign: AttrAggr,
    pre_mul: AttrStack,
    pre_div: AttrStack,
    add: AttrAggr,
    sub: AttrAggr,
    post_mul: AttrStack,
    post_div: AttrStack,
    post_perc: AttrStack,
    post_assign: AttrAggr,
    extra_add: AttrAggr,
    extra_mul: AttrAggr,
    pen_chains: PenChains,
}
impl ModAccumFast {
    pub(in crate::sol::svc::calc) fn new() -> Self {
        Self {
            pre_assign: AttrAggr::new(),
            pre_mul: AttrStack::new(),
            pre_div: AttrStack::new(),
            add: AttrAggr::new(),
            sub: AttrAggr::new(),
            post_mul: AttrStack::new(),
            post_div: AttrStack::new(),
            post_perc: AttrStack::new(),
            post_assign: AttrAggr::new(),
            extra_add: AttrAggr::new(),
            extra_mul: AttrAggr::new(),
            pen_chains: PenChains::new(),
        }
    }
    pub(in crate::sol::svc::calc) fn add_val(
        &mut self,
        val: AttrVal,
        proj_mult: Option<AttrVal>,
        res_mult: Option<AttrVal>,
        op: &Op,
        attr_pen: bool,
        a_item_cat: &ad::AItemCatId,
        aggr_mode: &AggrMode,
    ) {
        match op {
            Op::PreAssign => self
                .pre_assign
                .add_val(val, None, None, normalize_noop, diminish_noop, aggr_mode),
            Op::PreMul => self.pre_mul.add_val(
                val,
                proj_mult,
                res_mult,
                normalize_noop,
                diminish_mul,
                is_penal(attr_pen, a_item_cat),
                aggr_mode,
            ),
            Op::PreDiv => self.pre_div.add_val(
                val,
                proj_mult,
                res_mult,
                normalize_div,
                diminish_mul,
                is_penal(attr_pen, a_item_cat),
                aggr_mode,
            ),
            Op::Add => self
                .add
                .add_val(val, proj_mult, res_mult, normalize_noop, diminish_basic, aggr_mode),
            Op::Sub => self
                .sub
                .add_val(val, proj_mult, res_mult, normalize_sub, diminish_basic, aggr_mode),
            Op::PostMul => self.post_mul.add_val(
                val,
                proj_mult,
                res_mult,
                normalize_noop,
                diminish_mul,
                is_penal(attr_pen, a_item_cat),
                aggr_mode,
            ),
            Op::PostMulImmune => {
                self.post_mul
                    .add_val(val, proj_mult, res_mult, normalize_noop, diminish_mul, false, aggr_mode)
            }
            Op::PostDiv => self.post_div.add_val(
                val,
                proj_mult,
                res_mult,
                normalize_div,
                diminish_mul,
                is_penal(attr_pen, a_item_cat),
                aggr_mode,
            ),
            Op::PostPerc => self.post_perc.add_val(
                val,
                proj_mult,
                res_mult,
                normalize_perc,
                diminish_mul,
                is_penal(attr_pen, a_item_cat),
                aggr_mode,
            ),
            Op::PostPercImmune => {
                self.post_perc
                    .add_val(val, proj_mult, res_mult, normalize_perc, diminish_mul, false, aggr_mode)
            }
            Op::PostAssign => self
                .post_assign
                .add_val(val, None, None, normalize_noop, diminish_noop, aggr_mode),
            Op::ExtraAdd => self
                .extra_add
                .add_val(val, proj_mult, res_mult, normalize_noop, diminish_basic, aggr_mode),
            Op::ExtraMul => self
                .extra_mul
                .add_val(val, proj_mult, res_mult, normalize_noop, diminish_mul, aggr_mode),
        };
    }
    pub(in crate::sol::svc::calc) fn apply_dogma_mods(&mut self, base_val: AttrVal, hig: bool) -> AttrVal {
        let val = apply_assign(
            base_val,
            self.pre_assign.get_comb_val(combine_assigns, hig, &mut self.pen_chains),
        );
        let val = apply_mul(
            val,
            self.pre_mul
                .get_comb_val(combine_muls, combine_muls_pen, hig, &mut self.pen_chains),
        );
        let val = apply_mul(
            val,
            self.pre_div
                .get_comb_val(combine_muls, combine_muls_pen, hig, &mut self.pen_chains),
        );
        let val = apply_add(val, self.add.get_comb_val(combine_adds, hig, &mut self.pen_chains));
        let val = apply_add(val, self.sub.get_comb_val(combine_adds, hig, &mut self.pen_chains));
        let val = apply_mul(
            val,
            self.post_mul
                .get_comb_val(combine_muls, combine_muls_pen, hig, &mut self.pen_chains),
        );
        let val = apply_mul(
            val,
            self.post_div
                .get_comb_val(combine_muls, combine_muls_pen, hig, &mut self.pen_chains),
        );
        let val = apply_mul(
            val,
            self.post_perc
                .get_comb_val(combine_muls, combine_muls_pen, hig, &mut self.pen_chains),
        );
        apply_assign(
            val,
            self.post_assign
                .get_comb_val(combine_assigns, hig, &mut self.pen_chains),
        )
    }
    pub(in crate::sol::svc::calc) fn apply_extra_mods(&mut self, val: AttrVal, hig: bool) -> AttrVal {
        let val = apply_add(
            val,
            self.extra_add.get_comb_val(combine_adds, hig, &mut self.pen_chains),
        );
        apply_mul(
            val,
            self.extra_mul.get_comb_val(combine_muls, hig, &mut self.pen_chains),
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
        val: AttrVal,
        proj_mult: Option<AttrVal>,
        res_mult: Option<AttrVal>,
        normalize_func: N,
        diminish_func: D,
        penalizable: bool,
        aggr_mode: &AggrMode,
    ) where
        N: Fn(AttrVal) -> Option<AttrVal>,
        D: Fn(AttrVal, Option<AttrVal>, Option<AttrVal>) -> AttrVal,
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
        pen_chains: &mut PenChains,
    ) -> Option<AttrVal>
    where
        F1: Fn(&[AttrVal], bool, &mut PenChains) -> Option<AttrVal>,
        F2: Fn(&[AttrVal], bool, &mut PenChains) -> Option<AttrVal>,
    {
        if let Some(val) = self.penalized.get_comb_val(pen_func, hig, pen_chains) {
            self.stacked.add_processed_val(val, &AggrMode::Stack);
        }
        self.stacked.get_comb_val(comb_func, hig, pen_chains)
    }
}

struct AttrAggr {
    stack: Vec<AttrVal>,
    aggr_min: RMap<AggrKey, Vec<AttrVal>>,
    aggr_max: RMap<AggrKey, Vec<AttrVal>>,
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
        val: AttrVal,
        proj_mult: Option<AttrVal>,
        res_mult: Option<AttrVal>,
        normalize_func: N,
        diminish_func: D,
        aggr_mode: &AggrMode,
    ) where
        N: Fn(AttrVal) -> Option<AttrVal>,
        D: Fn(AttrVal, Option<AttrVal>, Option<AttrVal>) -> AttrVal,
    {
        let mut val = match normalize_func(val) {
            Some(val) => val,
            None => return,
        };
        val = diminish_func(val, proj_mult, res_mult);
        self.add_processed_val(val, aggr_mode);
    }
    fn add_processed_val(&mut self, val: AttrVal, aggr_mode: &AggrMode) {
        match aggr_mode {
            AggrMode::Stack => self.stack.push(val),
            AggrMode::Min(key) => self.aggr_min.entry(*key).or_default().push(val),
            AggrMode::Max(key) => self.aggr_max.entry(*key).or_default().push(val),
        }
    }
    fn get_comb_val<F>(&mut self, comb_func: F, high_is_good: bool, pen_chains: &mut PenChains) -> Option<AttrVal>
    where
        F: Fn(&[AttrVal], bool, &mut PenChains) -> Option<AttrVal>,
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
        comb_func(&self.stack, high_is_good, pen_chains)
    }
}

struct PenChains {
    positive: Vec<AttrVal>,
    negative: Vec<AttrVal>,
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
fn apply_assign(base_val: AttrVal, other_val: Option<AttrVal>) -> AttrVal {
    other_val.unwrap_or(base_val)
}
fn apply_add(base_val: AttrVal, other_val: Option<AttrVal>) -> AttrVal {
    match other_val {
        Some(other_val) => base_val + other_val,
        None => base_val,
    }
}
fn apply_mul(base_val: AttrVal, other_val: Option<AttrVal>) -> AttrVal {
    match other_val {
        Some(other_val) => base_val * other_val,
        None => base_val,
    }
}

// Regular combination functions
fn combine_assigns(vals: &[AttrVal], high_is_good: bool, _pen_chains: &mut PenChains) -> Option<AttrVal> {
    match high_is_good {
        true => get_max(vals),
        false => get_min(vals),
    }
}
fn combine_adds(vals: &[AttrVal], _high_is_good: bool, _pen_chains: &mut PenChains) -> Option<AttrVal> {
    if vals.is_empty() {
        return None;
    }
    Some(vals.iter().sum())
}
fn combine_muls(vals: &[AttrVal], _high_is_good: bool, _pen_chains: &mut PenChains) -> Option<AttrVal> {
    if vals.is_empty() {
        return None;
    }
    Some(vals.iter().product())
}

// Penalized combination functions
fn combine_muls_pen(vals: &[AttrVal], _high_is_good: bool, pen_chains: &mut PenChains) -> Option<AttrVal> {
    // Gather positive multipliers into one chain, negative into another, with stronger
    // modifications being first
    pen_chains.clear();
    for val in vals.iter() {
        if *val > OF(1.0) {
            pen_chains.positive.push(*val);
        } else if *val < OF(1.0) {
            pen_chains.negative.push(*val);
        }
    }
    if pen_chains.is_empty() {
        return None;
    }
    pen_chains.positive.sort_unstable_by_key(|v| -v);
    pen_chains.negative.sort_unstable();
    Some(get_chain_val(&mut pen_chains.positive) * get_chain_val(&mut pen_chains.negative))
}
fn get_chain_val(vals: &mut [AttrVal]) -> AttrVal {
    let mut val = OF(1.0);
    // Ignore 12th modification and further as non-significant
    for (i, mod_val) in vals.iter().enumerate() {
        if i >= PENALTY_SIGNIFICANT_MODIFICATIONS {
            break;
        }
        val *= OF(1.0) + (mod_val - OF(1.0)) * PENALTY_BASE.powi((i as i32).pow(2));
    }
    val
}

// Misc functions
fn get_min(vals: &[AttrVal]) -> Option<AttrVal> {
    vals.iter().min().copied()
}
fn get_max(vals: &[AttrVal]) -> Option<AttrVal> {
    vals.iter().max().copied()
}
