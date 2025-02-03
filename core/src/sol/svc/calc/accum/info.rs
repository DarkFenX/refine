//! Attribute calculator which is modified to provide info on modification instead of just value.
//!
//! Whenever regular calculator changes, those changes have to be carried over here, to keep actual
//! calculation process and modification info consistent.

use smallvec::SmallVec;

use crate::{
    defs::{AggrKey, AttrVal, EItemCatId, OF},
    sol::svc::calc::{SolAggrMode, SolModificationInfo, SolOp},
    util::StMap,
    SolAffectorInfo,
};

use super::shared::{
    diminish_basic, diminish_mul, diminish_noop, is_penal, normalize_div, normalize_noop, normalize_perc,
    normalize_sub, PENALTY_BASE,
};

pub(in crate::sol::svc::calc) struct SolAttrValInfo {
    pub(in crate::sol::svc::calc) value: AttrVal,
    pub(in crate::sol::svc::calc) effective_infos: Vec<SolModificationInfo>,
    pub(in crate::sol::svc::calc) filtered_infos: Vec<SolModificationInfo>,
}
impl SolAttrValInfo {
    pub(in crate::sol::svc::calc) fn new(value: AttrVal) -> Self {
        Self {
            value,
            effective_infos: Vec::new(),
            filtered_infos: Vec::new(),
        }
    }
    fn from_effective_info(value: AttrVal, info: SolModificationInfo) -> Self {
        Self {
            value,
            effective_infos: vec![info],
            filtered_infos: Vec::new(),
        }
    }
    fn merge(&mut self, mut other: SolAttrValInfo) {
        self.effective_infos
            .extend(other.effective_infos.extract_if(.., |_| true));
        self.filtered_infos
            .extend(other.filtered_infos.extract_if(.., |_| true));
    }
    fn merge_ineffective(&mut self, mut other: SolAttrValInfo) {
        self.filtered_infos
            .extend(other.effective_infos.extract_if(.., |_| true));
        self.filtered_infos
            .extend(other.filtered_infos.extract_if(.., |_| true));
    }
    fn is_single_effective(&self) -> bool {
        self.effective_infos.len() <= 1
    }
}

