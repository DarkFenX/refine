//! This is attribute calculator designed to be used for attribute calculation.
//!
//! It has more bloated brother, which is built to calculate attribute value, and provide info about
//! what went into it. Since they duplicate each other, when doing any changes, MAKE SURE TO APPLY
//! THEM TO BOTH.

use std::{cmp::Ordering, collections::hash_map::Entry};

use super::shared::{MultMath, MultMathDiv, MultMathMul, MultMathPerc, PENALTY_MULTS, is_penal};
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
            CalcOp::PreMul => {
                self.pre_mul
                    .add_raw_val(val, proj_mult, res_mult, aggr_mode, is_penal(attr_pen, &item_cat))
            }
            CalcOp::PreDiv => {
                self.pre_div
                    .add_raw_val(val, proj_mult, res_mult, aggr_mode, is_penal(attr_pen, &item_cat))
            }
            CalcOp::Add => self.add.add_val(val, comb_mult, aggr_mode),
            CalcOp::Sub => self.sub.add_val(val, comb_mult, aggr_mode),
            CalcOp::PostMul => {
                self.post_mul
                    .add_raw_val(val, proj_mult, res_mult, aggr_mode, is_penal(attr_pen, &item_cat))
            }
            CalcOp::PostMulImmune => self.post_mul.add_raw_val(val, proj_mult, res_mult, aggr_mode, false),
            CalcOp::PostDiv => {
                self.post_div
                    .add_raw_val(val, proj_mult, res_mult, aggr_mode, is_penal(attr_pen, &item_cat))
            }
            CalcOp::PostPerc => {
                self.post_perc
                    .add_raw_val(val, proj_mult, res_mult, aggr_mode, is_penal(attr_pen, &item_cat))
            }
            CalcOp::PostPercImmune => self.post_perc.add_raw_val(val, proj_mult, res_mult, aggr_mode, false),
            CalcOp::PostAssign => self.post_assign.add_val(val, comb_mult, aggr_mode, attr_hig),
            CalcOp::ExtraAdd => self.extra_add.add_val(val, comb_mult, aggr_mode),
            CalcOp::ExtraMul => {
                self.extra_mul
                    .add_raw_val(val, proj_mult, res_mult, aggr_mode, is_penal(attr_pen, &item_cat))
            }
        };
    }
    pub(in crate::svc::calc) fn apply_dogma_mods(&mut self, base_val: Value, attr_hig: bool) -> Value {
        let val = self.pre_assign.calc_val(base_val, attr_hig);
        let val = self.pre_mul.modify_val(val);
        let val = self.pre_div.modify_val(val);
        let val = self.add.calc_val(val);
        let val = self.sub.calc_val(val);
        let val = self.post_mul.modify_val(val);
        let val = self.post_div.modify_val(val);
        let val = self.post_perc.modify_val(val);
        self.post_assign.calc_val(val, attr_hig)
    }
    pub(in crate::svc::calc) fn apply_extra_mods(&mut self, val: Value) -> Value {
        let val = self.extra_add.calc_val(val);
        self.extra_mul.modify_val(val)
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
        if self.main.is_some() || !self.aggr_min.is_empty() || !self.aggr_max.is_empty() {
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
        }
        val
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Addition/subtraction
////////////////////////////////////////////////////////////////////////////////////////////////////
struct ModAccumAdd {
    // Folded sum of increases
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
        if !self.aggr_min.is_empty() {
            self.main += self.aggr_min.drain_values().sum::<Value>();
        }
        if !self.aggr_max.is_empty() {
            self.main += self.aggr_max.drain_values().sum::<Value>();
        }
        val + self.main
    }
}

