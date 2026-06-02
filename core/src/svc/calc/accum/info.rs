//! Attribute calculator which is modified to provide info on modification instead of just value.
//!
//! Whenever regular calculator changes, those changes have to be carried over here, to keep actual
//! calculation process and modification info consistent.

use std::cmp::Ordering;

use smallvec::SmallVec;

use super::shared::{MultMath, MultMathMul, PENALTY_MULTS, is_penal};
use crate::{
    ad::AItemCatId,
    api::Op,
    num::{PValue, Value},
    svc::calc::{Affector, AggrKey, AggrMode, CalcOp, Modification},
    util::RMap,
};

pub(in crate::svc::calc) struct AttrValInfo {
    pub(in crate::svc::calc) value: Value,
    pub(in crate::svc::calc) effective_infos: Vec<Modification>,
    pub(in crate::svc::calc) filtered_infos: Vec<Modification>,
}
impl AttrValInfo {
    pub(in crate::svc::calc) fn new(value: Value) -> Self {
        Self {
            value,
            effective_infos: Vec::new(),
            filtered_infos: Vec::new(),
        }
    }
    fn from_effective_info(value: Value, info: Modification) -> Self {
        Self {
            value,
            effective_infos: vec![info],
            filtered_infos: Vec::new(),
        }
    }
    fn merge(&mut self, other: AttrValInfo) {
        self.effective_infos.extend(other.effective_infos.into_iter());
        self.filtered_infos.extend(other.filtered_infos.into_iter());
    }
    fn merge_ineffective(&mut self, other: AttrValInfo) {
        self.filtered_infos.extend(other.effective_infos.into_iter());
        self.filtered_infos.extend(other.filtered_infos.into_iter());
    }
    fn is_single_effective(&self) -> bool {
        self.effective_infos.len() <= 1
    }
}