pub(in crate::sol::svc::calc) struct SolModAccumInfo {
    pre_assign: SolAttrAggr,
    pre_mul: SolAttrStack,
    pre_div: SolAttrStack,
    add: SolAttrAggr,
    sub: SolAttrAggr,
    post_mul: SolAttrStack,
    post_div: SolAttrStack,
    post_perc: SolAttrStack,
    post_assign: SolAttrAggr,
    extra_mul: SolAttrAggr,
}
impl SolModAccumInfo {
    pub(in crate::sol::svc::calc) fn new() -> Self {
        Self {
            pre_assign: SolAttrAggr::new(),
            pre_mul: SolAttrStack::new(),
            pre_div: SolAttrStack::new(),
            add: SolAttrAggr::new(),
            sub: SolAttrAggr::new(),
            post_mul: SolAttrStack::new(),
            post_div: SolAttrStack::new(),
            post_perc: SolAttrStack::new(),
            post_assign: SolAttrAggr::new(),
            extra_mul: SolAttrAggr::new(),
        }
    }
    pub(in crate::sol::svc::calc) fn add_val(
        &mut self,
        val: AttrVal,
        proj_mult: Option<AttrVal>,
        res_mult: Option<AttrVal>,
        op: &SolOp,
        attr_pen: bool,
        item_cat: &EItemCatId,
        aggr_mode: &SolAggrMode,
        affectors: SmallVec<SolAffectorInfo, 1>,
    ) {
        match op {
            SolOp::PreAssign => self.pre_assign.add_val(
                SolOp::PreAssign,
                val,
                None,
                None,
                &normalize_noop,
                &diminish_noop,
                &revert_noop,
                aggr_mode,
                affectors,
            ),
            SolOp::PreMul => self.pre_mul.add_val(
                SolOp::PreMul,
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
            SolOp::PreDiv => self.pre_div.add_val(
                SolOp::PreDiv,
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
            SolOp::Add => self.add.add_val(
                SolOp::Add,
                val,
                proj_mult,
                res_mult,
                &normalize_noop,
                &diminish_basic,
                &revert_noop,
                aggr_mode,
                affectors,
            ),
            SolOp::Sub => self.sub.add_val(
                SolOp::Sub,
                val,
                proj_mult,
                res_mult,
                &normalize_sub,
                &diminish_basic,
                &revert_sub,
                aggr_mode,
                affectors,
            ),
            SolOp::PostMul => self.post_mul.add_val(
                SolOp::PostMul,
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
            SolOp::PostMulImmune => self.post_mul.add_val(
                SolOp::PostMulImmune,
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
            SolOp::PostDiv => self.post_div.add_val(
                SolOp::PostDiv,
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
            SolOp::PostPerc => self.post_perc.add_val(
                SolOp::PostPerc,
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
            SolOp::PostAssign => self.post_assign.add_val(
                SolOp::PostAssign,
                val,
                None,
                None,
                &normalize_noop,
                &diminish_noop,
                &revert_noop,
                aggr_mode,
                affectors,
            ),
            SolOp::ExtraMul => self.extra_mul.add_val(
                SolOp::ExtraMul,
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
    pub(in crate::sol::svc::calc) fn apply_dogma_mods(&mut self, base_val: AttrVal, hig: bool) -> SolAttrValInfo {
        let attr_info = SolAttrValInfo::new(base_val);
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
    pub(in crate::sol::svc::calc) fn apply_extra_mods(
        &mut self,
        attr_info: SolAttrValInfo,
        hig: bool,
    ) -> SolAttrValInfo {
        apply_mul(
            attr_info,
            self.extra_mul.get_comb_attr_info(&combine_muls, &revert_noop, hig),
        )
    }
}

struct SolAttrStack {
    stacked: SolAttrAggr,
    penalized: SolAttrAggr,
}
impl SolAttrStack {
    fn new() -> Self {
        Self {
            stacked: SolAttrAggr::new(),
            penalized: SolAttrAggr::new(),
        }
    }
    fn add_val<N, D, R>(
        &mut self,
        op: SolOp,
        initial_val: AttrVal,
        proj_mult: Option<AttrVal>,
        res_mult: Option<AttrVal>,
        normalize_func: &N,
        diminish_func: &D,
        revert_func: &R,
        penalizable: bool,
        aggr_mode: &SolAggrMode,
        affectors: SmallVec<SolAffectorInfo, 1>,
    ) where
        N: Fn(AttrVal) -> Option<AttrVal>,
        D: Fn(AttrVal, Option<AttrVal>, Option<AttrVal>) -> AttrVal,
        R: Fn(AttrVal) -> AttrVal,
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
    ) -> Option<SolAttrValInfo>
    where
        C: Fn(&mut Vec<SolAttrValInfo>, &R, bool) -> Option<SolAttrValInfo>,
        P: Fn(&mut Vec<SolAttrValInfo>, &R, bool) -> Option<SolAttrValInfo>,
        R: Fn(AttrVal) -> AttrVal,
    {
        if let Some(attr_info) = self.penalized.get_comb_attr_info(pen_func, revert_func, hig) {
            self.stacked.add_attr_info(attr_info, &SolAggrMode::Stack);
        }
        self.stacked.get_comb_attr_info(comb_func, revert_func, hig)
    }
}

struct SolAttrAggr {
    stack: Vec<SolAttrValInfo>,
    aggr_min: StMap<AggrKey, Vec<SolAttrValInfo>>,
    aggr_max: StMap<AggrKey, Vec<SolAttrValInfo>>,
}
impl SolAttrAggr {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            aggr_min: StMap::new(),
            aggr_max: StMap::new(),
        }
    }
    fn add_val<N, D, R>(
        &mut self,
        op: SolOp,
        initial_val: AttrVal,
        proj_mult: Option<AttrVal>,
        res_mult: Option<AttrVal>,
        normalize_func: &N,
        diminish_func: &D,
        revert_func: &R,
        aggr_mode: &SolAggrMode,
        affectors: SmallVec<SolAffectorInfo, 1>,
    ) where
        N: Fn(AttrVal) -> Option<AttrVal>,
        D: Fn(AttrVal, Option<AttrVal>, Option<AttrVal>) -> AttrVal,
        R: Fn(AttrVal) -> AttrVal,
    {
        let normalized_val = match normalize_func(initial_val) {
            Some(val) => val,
            None => return,
        };
        let diminished_val = diminish_func(normalized_val, proj_mult, res_mult);
        let info = SolModificationInfo::new(
            op.into(),
            initial_val,
            proj_mult,
            res_mult,
            None,
            revert_func(diminished_val),
            affectors.into_vec(),
        );
        let attr_info = SolAttrValInfo::from_effective_info(diminished_val, info);
        self.add_attr_info(attr_info, aggr_mode);
    }
    fn add_attr_info(&mut self, attr_info: SolAttrValInfo, aggr_mode: &SolAggrMode) {
        match aggr_mode {
            SolAggrMode::Stack => self.stack.push(attr_info),
            SolAggrMode::Min(key) => self.aggr_min.entry(*key).or_default().push(attr_info),
            SolAggrMode::Max(key) => self.aggr_max.entry(*key).or_default().push(attr_info),
        }
    }
    fn get_comb_attr_info<C, R>(&mut self, comb_func: &C, revert_func: &R, high_is_good: bool) -> Option<SolAttrValInfo>
    where
        C: Fn(&mut Vec<SolAttrValInfo>, &R, bool) -> Option<SolAttrValInfo>,
        R: Fn(AttrVal) -> AttrVal,
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
fn revert_noop(val: AttrVal) -> AttrVal {
    val
}
fn revert_sub(val: AttrVal) -> AttrVal {
    -val
}
fn revert_div(val: AttrVal) -> AttrVal {
    OF(1.0) / val
}
fn revert_perc(val: AttrVal) -> AttrVal {
    (val - OF(1.0)) * OF(100.0)
}

// Application functions - they treat left side and right side differently
fn apply_assign(base_attr_info: SolAttrValInfo, other_attr_info: Option<SolAttrValInfo>) -> SolAttrValInfo {
    match other_attr_info {
        // If there are any assignments, they dismiss left side as ineffective
        Some(mut other_attr_info) => {
            other_attr_info.merge_ineffective(base_attr_info);
            other_attr_info
        }
        None => base_attr_info,
    }
}
fn apply_add(mut base_attr_info: SolAttrValInfo, other_attr_info: Option<SolAttrValInfo>) -> SolAttrValInfo {
    if let Some(other_attr_info) = other_attr_info {
        base_attr_info.value += other_attr_info.value;
        base_attr_info.merge(other_attr_info);
    }
    base_attr_info
}
fn apply_mul(mut base_attr_info: SolAttrValInfo, other_attr_info: Option<SolAttrValInfo>) -> SolAttrValInfo {
    match other_attr_info {
        Some(mut other_attr_info) => match (base_attr_info.value, other_attr_info.value) {
            // Right side 0 means left side has no effect on the result
            (_, OF(0.0)) => {
                other_attr_info.merge_ineffective(base_attr_info);
                other_attr_info
            }
            // Left side 0 means right side has no effect on the result
            (OF(0.0), _) => {
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
fn combine_assigns<R>(
    attr_infos: &mut Vec<SolAttrValInfo>,
    _revert_func: &R,
    high_is_good: bool,
) -> Option<SolAttrValInfo> {
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
fn combine_adds<R>(
    attr_infos: &mut Vec<SolAttrValInfo>,
    _revert_func: &R,
    _high_is_good: bool,
) -> Option<SolAttrValInfo> {
    if attr_infos.is_empty() {
        return None;
    }
    let value = attr_infos.iter().map(|v| v.value).sum();
    let mut attr_info = SolAttrValInfo::new(value);
    for other_attr_info in attr_infos.extract_if(.., |_| true) {
        match other_attr_info.value {
            // Adding 0 is not changing the result
            OF(0.0) => attr_info.merge_ineffective(other_attr_info),
            _ => attr_info.merge(other_attr_info),
        }
    }
    Some(attr_info)
}
fn combine_muls<R>(
    attr_infos: &mut Vec<SolAttrValInfo>,
    _revert_func: &R,
    _high_is_good: bool,
) -> Option<SolAttrValInfo> {
    if attr_infos.is_empty() {
        return None;
    }
    let value = attr_infos.iter().map(|v| v.value).product();
    let mut attr_info = SolAttrValInfo::new(value);
    match value {
        // Value of 0 means that some multipliers were 0. Expose only those, and hide the rest,
        // those we hid have no effect on value anyway
        OF(0.0) => {
            for other_attr_info in attr_infos.extract_if(.., |_| true) {
                match other_attr_info.value {
                    OF(0.0) => attr_info.merge(other_attr_info),
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
                if other_attr_info.value == OF(1.0) && other_attr_info.is_single_effective() {
                    attr_info.merge_ineffective(other_attr_info)
                } else {
                    attr_info.merge(other_attr_info);
                }
            }
        }
    }
    Some(attr_info)
}
fn combine_muls_pen<R>(
    attr_infos: &mut Vec<SolAttrValInfo>,
    revert_func: &R,
    _high_is_good: bool,
) -> Option<SolAttrValInfo>
where
    R: Fn(AttrVal) -> AttrVal,
{
    // Gather positive multipliers into one chain, negative into another, with stronger
    // modifications being first
    let mut positive = Vec::new();
    let mut negative = Vec::new();
    let mut neutral = Vec::new();
    for attr_info in attr_infos.extract_if(.., |_| true) {
        if attr_info.value > OF(1.0) {
            positive.push(attr_info);
        } else if attr_info.value < OF(1.0) {
            negative.push(attr_info);
        } else {
            neutral.push(attr_info)
        }
    }
    if positive.is_empty() && negative.is_empty() {
        return None;
    }
    positive.sort_by_key(|v| -v.value);
    negative.sort_by_key(|v| v.value);
    let mut attr_info = SolAttrValInfo::new(OF(1.0));
    // Do negative chain first, since it can result in final multiplier of 0
    let negative_attr_info = get_chain_attr_info(negative, revert_func);
    attr_info.value *= negative_attr_info.value;
    attr_info.merge(negative_attr_info);
    let positive_attr_info = get_chain_attr_info(positive, revert_func);
    // It doesn't matter what is in positive chain if our multiplier is 0 already
    if attr_info.value == OF(0.0) {
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
fn extract_min(attr_infos: &mut Vec<SolAttrValInfo>) -> Option<SolAttrValInfo> {
    let index = attr_infos
        .iter()
        .enumerate()
        .min_by_key(|(_, v)| v.value)
        .map(|(index, _)| index);
    index.map(|index| attr_infos.remove(index))
}
fn extract_max(attr_infos: &mut Vec<SolAttrValInfo>) -> Option<SolAttrValInfo> {
    let index = attr_infos
        .iter()
        .enumerate()
        .max_by_key(|(_, v)| v.value)
        .map(|(index, _)| index);
    index.map(|index| attr_infos.remove(index))
}
fn get_chain_attr_info<R>(attr_infos: Vec<SolAttrValInfo>, revert_func: &R) -> SolAttrValInfo
where
    R: Fn(AttrVal) -> AttrVal,
{
    let mut attr_info = SolAttrValInfo::new(OF(1.0));
    // Special case for when first element of chain is a multiplier by 0, for the same reason as in
    // multiplication combination function. We know final chain multiplier is going to be 0, we know
    // other elements are not going to be multipliers by 0 after penalty is applied, so we just
    // expose multiplier by 0 as the only effective modification, and consider others ineffective
    let first_zero = match attr_infos.first() {
        Some(other_attr_info) => other_attr_info.value == OF(0.0),
        None => false,
    };
    for (i, mut other_attr_info) in attr_infos.into_iter().enumerate() {
        // Ignore 12th modification and further as insignificant
        if i > 10 {
            for info in other_attr_info.effective_infos.iter_mut() {
                info.stacking_mult = Some(OF(0.0));
                info.applied_val = revert_func(OF(1.0));
            }
            attr_info.merge_ineffective(other_attr_info);
        } else {
            let penalty_multiplier = PENALTY_BASE.powi((i as i32).pow(2));
            let value_multiplier = OF(1.0) + (other_attr_info.value - OF(1.0)) * penalty_multiplier;
            for info in other_attr_info.effective_infos.iter_mut() {
                info.stacking_mult = Some(OF(penalty_multiplier));
                info.applied_val = revert_func(value_multiplier);
            }
            if first_zero && i > 0 {
                attr_info.merge_ineffective(other_attr_info);
            } else {
                attr_info.value *= value_multiplier;
                attr_info.merge(other_attr_info);
            }
        }
    }
    attr_info
}