struct ModAccumSub {
    // Folded sum of decreases
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
        if !self.aggr_min.is_empty() {
            self.main += self.aggr_min.drain_values().sum::<Value>();
        }
        if !self.aggr_max.is_empty() {
            self.main += self.aggr_max.drain_values().sum::<Value>();
        }
        val - self.main
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Multiplicative accumulator
////////////////////////////////////////////////////////////////////////////////////////////////////
type ModAccumMul = ModAccumMult<MultMathMul>;
type ModAccumDiv = ModAccumMult<MultMathDiv>;
type ModAccumPerc = ModAccumMult<MultMathPerc>;

struct ModAccumMult<M>
where
    M: MultMath,
{
    // Folded multiplier
    main: Value,
    // Penalizable, internally stored in form of mult change
    pens: Pens<M>,
    // Aggregable, stored in raw form of value with penalization flag
    aggr_min: AggrPenMin,
    aggr_max: AggrPenMax,
}
impl<M> ModAccumMult<M>
where
    M: MultMath,
{
    fn new() -> Self {
        Self {
            main: Value::ONE,
            pens: Pens::new(),
            aggr_min: AggrPenMin::new(),
            aggr_max: AggrPenMax::new(),
        }
    }
    fn add_raw_val(
        &mut self,
        raw: Value,
        proj_mult: Option<PValue>,
        res_mult: Option<PValue>,
        aggr_mode: AggrMode,
        pen: bool,
    ) {
        if !M::check_raw(raw) {
            return;
        }
        let raw = M::diminish_raw(raw, proj_mult, res_mult);
        match aggr_mode {
            AggrMode::Stack => add_raw_stacking(&mut self.main, &mut self.pens, raw, pen),
            AggrMode::Min(key) => self.aggr_min.add_val(key, PenEntry { raw, pen }),
            AggrMode::Max(key) => self.aggr_max.add_val(key, PenEntry { raw, pen }),
        }
    }
    fn modify_val(&mut self, base: Value) -> Value {
        // Distribute aggregable values first
        if !self.aggr_min.is_empty() {
            for aggr_entry in self.aggr_min.drain_values() {
                add_raw_stacking(&mut self.main, &mut self.pens, aggr_entry.raw, aggr_entry.pen);
            }
        }
        if !self.aggr_max.is_empty() {
            for aggr_entry in self.aggr_max.drain_values() {
                add_raw_stacking(&mut self.main, &mut self.pens, aggr_entry.raw, aggr_entry.pen);
            }
        }
        self.main *= self.pens.get_mult();
        base * self.main
    }
}

fn add_raw_stacking<M>(main: &mut Value, pens: &mut Pens<M>, raw: Value, pen: bool)
where
    M: MultMath,
{
    match pen {
        true => pens.add_raw(raw),
        // Store in main multiplier in case of stacked & unpenalized modification
        false => M::apply_raw(main, raw),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Penalizable values
////////////////////////////////////////////////////////////////////////////////////////////////////
struct Pens<M>
where
    M: MultMath,
{
    neg: Vec<Value>,
    pos: Vec<Value>,
    conv: std::marker::PhantomData<M>,
}
impl<M> Pens<M>
where
    M: MultMath,
{
    fn new() -> Self {
        Self {
            neg: Vec::new(),
            pos: Vec::new(),
            conv: std::marker::PhantomData,
        }
    }
    fn add_raw(&mut self, raw: Value) {
        let mult_change = M::raw_to_mult_change(raw);
        match mult_change.cmp(&Value::ZERO) {
            Ordering::Less => self.neg.push(mult_change),
            Ordering::Equal => (),
            Ordering::Greater => self.pos.push(mult_change),
        }
    }
    fn get_mult(&mut self) -> Value {
        let mut mult = Value::ONE;
        if !self.neg.is_empty() {
            self.neg.sort_unstable();
            mult *= get_penalty_chain_mult(&self.neg);
        }
        if !self.pos.is_empty() {
            self.pos.sort_unstable_by_key(|&v| -v);
            mult *= get_penalty_chain_mult(&self.pos);
        }
        mult
    }
}

// Take a slice of mult changes, return mult
// TODO: consider draining vec instead of iterating
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
    raw: Value,
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
    fn is_empty(&self) -> bool {
        self.data.is_empty()
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
        match added.raw.cmp(&stored.raw) {
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
        match added.raw.cmp(&stored.raw) {
            Ordering::Greater => true,
            Ordering::Equal => !added.pen && stored.pen,
            Ordering::Less => false,
        }
    }
}