pub(in crate::svc::calc) struct ModAccumInfo {
    pre_assign: ModAccumAssign,
    pre_mul: ModAccumMul,
    pre_div: AttrStack,
    add: ModAccumAdd,
    sub: ModAccumSub,
    post_mul: ModAccumMul,
    post_div: AttrStack,
    post_perc: AttrStack,
    post_assign: ModAccumAssign,
    extra_add: ModAccumAdd,
    extra_mul: ModAccumMul,
}
impl ModAccumInfo {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
            pre_assign: ModAccumAssign::new(),
            pre_mul: ModAccumMul::new(),
            pre_div: AttrStack::new(),
            add: ModAccumAdd::new(),
            sub: ModAccumSub::new(),
            post_mul: ModAccumMul::new(),
            post_div: AttrStack::new(),
            post_perc: AttrStack::new(),
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
        affectors: SmallVec<[Affector; 1]>,
    ) {
        match op {
            CalcOp::PreAssign => self
                .pre_assign
                .add_val(op, val, proj_mult, res_mult, aggr_mode, affectors),
            CalcOp::PreMul => self.pre_mul.add_val(
                op,
                val,
                proj_mult,
                res_mult,
                aggr_mode,
                is_penal(attr_pen, &item_cat),
                affectors,
            ),
            CalcOp::PreDiv => self.pre_div.add_val(
                op,
                val,
                proj_mult,
                res_mult,
                &normalize_div,
                &diminish_mul,
                &revert_div,
                is_penal(attr_pen, &item_cat),
                &aggr_mode,
                affectors,
            ),
            CalcOp::Add => self.add.add_val(op, val, proj_mult, res_mult, aggr_mode, affectors),
            CalcOp::Sub => self.sub.add_val(op, val, proj_mult, res_mult, aggr_mode, affectors),
            CalcOp::PostMul => self.post_mul.add_val(
                op,
                val,
                proj_mult,
                res_mult,
                aggr_mode,
                is_penal(attr_pen, &item_cat),
                affectors,
            ),
            CalcOp::PostMulImmune => self
                .post_mul
                .add_val(op, val, proj_mult, res_mult, aggr_mode, false, affectors),
            CalcOp::PostDiv => self.post_div.add_val(
                op,
                val,
                proj_mult,
                res_mult,
                &normalize_div,
                &diminish_mul,
                &revert_div,
                is_penal(attr_pen, &item_cat),
                &aggr_mode,
                affectors,
            ),
            CalcOp::PostPerc => self.post_perc.add_val(
                op,
                val,
                proj_mult,
                res_mult,
                &normalize_perc,
                &diminish_mul,
                &revert_perc,
                is_penal(attr_pen, &item_cat),
                &aggr_mode,
                affectors,
            ),
            CalcOp::PostPercImmune => self.post_perc.add_val(
                op,
                val,
                proj_mult,
                res_mult,
                &normalize_perc,
                &diminish_mul,
                &revert_perc,
                false,
                &aggr_mode,
                affectors,
            ),
            CalcOp::PostAssign => self
                .post_assign
                .add_val(op, val, proj_mult, res_mult, aggr_mode, affectors),
            CalcOp::ExtraAdd => self
                .extra_add
                .add_val(op, val, proj_mult, res_mult, aggr_mode, affectors),
            CalcOp::ExtraMul => self
                .extra_mul
                .add_val(op, val, proj_mult, res_mult, aggr_mode, false, affectors),
        };
    }
    pub(in crate::svc::calc) fn apply_dogma_mods(&mut self, attr_info: AttrValInfo, hig: bool) -> AttrValInfo {
        let attr_info = self.pre_assign.process_attr_info(attr_info, hig);
        let attr_info = self.pre_mul.process_attr_info(attr_info);
        let attr_info = apply_mul(
            attr_info,
            self.pre_div
                .get_comb_attr_info(&combine_muls, &combine_muls_pen, &revert_div, hig),
        );
        let attr_info = self.add.process_attr_info(attr_info);
        let attr_info = self.sub.process_attr_info(attr_info);
        let attr_info = self.post_mul.process_attr_info(attr_info);
        let attr_info = apply_mul(
            attr_info,
            self.post_div
                .get_comb_attr_info(&combine_muls, &combine_muls_pen, &revert_div, hig),
        );
        let attr_info = apply_mul(
            attr_info,
            self.post_perc
                .get_comb_attr_info(&combine_muls, &combine_muls_pen, &revert_perc, hig),
        );
        self.post_assign.process_attr_info(attr_info, hig)
    }
    pub(in crate::svc::calc) fn apply_extra_mods(&mut self, attr_info: AttrValInfo) -> AttrValInfo {
        let attr_info = self.extra_add.process_attr_info(attr_info);
        self.extra_mul.process_attr_info(attr_info)
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
    fn add_val<N, D, R>(
        &mut self,
        op: CalcOp,
        initial_val: Value,
        proj_mult: Option<PValue>,
        res_mult: Option<PValue>,
        normalize_func: &N,
        diminish_func: &D,
        revert_func: &R,
        penalizable: bool,
        aggr_mode: &AggrMode,
        affectors: SmallVec<[Affector; 1]>,
    ) where
        N: Fn(Value) -> Option<Value>,
        D: Fn(Value, Option<PValue>, Option<PValue>) -> Value,
        R: Fn(Value) -> Value,
    {
        let attr_aggr = match penalizable {
            true => &mut self.penalized,
            false => &mut self.stacked,
        };
        attr_aggr.add_val(
            op,
            initial_val,
            proj_mult,
            res_mult,
            normalize_func,
            diminish_func,
            revert_func,
            aggr_mode,
            affectors,
        )
    }
    fn get_comb_attr_info<C, P, R>(
        &mut self,
        comb_func: &C,
        pen_func: &P,
        revert_func: &R,
        hig: bool,
    ) -> Option<AttrValInfo>
    where
        C: Fn(&mut Vec<AttrValInfo>, &R, bool) -> Option<AttrValInfo>,
        P: Fn(&mut Vec<AttrValInfo>, &R, bool) -> Option<AttrValInfo>,
        R: Fn(Value) -> Value,
    {
        if let Some(attr_info) = self.penalized.get_comb_attr_info(pen_func, revert_func, hig) {
            self.stacked.add_attr_info(attr_info, &AggrMode::Stack);
        }
        self.stacked.get_comb_attr_info(comb_func, revert_func, hig)
    }
}

struct AttrAggr {
    stack: Vec<AttrValInfo>,
    aggr_min: RMap<AggrKey, Vec<AttrValInfo>>,
    aggr_max: RMap<AggrKey, Vec<AttrValInfo>>,
}
impl AttrAggr {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            aggr_min: RMap::new(),
            aggr_max: RMap::new(),
        }
    }
    fn add_val<N, D, R>(
        &mut self,
        op: CalcOp,
        initial_val: Value,
        proj_mult: Option<PValue>,
        res_mult: Option<PValue>,
        normalize_func: &N,
        diminish_func: &D,
        revert_func: &R,
        aggr_mode: &AggrMode,
        affectors: SmallVec<[Affector; 1]>,
    ) where
        N: Fn(Value) -> Option<Value>,
        D: Fn(Value, Option<PValue>, Option<PValue>) -> Value,
        R: Fn(Value) -> Value,
    {
        let Some(normalized_val) = normalize_func(initial_val) else {
            return;
        };
        let diminished_val = diminish_func(normalized_val, proj_mult, res_mult);
        let info = Modification {
            op: Op::from_calc_op(op),
            initial_str: initial_val,
            range_mult: proj_mult,
            resist_mult: res_mult,
            stacking_mult: None,
            applied_str: revert_func(diminished_val),
            affectors: affectors.into_vec(),
        };
        let attr_info = AttrValInfo::from_effective_info(diminished_val, info);
        self.add_attr_info(attr_info, aggr_mode);
    }
    fn add_attr_info(&mut self, attr_info: AttrValInfo, aggr_mode: &AggrMode) {
        match aggr_mode {
            AggrMode::Stack => self.stack.push(attr_info),
            AggrMode::Min(key) => self.aggr_min.entry(*key).or_default().push(attr_info),
            AggrMode::Max(key) => self.aggr_max.entry(*key).or_default().push(attr_info),
        }
    }
    fn get_comb_attr_info<C, R>(&mut self, comb_func: &C, revert_func: &R, high_is_good: bool) -> Option<AttrValInfo>
    where
        C: Fn(&mut Vec<AttrValInfo>, &R, bool) -> Option<AttrValInfo>,
        R: Fn(Value) -> Value,
    {
        // Resolve aggregations
        for attr_infos in self.aggr_min.values_mut() {
            if let Some(mut attr_info) = extract_min_old(attr_infos) {
                for other_attr_info in attr_infos.extract_if(.., |_| true) {
                    attr_info.merge_ineffective(other_attr_info)
                }
                self.stack.push(attr_info);
            }
        }
        for attr_infos in self.aggr_max.values_mut() {
            if let Some(mut attr_info) = extract_max_old(attr_infos) {
                for other_attr_info in attr_infos.extract_if(.., |_| true) {
                    attr_info.merge_ineffective(other_attr_info)
                }
                self.stack.push(attr_info);
            }
        }
        comb_func(&mut self.stack, revert_func, high_is_good)
    }
}

