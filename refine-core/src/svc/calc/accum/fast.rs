//! This is attribute calculator designed to be used for attribute calculation.
//!
//! It has more bloated brother, which is built to calculate attribute value, and provide info about
//! what went into it. Since they duplicate each other, when doing any changes, MAKE SURE TO APPLY
//! THEM TO BOTH.

use std::{cmp::Ordering, collections::hash_map::Entry};

use bitflags::bitflags;

use super::shared::{
    AddMath, AddMathAdd, AddMathSub, MultMath, MultMathDiv, MultMathMul, MultMathPerc, PENALTY_MULTS, is_penal,
};
use crate::{
    PValue, UnitInterval, Value,
    ad::AItemCatId,
    svc::calc::{AggrKey, AggrMode, CalcOp},
    util::RMap,
};

pub(in crate::svc::calc) struct ModAccumFast {
    touched: SubAccums,
    pre_assign: AccumAssign,
    pre_mul: AccumMul,
    pre_div: AccumDiv,
    add: AccumAdd,
    sub: AccumSub,
    post_mul: AccumMul,
    post_div: AccumDiv,
    post_perc: AccumPerc,
    post_assign: AccumAssign,
    extra_add: AccumAdd,
    extra_mul: AccumMul,
}
impl ModAccumFast {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
            touched: SubAccums::empty(),
            pre_assign: AccumAssign::new(),
            pre_mul: AccumMul::new(),
            pre_div: AccumDiv::new(),
            add: AccumAdd::new(),
            sub: AccumSub::new(),
            post_mul: AccumMul::new(),
            post_div: AccumDiv::new(),
            post_perc: AccumPerc::new(),
            post_assign: AccumAssign::new(),
            extra_add: AccumAdd::new(),
            extra_mul: AccumMul::new(),
        }
    }
    pub(in crate::svc::calc) fn add_val(
        &mut self,
        val: Value,
        op: CalcOp,
        proj_mult: Option<PValue>,
        res_mult: Option<UnitInterval>,
        attr_pen: bool,
        item_cat: AItemCatId,
        aggr_mode: AggrMode,
        attr_hig: bool,
    ) {
        match op {
            CalcOp::PreAssign => {
                self.touched.insert(SubAccums::PRE_ASSIGN);
                self.pre_assign.add_val(val, proj_mult, res_mult, aggr_mode, attr_hig)
            }
            CalcOp::PreMul => {
                self.touched.insert(SubAccums::PRE_MUL);
                self.pre_mul
                    .add_raw_val(val, proj_mult, res_mult, aggr_mode, is_penal(attr_pen, &item_cat))
            }
            CalcOp::PreDiv => {
                self.touched.insert(SubAccums::PRE_DIV);
                self.pre_div
                    .add_raw_val(val, proj_mult, res_mult, aggr_mode, is_penal(attr_pen, &item_cat))
            }
            CalcOp::Add => {
                self.touched.insert(SubAccums::ADD);
                self.add.add_raw_val(val, proj_mult, res_mult, aggr_mode)
            }
            CalcOp::Sub => {
                self.touched.insert(SubAccums::SUB);
                self.sub.add_raw_val(val, proj_mult, res_mult, aggr_mode)
            }
            CalcOp::PostMul => {
                self.touched.insert(SubAccums::POST_MUL);
                self.post_mul
                    .add_raw_val(val, proj_mult, res_mult, aggr_mode, is_penal(attr_pen, &item_cat))
            }
            CalcOp::PostMulImmune => {
                self.touched.insert(SubAccums::POST_MUL);
                self.post_mul.add_raw_val(val, proj_mult, res_mult, aggr_mode, false)
            }
            CalcOp::PostDiv => {
                self.touched.insert(SubAccums::POST_DIV);
                self.post_div
                    .add_raw_val(val, proj_mult, res_mult, aggr_mode, is_penal(attr_pen, &item_cat))
            }
            CalcOp::PostPerc => {
                self.touched.insert(SubAccums::POST_PERC);
                self.post_perc
                    .add_raw_val(val, proj_mult, res_mult, aggr_mode, is_penal(attr_pen, &item_cat))
            }
            CalcOp::PostPercImmune => {
                self.touched.insert(SubAccums::POST_PERC);
                self.post_perc.add_raw_val(val, proj_mult, res_mult, aggr_mode, false)
            }
            CalcOp::PostAssign => {
                self.touched.insert(SubAccums::POST_ASSIGN);
                self.post_assign.add_val(val, proj_mult, res_mult, aggr_mode, attr_hig)
            }
            CalcOp::ExtraAdd => {
                self.touched.insert(SubAccums::EXTRA_ADD);
                self.extra_add.add_raw_val(val, proj_mult, res_mult, aggr_mode)
            }
            CalcOp::ExtraMul => {
                self.touched.insert(SubAccums::EXTRA_MUL);
                self.extra_mul
                    .add_raw_val(val, proj_mult, res_mult, aggr_mode, is_penal(attr_pen, &item_cat))
            }
        };
    }
    pub(in crate::svc::calc) fn apply_dogma_mods(&mut self, base_val: Value, attr_hig: bool) -> Value {
        let mut val = base_val;
        if self.touched.contains(SubAccums::PRE_ASSIGN) {
            val = self.pre_assign.modify_val(val, attr_hig);
        }
        if self.touched.contains(SubAccums::PRE_MUL) {
            val = self.pre_mul.modify_val(val);
        }
        if self.touched.contains(SubAccums::PRE_DIV) {
            val = self.pre_div.modify_val(val);
        }
        if self.touched.contains(SubAccums::ADD) {
            val = self.add.modify_val(val);
        }
        if self.touched.contains(SubAccums::SUB) {
            val = self.sub.modify_val(val);
        }
        if self.touched.contains(SubAccums::POST_MUL) {
            val = self.post_mul.modify_val(val);
        }
        if self.touched.contains(SubAccums::POST_DIV) {
            val = self.post_div.modify_val(val);
        }
        if self.touched.contains(SubAccums::POST_PERC) {
            val = self.post_perc.modify_val(val);
        }
        if self.touched.contains(SubAccums::POST_ASSIGN) {
            val = self.post_assign.modify_val(val, attr_hig);
        }
        val
    }
    pub(in crate::svc::calc) fn apply_extra_mods(&mut self, mut val: Value) -> Value {
        if self.touched.contains(SubAccums::EXTRA_ADD) {
            val = self.extra_add.modify_val(val);
        }
        if self.touched.contains(SubAccums::EXTRA_MUL) {
            val = self.extra_mul.modify_val(val);
        }
        val
    }
    pub(in crate::svc::calc) fn reset(&mut self) {
        if self.touched.is_empty() {
            return;
        }
        if self.touched.contains(SubAccums::PRE_ASSIGN) {
            self.pre_assign.reset();
        }
        if self.touched.contains(SubAccums::PRE_MUL) {
            self.pre_mul.reset();
        }
        if self.touched.contains(SubAccums::PRE_DIV) {
            self.pre_div.reset();
        }
        if self.touched.contains(SubAccums::ADD) {
            self.add.reset();
        }
        if self.touched.contains(SubAccums::SUB) {
            self.sub.reset();
        }
        if self.touched.contains(SubAccums::POST_MUL) {
            self.post_mul.reset();
        }
        if self.touched.contains(SubAccums::POST_DIV) {
            self.post_div.reset();
        }
        if self.touched.contains(SubAccums::POST_PERC) {
            self.post_perc.reset();
        }
        if self.touched.contains(SubAccums::POST_ASSIGN) {
            self.post_assign.reset();
        }
        if self.touched.contains(SubAccums::EXTRA_ADD) {
            self.extra_add.reset();
        }
        if self.touched.contains(SubAccums::EXTRA_MUL) {
            self.extra_mul.reset();
        }
        self.touched = SubAccums::empty();
    }
}

