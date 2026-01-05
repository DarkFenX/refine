//! Attribute calculator which is modified to provide info on modification instead of just value.
//!
//! Whenever regular calculator changes, those changes have to be carried over here, to keep actual
//! calculation process and modification info consistent.

use smallvec::SmallVec;

use super::shared::{
    PENALTY_DENOMINATORS, diminish_basic, diminish_mul, is_penal, normalize_div, normalize_noop, normalize_perc,
    normalize_sub, preprocess_assign_diminish_mult,
};
use crate::{
    ad::AItemCatId,
    misc::{PValue, Value},
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
    fn merge(&mut self, mut other: AttrValInfo) {
        self.effective_infos
            .extend(other.effective_infos.extract_if(.., |_| true));
        self.filtered_infos
            .extend(other.filtered_infos.extract_if(.., |_| true));
    }
    fn merge_ineffective(&mut self, mut other: AttrValInfo) {
        self.filtered_infos
            .extend(other.effective_infos.extract_if(.., |_| true));
        self.filtered_infos
            .extend(other.filtered_infos.extract_if(.., |_| true));
    }
    fn is_single_effective(&self) -> bool {
        self.effective_infos.len() <= 1
    }
}

pub(in crate::svc::calc) struct ModAccumInfo {
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
}
impl ModAccumInfo {
    pub(in crate::svc::calc) fn new() -> Self {
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
        }
    }
    pub(in crate::svc::calc) fn add_val(
        &mut self,
        val: Value,
        proj_mult: Option<PValue>,
        res_mult: Option<PValue>,
        op: &CalcOp,
        attr_pen: bool,
        item_cat: &AItemCatId,
        aggr_mode: &AggrMode,
        affectors: SmallVec<Affector, 1>,
    ) {
        match op {
            CalcOp::PreAssign => {
                if let Some(proj_mult) = preprocess_assign_diminish_mult(proj_mult)
                    && let Some(res_mult) = preprocess_assign_diminish_mult(res_mult)
                {
                    self.pre_assign.add_val(
                        CalcOp::PreAssign,
                        val,
                        proj_mult,
                        res_mult,
                        &normalize_noop,
                        &diminish_basic,
                        &revert_noop,
                        aggr_mode,
                        affectors,
                    )
                }
            }
            CalcOp::PreMul => self.pre_mul.add_val(
                CalcOp::PreMul,
                val,
                proj_mult,
                res_mult,
                &normalize_noop,
                &diminish_mul,
                &revert_noop,
                is_penal(attr_pen, item_cat),
                aggr_mode,
                affectors,
            ),
            CalcOp::PreDiv => self.pre_div.add_val(
                CalcOp::PreDiv,
                val,
                proj_mult,
                res_mult,
                &normalize_div,
                &diminish_mul,
                &revert_div,
                is_penal(attr_pen, item_cat),
                aggr_mode,
                affectors,
            ),
            CalcOp::Add => self.add.add_val(
                CalcOp::Add,
                val,
                proj_mult,
                res_mult,
                &normalize_noop,
                &diminish_basic,
                &revert_noop,
                aggr_mode,
                affectors,
            ),
            CalcOp::Sub => self.sub.add_val(
                CalcOp::Sub,
                val,
                proj_mult,
                res_mult,
                &normalize_sub,
                &diminish_basic,
                &revert_sub,
                aggr_mode,
                affectors,
            ),
            CalcOp::PostMul => self.post_mul.add_val(
                CalcOp::PostMul,
                val,
                proj_mult,
                res_mult,
                &normalize_noop,
                &diminish_mul,
                &revert_noop,
                is_penal(attr_pen, item_cat),
                aggr_mode,
                affectors,
            ),
            CalcOp::PostMulImmune => self.post_mul.add_val(
                CalcOp::PostMulImmune,
                val,
                proj_mult,
                res_mult,
                &normalize_noop,
                &diminish_mul,
                &revert_noop,
                false,
                aggr_mode,
                affectors,
            ),
            CalcOp::PostDiv => self.post_div.add_val(
                CalcOp::PostDiv,
                val,
                proj_mult,
                res_mult,
                &normalize_div,
                &diminish_mul,
                &revert_div,
                is_penal(attr_pen, item_cat),
                aggr_mode,
                affectors,
            ),
            CalcOp::PostPerc => self.post_perc.add_val(
                CalcOp::PostPerc,
                val,
                proj_mult,
                res_mult,
                &normalize_perc,
                &diminish_mul,
                &revert_perc,
                is_penal(attr_pen, item_cat),
                aggr_mode,
                affectors,
            ),
            CalcOp::PostPercImmune => self.post_perc.add_val(
                CalcOp::PostPerc,
                val,
                proj_mult,
                res_mult,
                &normalize_perc,
                &diminish_mul,
                &revert_perc,
                false,
                aggr_mode,
                affectors,
            ),
            CalcOp::PostAssign => {
                if let Some(proj_mult) = preprocess_assign_diminish_mult(proj_mult)
                    && let Some(res_mult) = preprocess_assign_diminish_mult(res_mult)
                {
                    self.post_assign.add_val(
                        CalcOp::PostAssign,
                        val,
                        proj_mult,
                        res_mult,
                        &normalize_noop,
                        &diminish_basic,
                        &revert_noop,
                        aggr_mode,
                        affectors,
                    )
                }
            }
            CalcOp::ExtraAdd => self.extra_add.add_val(
                CalcOp::ExtraAdd,
                val,
                proj_mult,
                res_mult,
                &normalize_noop,
                &diminish_basic,
                &revert_noop,
                aggr_mode,
                affectors,
            ),
            CalcOp::ExtraMul => self.extra_mul.add_val(
                CalcOp::ExtraMul,
                val,
                proj_mult,
                res_mult,
                &normalize_noop,
                &diminish_mul,
                &revert_noop,
                aggr_mode,
                affectors,
            ),
        };
    }
    pub(in crate::svc::calc) fn apply_dogma_mods(&mut self, attr_info: AttrValInfo, hig: bool) -> AttrValInfo {
        let attr_info = apply_assign(
            attr_info,
            self.pre_assign.get_comb_attr_info(&combine_assigns, &revert_noop, hig),
        );
        let attr_info = apply_mul(
            attr_info,
            self.pre_mul
                .get_comb_attr_info(&combine_muls, &combine_muls_pen, &revert_noop, hig),
        );
        let attr_info = apply_mul(
            attr_info,
            self.pre_div
                .get_comb_attr_info(&combine_muls, &combine_muls_pen, &revert_div, hig),
        );
        let attr_info = apply_add(attr_info, self.add.get_comb_attr_info(&combine_adds, &revert_noop, hig));
        let attr_info = apply_add(attr_info, self.sub.get_comb_attr_info(&combine_adds, &revert_sub, hig));
        let attr_info = apply_mul(
            attr_info,
            self.post_mul
                .get_comb_attr_info(&combine_muls, &combine_muls_pen, &revert_noop, hig),
        );
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
        apply_assign(
            attr_info,
            self.post_assign.get_comb_attr_info(&combine_assigns, &revert_noop, hig),
        )
    }
    pub(in crate::svc::calc) fn apply_extra_mods(&mut self, attr_info: AttrValInfo, hig: bool) -> AttrValInfo {
        let attr_info = apply_add(
            attr_info,
            self.extra_add.get_comb_attr_info(&combine_adds, &revert_noop, hig),
        );
        apply_mul(
            attr_info,
            self.extra_mul.get_comb_attr_info(&combine_muls, &revert_noop, hig),
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
        affectors: SmallVec<Affector, 1>,
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
        affectors: SmallVec<Affector, 1>,
    ) where
        N: Fn(Value) -> Option<Value>,
        D: Fn(Value, Option<PValue>, Option<PValue>) -> Value,
        R: Fn(Value) -> Value,
    {
        let normalized_val = match normalize_func(initial_val) {
            Some(val) => val,
            None => return,
        };
        let diminished_val = diminish_func(normalized_val, proj_mult, res_mult);
        let info = Modification {
            op: op.into(),
            initial_val,
            range_mult: proj_mult,
            resist_mult: res_mult,
            stacking_mult: None,
            applied_val: revert_func(diminished_val),
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
            if let Some(mut attr_info) = extract_min(attr_infos) {
                for other_attr_info in attr_infos.extract_if(.., |_| true) {
                    attr_info.merge_ineffective(other_attr_info)
                }
                self.stack.push(attr_info);
            }
        }
        for attr_infos in self.aggr_max.values_mut() {
            if let Some(mut attr_info) = extract_max(attr_infos) {
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
fn revert_noop(val: Value) -> Value {
    val
}
fn revert_sub(val: Value) -> Value {
    -val
}
fn revert_div(val: Value) -> Value {
    Value::ONE / val
}
fn revert_perc(val: Value) -> Value {
    (val - Value::ONE) * Value::HUNDRED
}

// Application functions - they treat left side and right side differently
fn apply_assign(base_attr_info: AttrValInfo, other_attr_info: Option<AttrValInfo>) -> AttrValInfo {
    match other_attr_info {
        // If there are any assignments, they dismiss left side as ineffective
        Some(mut other_attr_info) => {
            other_attr_info.merge_ineffective(base_attr_info);
            other_attr_info
        }
        None => base_attr_info,
    }
}
fn apply_add(mut base_attr_info: AttrValInfo, other_attr_info: Option<AttrValInfo>) -> AttrValInfo {
    if let Some(other_attr_info) = other_attr_info {
        base_attr_info.value += other_attr_info.value;
        base_attr_info.merge(other_attr_info);
    }
    base_attr_info
}
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
fn combine_assigns<R>(attr_infos: &mut Vec<AttrValInfo>, _revert_func: &R, high_is_good: bool) -> Option<AttrValInfo> {
    let effective = match high_is_good {
        true => extract_max(attr_infos),
        false => extract_min(attr_infos),
    };
    match effective {
        // Only one assign is considered effective, the rest are not
        Some(mut attr_info) => {
            for other_attr_info in attr_infos.extract_if(.., |_| true) {
                attr_info.merge_ineffective(other_attr_info)
            }
            Some(attr_info)
        }
        None => None,
    }
}
fn combine_adds<R>(attr_infos: &mut Vec<AttrValInfo>, _revert_func: &R, _high_is_good: bool) -> Option<AttrValInfo> {
    if attr_infos.is_empty() {
        return None;
    }
    let value = attr_infos.iter().map(|v| v.value).sum();
    let mut attr_info = AttrValInfo::new(value);
    for other_attr_info in attr_infos.extract_if(.., |_| true) {
        match other_attr_info.value {
            // Adding 0 is not changing the result
            Value::ZERO => attr_info.merge_ineffective(other_attr_info),
            _ => attr_info.merge(other_attr_info),
        }
    }
    Some(attr_info)
}
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
fn extract_min(attr_infos: &mut Vec<AttrValInfo>) -> Option<AttrValInfo> {
    let index = attr_infos
        .iter()
        .enumerate()
        .min_by_key(|(_, v)| v.value)
        .map(|(index, _)| index);
    index.map(|index| attr_infos.remove(index))
}
fn extract_max(attr_infos: &mut Vec<AttrValInfo>) -> Option<AttrValInfo> {
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
        match PENALTY_DENOMINATORS.get(i) {
            Some(denominator) => {
                let penalty_multiplier = Value::ONE / denominator;
                let value_multiplier = Value::ONE + (other_attr_info.value - Value::ONE) * penalty_multiplier;
                for info in other_attr_info.effective_infos.iter_mut() {
                    info.stacking_mult = Some(penalty_multiplier);
                    info.applied_val = revert_func(value_multiplier);
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
                    info.stacking_mult = Some(Value::ZERO);
                    info.applied_val = revert_func(Value::ONE);
                }
                attr_info.merge_ineffective(other_attr_info);
            }
        }
    }
    attr_info
}