// Revert normalization functions
fn revert_div(val: Value) -> Value {
    Value::ONE / val
}
fn revert_perc(val: Value) -> Value {
    (val - Value::ONE) * Value::HUNDRED
}

// Application functions - they treat left side and right side differently
fn apply_mul(mut base_attr_info: AttrValInfo, other_attr_info: Option<AttrValInfo>) -> AttrValInfo {
    match other_attr_info {
        Some(mut other_attr_info) => match (base_attr_info.value, other_attr_info.value) {
            // Right side 0 means left side has no effect on the result
            (_, Value::ZERO) => {
                other_attr_info.merge_ineffective(base_attr_info);
                other_attr_info
            }
            // Left side 0 means right side has no effect on the result
            (Value::ZERO, _) => {
                base_attr_info.merge_ineffective(other_attr_info);
                base_attr_info
            }
            _ => {
                base_attr_info.value *= other_attr_info.value;
                base_attr_info.merge(other_attr_info);
                base_attr_info
            }
        },
        None => base_attr_info,
    }
}

// Combination functions - they treat all values equally
fn combine_muls<R>(attr_infos: &mut Vec<AttrValInfo>, _revert_func: &R, _high_is_good: bool) -> Option<AttrValInfo> {
    if attr_infos.is_empty() {
        return None;
    }
    let value = attr_infos.iter().map(|v| v.value).product();
    let mut attr_info = AttrValInfo::new(value);
    match value {
        // Value of 0 means that some multipliers were 0. Expose only those, and hide the rest,
        // those we hid have no effect on value anyway
        Value::ZERO => {
            for other_attr_info in attr_infos.extract_if(.., |_| true) {
                match other_attr_info.value {
                    Value::ZERO => attr_info.merge(other_attr_info),
                    _ => attr_info.merge_ineffective(other_attr_info),
                }
            }
        }
        _ => {
            for other_attr_info in attr_infos.extract_if(.., |_| true) {
                // Multiplication by 1 is not changing result. But, as an exception, we add all the
                // modifications from it, if 1 is a result of multiple effective modifications. This
                // can happen when stacking penalty chains are calculated and aggregated into value
                // of 1.0; we want to expose all modifications which led to it even if final result
                // is 1.0
                if other_attr_info.value == Value::ONE && other_attr_info.is_single_effective() {
                    attr_info.merge_ineffective(other_attr_info)
                } else {
                    attr_info.merge(other_attr_info);
                }
            }
        }
    }
    Some(attr_info)
}
fn combine_muls_pen<R>(attr_infos: &mut Vec<AttrValInfo>, revert_func: &R, _high_is_good: bool) -> Option<AttrValInfo>
where
    R: Fn(Value) -> Value,
{
    // Gather positive multipliers into one chain, negative into another, with stronger
    // modifications being first
    let mut positive = Vec::new();
    let mut negative = Vec::new();
    let mut neutral = Vec::new();
    for attr_info in attr_infos.extract_if(.., |_| true) {
        if attr_info.value > Value::ONE {
            positive.push(attr_info);
        } else if attr_info.value < Value::ONE {
            negative.push(attr_info);
        } else {
            neutral.push(attr_info)
        }
    }
    if positive.is_empty() && negative.is_empty() {
        return None;
    }
    positive.sort_unstable_by_key(|v| -v.value);
    negative.sort_unstable_by_key(|v| v.value);
    let mut attr_info = AttrValInfo::new(Value::ONE);
    // Do negative chain first, since it can result in final multiplier of 0
    let negative_attr_info = get_chain_attr_info(negative, revert_func);
    attr_info.value *= negative_attr_info.value;
    attr_info.merge(negative_attr_info);
    let positive_attr_info = get_chain_attr_info(positive, revert_func);
    // It doesn't matter what is in positive chain if our multiplier is 0 already
    if attr_info.value == Value::ZERO {
        attr_info.merge_ineffective(positive_attr_info);
    } else {
        attr_info.value *= positive_attr_info.value;
        attr_info.merge(positive_attr_info);
    }
    // Multiplication by 1 is not changing the result
    for other_attr_info in neutral.into_iter() {
        attr_info.merge_ineffective(other_attr_info);
    }
    Some(attr_info)
}