bitflags! {
    struct SubAccums: u16 {
        const PRE_ASSIGN = 1 << 0;
        const PRE_MUL = 1 << 1;
        const PRE_DIV = 1 << 2;
        const ADD = 1 << 3;
        const SUB = 1 << 4;
        const POST_MUL = 1 << 5;
        const POST_DIV = 1 << 6;
        const POST_PERC = 1 << 7;
        const POST_ASSIGN = 1 << 8;
        const EXTRA_ADD = 1 << 9;
        const EXTRA_MUL = 1 << 10;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Assignment
////////////////////////////////////////////////////////////////////////////////////////////////////
struct AccumAssign {
    /// Best seen non-aggregable assignment
    main: Option<Value>,
    // Aggregable assignments
    aggr_min: AggrMin,
    aggr_max: AggrMax,
}
impl AccumAssign {
    fn new() -> Self {
        Self {
            main: None,
            aggr_min: AggrMin::new(),
            aggr_max: AggrMax::new(),
        }
    }
    fn reset(&mut self) {
        self.main = None;
        self.aggr_min.reset();
        self.aggr_max.reset();
    }
    fn add_val(
        &mut self,
        val: Value,
        proj_mult: Option<PValue>,
        res_mult: Option<UnitInterval>,
        aggr_mode: AggrMode,
        attr_hig: bool,
    ) {
        // Projection/resist multipliers affect assign operations differently: if any of multipliers
        // is zero, then modification is not applied altogether, otherwise it is applied fully.
        // There are no such modifiers in EVE, but the lib makes it to work this way.
        if proj_mult == Some(PValue::ZERO) || res_mult == Some(UnitInterval::ZERO) {
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
    fn modify_val(&mut self, base: Value, attr_hig: bool) -> Value {
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
                return best_value;
            }
        }
        base
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Additive accumulator
////////////////////////////////////////////////////////////////////////////////////////////////////
type AccumAdd = AddAccum<AddMathAdd>;
type AccumSub = AddAccum<AddMathSub>;

struct AddAccum<M>
where
    M: AddMath,
{
    // Folded sum of raw values
    main: Value,
    // Aggregable, raw
    aggr_min: AggrMin,
    aggr_max: AggrMax,
    math: std::marker::PhantomData<M>,
}
impl<M> AddAccum<M>
where
    M: AddMath,
{
    fn new() -> Self {
        Self {
            main: Value::ZERO,
            aggr_min: AggrMin::new(),
            aggr_max: AggrMax::new(),
            math: std::marker::PhantomData,
        }
    }
    fn reset(&mut self) {
        self.main = Value::ZERO;
        self.aggr_min.reset();
        self.aggr_max.reset();
    }
    fn add_raw_val(
        &mut self,
        raw: Value,
        proj_mult: Option<PValue>,
        res_mult: Option<UnitInterval>,
        aggr_mode: AggrMode,
    ) {
        let raw = M::diminish_raw(raw, proj_mult, res_mult);
        match aggr_mode {
            AggrMode::Stack => self.main += raw,
            // Store addition in its original format for aggregation
            AggrMode::Min(key) => self.aggr_min.add_val(key, raw),
            AggrMode::Max(key) => self.aggr_max.add_val(key, raw),
        }
    }
    fn modify_val(&mut self, base: Value) -> Value {
        if !self.aggr_min.is_empty() {
            self.main += self.aggr_min.drain_values().sum::<Value>();
        }
        if !self.aggr_max.is_empty() {
            self.main += self.aggr_max.drain_values().sum::<Value>();
        }
        M::apply_raw(base, self.main)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Multiplicative accumulator
////////////////////////////////////////////////////////////////////////////////////////////////////
type AccumMul = MultAccum<MultMathMul>;
type AccumDiv = MultAccum<MultMathDiv>;
type AccumPerc = MultAccum<MultMathPerc>;

struct MultAccum<M>
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
impl<M> MultAccum<M>
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
    fn reset(&mut self) {
        self.main = Value::ONE;
        self.pens.reset();
        self.aggr_min.reset();
        self.aggr_max.reset();
    }
    fn add_raw_val(
        &mut self,
        raw: Value,
        proj_mult: Option<PValue>,
        res_mult: Option<UnitInterval>,
        aggr_mode: AggrMode,
        pen: bool,
    ) {
        if !M::check_raw(raw) {
            return;
        }
        let raw = M::diminish_raw(raw, proj_mult, res_mult);
        match aggr_mode {
            AggrMode::Stack => Self::add_raw_stacking(&mut self.main, &mut self.pens, raw, pen),
            AggrMode::Min(key) => self.aggr_min.add_val(key, PenEntry { raw, pen }),
            AggrMode::Max(key) => self.aggr_max.add_val(key, PenEntry { raw, pen }),
        }
    }
    fn modify_val(&mut self, base: Value) -> Value {
        // Distribute aggregable values first
        if !self.aggr_min.is_empty() {
            for aggr_entry in self.aggr_min.drain_values() {
                Self::add_raw_stacking(&mut self.main, &mut self.pens, aggr_entry.raw, aggr_entry.pen);
            }
        }
        if !self.aggr_max.is_empty() {
            for aggr_entry in self.aggr_max.drain_values() {
                Self::add_raw_stacking(&mut self.main, &mut self.pens, aggr_entry.raw, aggr_entry.pen);
            }
        }
        self.main *= self.pens.get_mult();
        base * self.main
    }
    fn add_raw_stacking(main: &mut Value, pens: &mut Pens<M>, raw: Value, pen: bool) {
        match pen {
            true => pens.add_raw(raw),
            // Store in main multiplier in case of stacked & unpenalized modification
            false => *main = M::apply_raw(*main, raw),
        }
    }
}

struct Pens<M>
where
    M: MultMath,
{
    neg: PenVals,
    pos: PenVals,
    math: std::marker::PhantomData<M>,
}
impl<M> Pens<M>
where
    M: MultMath,
{
    fn new() -> Self {
        Self {
            neg: PenVals::new(),
            pos: PenVals::new(),
            math: std::marker::PhantomData,
        }
    }
    fn reset(&mut self) {
        self.neg.len = 0;
        self.pos.len = 0;
    }
    fn add_raw(&mut self, raw: Value) {
        let mult_change = M::raw_to_mult_change(raw);
        match mult_change.cmp(&Value::ZERO) {
            Ordering::Less => self.neg.add(mult_change, neg_key),
            Ordering::Equal => (),
            Ordering::Greater => self.pos.add(mult_change, pos_key),
        }
    }
    fn get_mult(&mut self) -> Value {
        let mut mult = Value::ONE;
        if !self.neg.is_empty() {
            mult *= get_penalty_chain_mult(self.neg.sorted(neg_key));
        }
        if !self.pos.is_empty() {
            mult *= get_penalty_chain_mult(self.pos.sorted(pos_key));
        }
        mult
    }
}

// Custom container for penalties, to avoid allocations altogether
struct PenVals {
    vals: [Value; PENALTY_MULTS.len()],
    len: usize,
}
impl PenVals {
    fn new() -> Self {
        Self {
            vals: [Value::ZERO; PENALTY_MULTS.len()],
            len: 0,
        }
    }
    fn is_empty(&self) -> bool {
        self.len == 0
    }
    fn add(&mut self, val: Value, key: fn(&Value) -> Value) {
        if self.len < self.vals.len() {
            self.vals[self.len] = val;
            self.len += 1;
            return;
        }
        // Long penalty chains are considered unlikely
        std::hint::cold_path();
        // Replace the weakest mult change, if new one is stronger
        let (weakest_idx, weakest_val) = self.vals.iter().enumerate().max_by_key(|(_, v)| key(v)).unwrap();
        if key(&val) < key(weakest_val) {
            self.vals[weakest_idx] = val;
        }
    }
    fn sorted(&mut self, key: fn(&Value) -> Value) -> &[Value] {
        let vals = &mut self.vals[..self.len];
        vals.sort_unstable_by_key(key);
        vals
    }
}

fn neg_key(val: &Value) -> Value {
    *val
}
fn pos_key(val: &Value) -> Value {
    -*val
}

/// Take a slice of mult changes, return mult
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
    fn reset(&mut self) {
        // Should be drained if values were used (which should be pretty much always), but reset for
        // safety anyway
        if !self.data.is_empty() {
            self.data.clear();
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