// Misc functions
fn extract_min_old(attr_infos: &mut Vec<AttrValInfo>) -> Option<AttrValInfo> {
    let index = attr_infos
        .iter()
        .enumerate()
        .min_by_key(|(_, v)| v.value)
        .map(|(index, _)| index);
    index.map(|index| attr_infos.remove(index))
}
fn extract_max_old(attr_infos: &mut Vec<AttrValInfo>) -> Option<AttrValInfo> {
    let index = attr_infos
        .iter()
        .enumerate()
        .max_by_key(|(_, v)| v.value)
        .map(|(index, _)| index);
    index.map(|index| attr_infos.remove(index))
}
fn get_chain_attr_info<R>(attr_infos: Vec<AttrValInfo>, revert_func: &R) -> AttrValInfo
where
    R: Fn(Value) -> Value,
{
    let mut attr_info = AttrValInfo::new(Value::ONE);
    // Special case for when first element of chain is a multiplier by 0, for the same reason as in
    // multiplication combination function. We know final chain multiplier is going to be 0, we know
    // other elements are not going to be multipliers by 0 after penalty is applied, so we just
    // expose multiplier by 0 as the only effective modification, and consider others ineffective
    let first_zero = match attr_infos.first() {
        Some(other_attr_info) => other_attr_info.value == Value::ZERO,
        None => false,
    };
    for (i, mut other_attr_info) in attr_infos.into_iter().enumerate() {
        match PENALTY_MULTS.get(i) {
            Some(&penalty_multiplier) => {
                let value_multiplier =
                    (other_attr_info.value - Value::ONE).mul_add(penalty_multiplier.into_value(), Value::ONE);
                for info in other_attr_info.effective_infos.iter_mut() {
                    info.stacking_mult = Some(penalty_multiplier);
                    info.applied_str = revert_func(value_multiplier);
                }
                if first_zero && i > 0 {
                    attr_info.merge_ineffective(other_attr_info);
                } else {
                    attr_info.value *= value_multiplier;
                    attr_info.merge(other_attr_info);
                }
            }
            // Modifications past those which have penalty multiplier are insignificant
            None => {
                for info in other_attr_info.effective_infos.iter_mut() {
                    info.stacking_mult = Some(PValue::ZERO);
                    info.applied_str = revert_func(Value::ONE);
                }
                attr_info.merge_ineffective(other_attr_info);
            }
        }
    }
    attr_info
}

// TODO: code moved from shared module, might need to clean up
// Normalization functions
fn normalize_div(val: Value) -> Option<Value> {
    if val == Value::ZERO {
        return None;
    }
    Some(Value::ONE / val)
}
fn normalize_perc(val: Value) -> Option<Value> {
    Some(Value::ONE + val / Value::HUNDRED)
}

// Apply diminishing factors (resistance- and projection-related reductions)
fn diminish_basic(mut val: Value, proj_mult: Option<PValue>, res_mult: Option<PValue>) -> Value {
    if let Some(proj_mult) = proj_mult {
        val *= proj_mult;
    }
    if let Some(res_mult) = res_mult {
        val *= res_mult;
    }
    val
}
fn diminish_mul(val: Value, proj_mult: Option<PValue>, res_mult: Option<PValue>) -> Value {
    if res_mult.is_none() && proj_mult.is_none() {
        return val;
    }
    diminish_basic(val - Value::ONE, res_mult, proj_mult) + Value::ONE
}

// TODO: new implementation ahead, remove things above & refactor new code when done
// TODO: check extract_if vs drain vs into_iter once refactoring is done
////////////////////////////////////////////////////////////////////////////////////////////////////
// Assignment
////////////////////////////////////////////////////////////////////////////////////////////////////
struct ModAccumAssign {
    stack: Vec<AttrValInfo>,
    // Aggregable assignments
    aggr_min: RMap<AggrKey, Vec<AttrValInfo>>,
    aggr_max: RMap<AggrKey, Vec<AttrValInfo>>,
}
impl ModAccumAssign {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            aggr_min: RMap::new(),
            aggr_max: RMap::new(),
        }
    }
    fn add_val(
        &mut self,
        op: CalcOp,
        val: Value,
        proj_mult: Option<PValue>,
        res_mult: Option<PValue>,
        aggr_mode: AggrMode,
        affectors: SmallVec<[Affector; 1]>,
    ) {
        // Projection/resist multipliers affect assign operations differently: if any of multipliers
        // is 0.0, then modification is not applied altogether, otherwise it is applied fully. There
        // are no such modifiers in EVE, but the lib makes it to work this way.
        let proj_mult = match proj_mult {
            Some(PValue::ZERO) => return,
            Some(_) => Some(PValue::ONE),
            None => None,
        };
        let res_mult = match res_mult {
            Some(PValue::ZERO) => return,
            Some(_) => Some(PValue::ONE),
            None => None,
        };
        let info = AttrValInfo::from_effective_info(
            val,
            Modification {
                op: Op::from_calc_op(op),
                initial_str: val,
                range_mult: proj_mult,
                resist_mult: res_mult,
                stacking_mult: None,
                applied_str: val,
                affectors: affectors.into_vec(),
            },
        );
        match aggr_mode {
            AggrMode::Stack => self.stack.push(info),
            // Store asignment in its original format for aggregation
            AggrMode::Min(key) => self.aggr_min.entry(key).or_default().push(info),
            AggrMode::Max(key) => self.aggr_max.entry(key).or_default().push(info),
        }
    }
    fn process_attr_info(&mut self, base_info: AttrValInfo, attr_hig: bool) -> AttrValInfo {
        self.resolve_aggrs();
        let comb_info = self.get_comb_info(attr_hig);
        Self::apply_comb_to_base(base_info, comb_info)
    }
    // Functions which are not part of public interface
    fn resolve_aggrs(&mut self) {
        for (_aggr_key, mut grouped_infos) in self.aggr_min.drain() {
            if let Some(mut min_info) = extract_min(&mut grouped_infos) {
                for other_info in grouped_infos.into_iter() {
                    min_info.merge_ineffective(other_info)
                }
                self.stack.push(min_info);
            }
        }
        for (_aggr_key, mut grouped_infos) in self.aggr_max.drain() {
            if let Some(mut max_info) = extract_max(&mut grouped_infos) {
                for other_info in grouped_infos.into_iter() {
                    max_info.merge_ineffective(other_info)
                }
                self.stack.push(max_info);
            }
        }
    }
    fn get_comb_info(&mut self, attr_hig: bool) -> Option<AttrValInfo> {
        let mut new_info = match attr_hig {
            true => extract_max(&mut self.stack),
            false => extract_min(&mut self.stack),
        }?;
        for other_info in self.stack.drain(..) {
            new_info.merge_ineffective(other_info)
        }
        Some(new_info)
    }
    fn apply_comb_to_base(base_info: AttrValInfo, comb_info: Option<AttrValInfo>) -> AttrValInfo {
        // If there are any assignments, they dismiss left side as ineffective
        if let Some(mut comb_info) = comb_info {
            comb_info.merge_ineffective(base_info);
            return comb_info;
        }
        base_info
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Addition/subtraction
////////////////////////////////////////////////////////////////////////////////////////////////////
struct ModAccumAdd {
    // Non-aggregable increases
    stack: Vec<AttrValInfo>,
    // Aggregable increases
    aggr_min: RMap<AggrKey, Vec<AttrValInfo>>,
    aggr_max: RMap<AggrKey, Vec<AttrValInfo>>,
}
impl ModAccumAdd {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            aggr_min: RMap::new(),
            aggr_max: RMap::new(),
        }
    }
    fn add_val(
        &mut self,
        op: CalcOp,
        val: Value,
        proj_mult: Option<PValue>,
        res_mult: Option<PValue>,
        aggr_mode: AggrMode,
        affectors: SmallVec<[Affector; 1]>,
    ) {
        let diminished_val = diminish(val, proj_mult, res_mult);
        let info = AttrValInfo::from_effective_info(
            diminished_val,
            Modification {
                op: Op::from_calc_op(op),
                initial_str: val,
                range_mult: proj_mult,
                resist_mult: res_mult,
                stacking_mult: None,
                applied_str: diminished_val,
                affectors: affectors.into_vec(),
            },
        );
        match aggr_mode {
            AggrMode::Stack => self.stack.push(info),
            // Store asignment in its original format for aggregation
            AggrMode::Min(key) => self.aggr_min.entry(key).or_default().push(info),
            AggrMode::Max(key) => self.aggr_max.entry(key).or_default().push(info),
        }
    }
    fn process_attr_info(&mut self, base_info: AttrValInfo) -> AttrValInfo {
        self.resolve_aggrs();
        let comb_info = self.get_comb_info();
        Self::apply_comb_to_base(base_info, comb_info)
    }
    // Functions which are not part of public interface
    fn resolve_aggrs(&mut self) {
        for (_aggr_key, mut grouped_infos) in self.aggr_min.drain() {
            if let Some(mut min_info) = extract_min(&mut grouped_infos) {
                for other_info in grouped_infos.into_iter() {
                    min_info.merge_ineffective(other_info)
                }
                self.stack.push(min_info);
            }
        }
        for (_aggr_key, mut grouped_infos) in self.aggr_max.drain() {
            if let Some(mut max_info) = extract_max(&mut grouped_infos) {
                for other_info in grouped_infos.into_iter() {
                    max_info.merge_ineffective(other_info)
                }
                self.stack.push(max_info);
            }
        }
    }
    fn get_comb_info(&mut self) -> Option<AttrValInfo> {
        if self.stack.is_empty() {
            return None;
        }
        let comb_value = self.stack.iter().map(|v| v.value).sum();
        let mut comb_info = AttrValInfo::new(comb_value);
        for other_info in self.stack.drain(..) {
            match other_info.value {
                // Adding 0 is not changing the result
                Value::ZERO => comb_info.merge_ineffective(other_info),
                _ => comb_info.merge(other_info),
            }
        }
        Some(comb_info)
    }
    fn apply_comb_to_base(mut base_info: AttrValInfo, comb_info: Option<AttrValInfo>) -> AttrValInfo {
        if let Some(comb_info) = comb_info {
            base_info.value += comb_info.value;
            base_info.merge(comb_info);
        }
        base_info
    }
}

struct ModAccumSub {
    // Non-aggregable decreases
    stack: Vec<AttrValInfo>,
    // Aggregable decreases
    aggr_min: RMap<AggrKey, Vec<AttrValInfo>>,
    aggr_max: RMap<AggrKey, Vec<AttrValInfo>>,
}
impl ModAccumSub {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            aggr_min: RMap::new(),
            aggr_max: RMap::new(),
        }
    }
    fn add_val(
        &mut self,
        op: CalcOp,
        val: Value,
        proj_mult: Option<PValue>,
        res_mult: Option<PValue>,
        aggr_mode: AggrMode,
        affectors: SmallVec<[Affector; 1]>,
    ) {
        let diminished_val = diminish(val, proj_mult, res_mult);
        let info = AttrValInfo::from_effective_info(
            diminished_val,
            Modification {
                op: Op::from_calc_op(op),
                initial_str: val,
                range_mult: proj_mult,
                resist_mult: res_mult,
                stacking_mult: None,
                applied_str: diminished_val,
                affectors: affectors.into_vec(),
            },
        );
        match aggr_mode {
            AggrMode::Stack => self.stack.push(info),
            // Store asignment in its original format for aggregation
            AggrMode::Min(key) => self.aggr_min.entry(key).or_default().push(info),
            AggrMode::Max(key) => self.aggr_max.entry(key).or_default().push(info),
        }
    }
    fn process_attr_info(&mut self, base_info: AttrValInfo) -> AttrValInfo {
        self.resolve_aggrs();
        let comb_info = self.get_comb_info();
        Self::apply_comb_to_base(base_info, comb_info)
    }
    // Functions which are not part of public interface
    fn resolve_aggrs(&mut self) {
        for (_aggr_key, mut grouped_infos) in self.aggr_min.drain() {
            if let Some(mut min_info) = extract_min(&mut grouped_infos) {
                for other_info in grouped_infos.into_iter() {
                    min_info.merge_ineffective(other_info)
                }
                self.stack.push(min_info);
            }
        }
        for (_aggr_key, mut grouped_infos) in self.aggr_max.drain() {
            if let Some(mut max_info) = extract_max(&mut grouped_infos) {
                for other_info in grouped_infos.into_iter() {
                    max_info.merge_ineffective(other_info)
                }
                self.stack.push(max_info);
            }
        }
    }
    fn get_comb_info(&mut self) -> Option<AttrValInfo> {
        if self.stack.is_empty() {
            return None;
        }
        let comb_value = self.stack.iter().map(|v| v.value).sum();
        let mut comb_info = AttrValInfo::new(comb_value);
        for other_info in self.stack.drain(..) {
            match other_info.value {
                // Subtracting 0 is not changing the result
                Value::ZERO => comb_info.merge_ineffective(other_info),
                _ => comb_info.merge(other_info),
            }
        }
        Some(comb_info)
    }
    fn apply_comb_to_base(mut base_info: AttrValInfo, comb_info: Option<AttrValInfo>) -> AttrValInfo {
        if let Some(comb_info) = comb_info {
            base_info.value -= comb_info.value;
            base_info.merge(comb_info);
        }
        base_info
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Multiplication/division
////////////////////////////////////////////////////////////////////////////////////////////////////
struct PenEntry {
    info: AttrValInfo,
    pen: bool,
}

struct ModAccumMul {
    // Non-aggregable multiplications
    stack: Vec<AttrValInfo>,
    // Penalizable multiplications
    pens: Pens<MultMathMul>,
    // Aggregable increases
    aggr_min: RMap<AggrKey, Vec<PenEntry>>,
    aggr_max: RMap<AggrKey, Vec<PenEntry>>,
}
impl ModAccumMul {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            pens: Pens::new(),
            aggr_min: RMap::new(),
            aggr_max: RMap::new(),
        }
    }
    fn add_val(
        &mut self,
        op: CalcOp,
        val: Value,
        proj_mult: Option<PValue>,
        res_mult: Option<PValue>,
        aggr_mode: AggrMode,
        pen: bool,
        affectors: SmallVec<[Affector; 1]>,
    ) {
        let diminished_val = MultMathMul::diminish_raw(val, proj_mult, res_mult);
        let info = AttrValInfo::from_effective_info(
            diminished_val,
            Modification {
                op: Op::from_calc_op(op),
                initial_str: val,
                range_mult: proj_mult,
                resist_mult: res_mult,
                stacking_mult: None,
                applied_str: diminished_val,
                affectors: affectors.into_vec(),
            },
        );
        match aggr_mode {
            AggrMode::Stack => Self::add_stacking_val(&mut self.stack, &mut self.pens, info, pen),
            AggrMode::Min(key) => self.aggr_min.entry(key).or_default().push(PenEntry { info, pen }),
            AggrMode::Max(key) => self.aggr_max.entry(key).or_default().push(PenEntry { info, pen }),
        }
    }
    fn process_attr_info(&mut self, base_info: AttrValInfo) -> AttrValInfo {
        self.resolve_aggrs();
        self.resolve_pens();
        let comb_info = self.get_comb_info();
        Self::apply_comb_to_base(base_info, comb_info)
    }
    // Functions which are not part of public interface
    fn add_stacking_val(stack: &mut Vec<AttrValInfo>, pens: &mut Pens<MultMathMul>, info: AttrValInfo, pen: bool) {
        match pen {
            true => pens.add_val(info),
            false => stack.push(info),
        }
    }
    fn resolve_aggrs(&mut self) {
        for (_aggr_key, mut grouped_entries) in self.aggr_min.drain() {
            if let Some(mut min_entry) = extract_pen_min(&mut grouped_entries) {
                for other_entry in grouped_entries.into_iter() {
                    min_entry.info.merge_ineffective(other_entry.info)
                }
                Self::add_stacking_val(&mut self.stack, &mut self.pens, min_entry.info, min_entry.pen);
            }
        }
        for (_aggr_key, mut grouped_entries) in self.aggr_max.drain() {
            if let Some(mut max_entry) = extract_pen_max(&mut grouped_entries) {
                for other_entry in grouped_entries.into_iter() {
                    max_entry.info.merge_ineffective(other_entry.info)
                }
                Self::add_stacking_val(&mut self.stack, &mut self.pens, max_entry.info, max_entry.pen);
            }
        }
    }
    fn resolve_pens(&mut self) {
        if let Some(pen_info) = self.pens.get_comb_info() {
            self.stack.push(pen_info);
        }
    }
    fn get_comb_info(&mut self) -> Option<AttrValInfo> {
        if self.stack.is_empty() {
            return None;
        }
        let value = self.stack.iter().map(|v| v.value).product();
        let mut comb_info = AttrValInfo::new(value);
        match value {
            // Value of 0 means that some multipliers were 0. Expose only those, and hide the rest,
            // those we hid have no effect on value anyway
            Value::ZERO => {
                for other_info in self.stack.drain(..) {
                    match other_info.value {
                        Value::ZERO => comb_info.merge(other_info),
                        _ => comb_info.merge_ineffective(other_info),
                    }
                }
            }
            _ => {
                for other_info in self.stack.drain(..) {
                    // Multiplication by 1 is not changing result. But, as an exception, we add all
                    // the modifications from it, if 1 is a result of multiple effective
                    // modifications. This can happen when stacking penalty chains are calculated
                    // and aggregated into value of 1.0; we want to expose all modifications which
                    // led to it even if final result is 1.0
                    match other_info.value == Value::ONE && other_info.is_single_effective() {
                        true => comb_info.merge_ineffective(other_info),
                        false => comb_info.merge(other_info),
                    }
                }
            }
        }
        Some(comb_info)
    }
    fn apply_comb_to_base(mut base_info: AttrValInfo, comb_info: Option<AttrValInfo>) -> AttrValInfo {
        match comb_info {
            Some(mut comb_info) => match (base_info.value, comb_info.value) {
                // Right side 0 means left side has no effect on the result
                (_, Value::ZERO) => {
                    comb_info.merge_ineffective(base_info);
                    comb_info
                }
                // Left side 0 means right side has no effect on the result
                (Value::ZERO, _) => {
                    base_info.merge_ineffective(comb_info);
                    base_info
                }
                _ => {
                    base_info.value *= comb_info.value;
                    base_info.merge(comb_info);
                    base_info
                }
            },
            None => base_info,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Penalizable values
////////////////////////////////////////////////////////////////////////////////////////////////////
struct Pens<M> {
    // Multiplier change in all the fields
    pos: Vec<AttrValInfo>,
    neut: Vec<AttrValInfo>,
    neg: Vec<AttrValInfo>,
    math: std::marker::PhantomData<M>,
}
impl<M> Pens<M> {
    fn new() -> Self {
        Self {
            pos: Vec::new(),
            neut: Vec::new(),
            neg: Vec::new(),
            math: std::marker::PhantomData,
        }
    }
    fn add_val(&mut self, mut added_info: AttrValInfo)
    where
        M: MultMath,
    {
        let mul_change_value = M::raw_to_mult_change(added_info.value);
        match mul_change_value.cmp(&Value::ZERO) {
            Ordering::Greater => {
                added_info.value = mul_change_value;
                self.pos.push(added_info)
            }
            // We won't be doing anything with neutral items, so don't need to change info for it
            Ordering::Equal => self.neut.push(added_info),
            Ordering::Less => {
                added_info.value = mul_change_value;
                self.neg.push(added_info)
            }
        }
    }
    fn get_comb_info(&mut self) -> Option<AttrValInfo>
    where
        M: MultMath,
    {
        if self.neg.is_empty() && self.neut.is_empty() && self.pos.is_empty() {
            return None;
        }
        let mut comb_info = AttrValInfo::new(Value::ONE);
        // Do negative chain first, since it can result in final multiplier of 0
        if !self.neg.is_empty() {
            self.neg.sort_unstable_by_key(|v| v.value);
            let neg_info = Self::get_penalty_chain_info(&mut self.neg);
            comb_info.value *= neg_info.value;
            comb_info.merge(neg_info);
        }
        if !self.pos.is_empty() {
            self.pos.sort_unstable_by_key(|v| -v.value);
            let pos_info = Self::get_penalty_chain_info(&mut self.pos);
            match comb_info.value == Value::ZERO {
                // It doesn't matter what is in positive chain if our multiplier is 0 already
                true => comb_info.merge_ineffective(pos_info),
                false => {
                    comb_info.value *= pos_info.value;
                    comb_info.merge(pos_info);
                }
            }
        }
        // Neutral changes are always ineffective
        if !self.neut.is_empty() {
            for other_info in self.neut.drain(..) {
                comb_info.merge_ineffective(other_info);
            }
        }
        comb_info.value = M::mult_to_raw(comb_info.value);
        Some(comb_info)
    }
    // Take a slice of mult changes, return mult
    fn get_penalty_chain_info(chain_infos: &mut Vec<AttrValInfo>) -> AttrValInfo
    where
        M: MultMath,
    {
        let mut new_info = AttrValInfo::new(Value::ONE);
        // Special case for when first element of chain is a multiplier by 0, for the same reason as
        // in multiplication combination function. We know final chain multiplier is going to be 0,
        // we know other elements are not going to be multipliers by 0 after penalty is applied, so
        // we just expose multiplier by 0 as the only effective modification, and consider others
        // ineffective
        let first_zeroing = match chain_infos.first() {
            Some(chain_info) => chain_info.value == -Value::ONE,
            None => false,
        };
        for (i, mut other_info) in chain_infos.drain(..).enumerate() {
            match PENALTY_MULTS.get(i) {
                Some(&pen_mult) => {
                    let val_mult = other_info.value.mul_add(pen_mult.into_value(), Value::ONE);
                    for other_info_mod in other_info.effective_infos.iter_mut() {
                        other_info_mod.stacking_mult = Some(pen_mult);
                        other_info_mod.applied_str = M::mult_to_raw(val_mult);
                    }
                    match first_zeroing && i > 0 {
                        true => new_info.merge_ineffective(other_info),
                        false => {
                            new_info.value *= val_mult;
                            new_info.merge(other_info);
                        }
                    }
                }
                // Modifications past those which have penalty multiplier are insignificant
                None => {
                    for other_info_mod in other_info.effective_infos.iter_mut() {
                        other_info_mod.stacking_mult = Some(PValue::ZERO);
                        other_info_mod.applied_str = M::mult_to_raw(Value::ONE);
                    }
                    new_info.merge_ineffective(other_info);
                }
            }
        }
        new_info
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helper functions
////////////////////////////////////////////////////////////////////////////////////////////////////
fn extract_min(attr_infos: &mut Vec<AttrValInfo>) -> Option<AttrValInfo> {
    let index = attr_infos.iter().enumerate().min_by_key(|(_, v)| v.value)?.0;
    Some(attr_infos.swap_remove(index))
}
fn extract_max(attr_infos: &mut Vec<AttrValInfo>) -> Option<AttrValInfo> {
    let index = attr_infos.iter().enumerate().max_by_key(|(_, v)| v.value)?.0;
    Some(attr_infos.swap_remove(index))
}

fn extract_pen_min(attr_infos: &mut Vec<PenEntry>) -> Option<PenEntry> {
    // Prefer non-penalizable entries if values match
    let index = attr_infos
        .iter()
        .enumerate()
        .min_by_key(|(_, v)| (v.info.value, v.pen))?
        .0;
    Some(attr_infos.swap_remove(index))
}
fn extract_pen_max(attr_infos: &mut Vec<PenEntry>) -> Option<PenEntry> {
    // Prefer non-penalizable entries if values match
    let index = attr_infos
        .iter()
        .enumerate()
        .max_by_key(|(_, v)| (v.info.value, !v.pen))?
        .0;
    Some(attr_infos.swap_remove(index))
}

// TODO: consider movind to add/sub converters in shared code
fn diminish(mut val: Value, proj_mult: Option<PValue>, res_mult: Option<PValue>) -> Value {
    // Follow the same order of operations as in fast accum: multiply multipliers first, then apply
    if let Some(comb_mult) = proj_mult.reduce(res_mult, |x, y| x * y) {
        val *= comb_mult;
    }
    val
}
