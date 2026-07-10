//! Attribute calculator which is modified to provide info on modification instead of just value.
//!
//! Whenever regular calculator changes, those changes have to be carried over here, to keep actual
//! calculation process and modification info consistent.

use std::cmp::Ordering;

use smallvec::SmallVec;

use super::shared::{
    AddMath, AddMathAdd, AddMathSub, MultMath, MultMathDiv, MultMathMul, MultMathPerc, PENALTY_MULTS, is_penal,
};
use crate::{
    ad::AItemCatId,
    api::Op,
    num::{PValue, Value},
    svc::calc::{AggrKey, AggrMode, CalcModInfo, CalcModInfoAffector, CalcOp},
    util::RMap,
};

pub(in crate::svc::calc) struct AttrValInfo {
    pub(in crate::svc::calc) value: Value,
    pub(in crate::svc::calc) effective_infos: Vec<CalcModInfo>,
    pub(in crate::svc::calc) filtered_infos: Vec<CalcModInfo>,
}
impl AttrValInfo {
    pub(in crate::svc::calc) fn new(value: Value) -> Self {
        Self {
            value,
            effective_infos: Vec::new(),
            filtered_infos: Vec::new(),
        }
    }
    fn from_effective_info(value: Value, info: CalcModInfo) -> Self {
        Self {
            value,
            effective_infos: vec![info],
            filtered_infos: Vec::new(),
        }
    }
    fn merge(&mut self, other: AttrValInfo) {
        self.effective_infos.extend(other.effective_infos);
        self.filtered_infos.extend(other.filtered_infos);
    }
    fn merge_ineffective(&mut self, other: AttrValInfo) {
        self.filtered_infos.extend(other.effective_infos);
        self.filtered_infos.extend(other.filtered_infos);
    }
    fn is_single_effective(&self) -> bool {
        self.effective_infos.len() <= 1
    }
}

pub(in crate::svc::calc) struct ModAccumInfo {
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
impl ModAccumInfo {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
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
        res_mult: Option<PValue>,
        attr_pen: bool,
        item_cat: AItemCatId,
        aggr_mode: AggrMode,
        affectors: SmallVec<[CalcModInfoAffector; 1]>,
    ) {
        match op {
            CalcOp::PreAssign => self
                .pre_assign
                .add_raw_val(op, val, proj_mult, res_mult, aggr_mode, affectors),
            CalcOp::PreMul => self.pre_mul.add_raw_val(
                op,
                val,
                proj_mult,
                res_mult,
                aggr_mode,
                is_penal(attr_pen, &item_cat),
                affectors,
            ),
            CalcOp::PreDiv => self.pre_div.add_raw_val(
                op,
                val,
                proj_mult,
                res_mult,
                aggr_mode,
                is_penal(attr_pen, &item_cat),
                affectors,
            ),
            CalcOp::Add => self.add.add_raw_val(op, val, proj_mult, res_mult, aggr_mode, affectors),
            CalcOp::Sub => self.sub.add_raw_val(op, val, proj_mult, res_mult, aggr_mode, affectors),
            CalcOp::PostMul => self.post_mul.add_raw_val(
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
                .add_raw_val(op, val, proj_mult, res_mult, aggr_mode, false, affectors),
            CalcOp::PostDiv => self.post_div.add_raw_val(
                op,
                val,
                proj_mult,
                res_mult,
                aggr_mode,
                is_penal(attr_pen, &item_cat),
                affectors,
            ),
            CalcOp::PostPerc => self.post_perc.add_raw_val(
                op,
                val,
                proj_mult,
                res_mult,
                aggr_mode,
                is_penal(attr_pen, &item_cat),
                affectors,
            ),
            CalcOp::PostPercImmune => self
                .post_perc
                .add_raw_val(op, val, proj_mult, res_mult, aggr_mode, false, affectors),
            CalcOp::PostAssign => self
                .post_assign
                .add_raw_val(op, val, proj_mult, res_mult, aggr_mode, affectors),
            CalcOp::ExtraAdd => self
                .extra_add
                .add_raw_val(op, val, proj_mult, res_mult, aggr_mode, affectors),
            CalcOp::ExtraMul => self
                .extra_mul
                .add_raw_val(op, val, proj_mult, res_mult, aggr_mode, false, affectors),
        };
    }
    pub(in crate::svc::calc) fn apply_dogma_mods(&mut self, attr_info: AttrValInfo, hig: bool) -> AttrValInfo {
        let attr_info = self.pre_assign.modify_info(attr_info, hig);
        let attr_info = self.pre_mul.modify_info(attr_info);
        let attr_info = self.pre_div.modify_info(attr_info);
        let attr_info = self.add.modify_info(attr_info);
        let attr_info = self.sub.modify_info(attr_info);
        let attr_info = self.post_mul.modify_info(attr_info);
        let attr_info = self.post_div.modify_info(attr_info);
        let attr_info = self.post_perc.modify_info(attr_info);
        self.post_assign.modify_info(attr_info, hig)
    }
    pub(in crate::svc::calc) fn apply_extra_mods(&mut self, attr_info: AttrValInfo) -> AttrValInfo {
        let attr_info = self.extra_add.modify_info(attr_info);
        self.extra_mul.modify_info(attr_info)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Assignment
////////////////////////////////////////////////////////////////////////////////////////////////////
struct AccumAssign {
    stack: Vec<AttrValInfo>,
    aggr_min: RMap<AggrKey, Vec<AttrValInfo>>,
    aggr_max: RMap<AggrKey, Vec<AttrValInfo>>,
}
impl AccumAssign {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            aggr_min: RMap::new(),
            aggr_max: RMap::new(),
        }
    }
    fn add_raw_val(
        &mut self,
        op: CalcOp,
        added_raw: Value,
        proj_mult: Option<PValue>,
        res_mult: Option<PValue>,
        aggr_mode: AggrMode,
        affectors: SmallVec<[CalcModInfoAffector; 1]>,
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
            added_raw,
            CalcModInfo {
                op: Op::from_calc_op(op),
                initial_str: added_raw,
                range_mult: proj_mult,
                resist_mult: res_mult,
                stacking_mult: None,
                applied_str: added_raw,
                affectors,
            },
        );
        match aggr_mode {
            AggrMode::Stack => self.stack.push(info),
            AggrMode::Min(key) => self.aggr_min.entry(key).or_default().push(info),
            AggrMode::Max(key) => self.aggr_max.entry(key).or_default().push(info),
        }
    }
    fn modify_info(&mut self, base_info: AttrValInfo, attr_hig: bool) -> AttrValInfo {
        resolve_aggr_min(&mut self.stack, &mut self.aggr_min);
        resolve_aggr_max(&mut self.stack, &mut self.aggr_max);
        let comb_info = self.get_comb_info(attr_hig);
        Self::apply_comb_to_base(base_info, comb_info)
    }
    // Functions which are not part of public interface
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
        // If there are any assignments, they dismiss base info as ineffective
        if let Some(mut comb_info) = comb_info {
            comb_info.merge_ineffective(base_info);
            return comb_info;
        }
        base_info
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
    stack: Vec<AttrValInfo>,
    aggr_min: RMap<AggrKey, Vec<AttrValInfo>>,
    aggr_max: RMap<AggrKey, Vec<AttrValInfo>>,
    math: std::marker::PhantomData<M>,
}
impl<M> AddAccum<M>
where
    M: AddMath,
{
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            aggr_min: RMap::new(),
            aggr_max: RMap::new(),
            math: std::marker::PhantomData,
        }
    }
    fn add_raw_val(
        &mut self,
        op: CalcOp,
        added_raw: Value,
        proj_mult: Option<PValue>,
        res_mult: Option<PValue>,
        aggr_mode: AggrMode,
        affectors: SmallVec<[CalcModInfoAffector; 1]>,
    ) {
        let diminished_raw = M::diminish_raw(added_raw, proj_mult, res_mult);
        let info = AttrValInfo::from_effective_info(
            diminished_raw,
            CalcModInfo {
                op: Op::from_calc_op(op),
                initial_str: added_raw,
                range_mult: proj_mult,
                resist_mult: res_mult,
                stacking_mult: None,
                applied_str: diminished_raw,
                affectors,
            },
        );
        match aggr_mode {
            AggrMode::Stack => self.stack.push(info),
            AggrMode::Min(key) => self.aggr_min.entry(key).or_default().push(info),
            AggrMode::Max(key) => self.aggr_max.entry(key).or_default().push(info),
        }
    }
    fn modify_info(&mut self, base_info: AttrValInfo) -> AttrValInfo {
        resolve_aggr_min(&mut self.stack, &mut self.aggr_min);
        resolve_aggr_max(&mut self.stack, &mut self.aggr_max);
        let comb_info = self.get_comb_info();
        Self::apply_comb_to_base(base_info, comb_info)
    }
    // Functions which are not part of public interface
    fn get_comb_info(&mut self) -> Option<AttrValInfo> {
        if self.stack.is_empty() {
            return None;
        }
        let comb_value = self.stack.iter().map(|v| v.value).sum();
        let mut comb_info = AttrValInfo::new(comb_value);
        for other_info in self.stack.drain(..) {
            match other_info.value {
                // Adding/subtracting 0 is not changing the result
                Value::ZERO => comb_info.merge_ineffective(other_info),
                _ => comb_info.merge(other_info),
            }
        }
        Some(comb_info)
    }
    fn apply_comb_to_base(mut base_info: AttrValInfo, comb_info: Option<AttrValInfo>) -> AttrValInfo {
        if let Some(comb_info) = comb_info {
            base_info.value = M::apply_raw(base_info.value, comb_info.value);
            base_info.merge(comb_info);
        }
        base_info
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
    // Multipliers in values
    stack: Vec<AttrValInfo>,
    // Internally stores multiplier change, but takes raw values as input, and exposes multipliers
    pens: Pens<M>,
    // Store raw values
    aggr_min: RMap<AggrKey, Vec<PenEntry>>,
    aggr_max: RMap<AggrKey, Vec<PenEntry>>,
}
impl<M> MultAccum<M>
where
    M: MultMath,
{
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            pens: Pens::new(),
            aggr_min: RMap::new(),
            aggr_max: RMap::new(),
        }
    }
    fn add_raw_val(
        &mut self,
        op: CalcOp,
        added_raw: Value,
        proj_mult: Option<PValue>,
        res_mult: Option<PValue>,
        aggr_mode: AggrMode,
        pen: bool,
        affectors: SmallVec<[CalcModInfoAffector; 1]>,
    ) {
        if !M::check_raw(added_raw) {
            return;
        }
        let diminished_raw = M::diminish_raw(added_raw, proj_mult, res_mult);
        let info_raw = AttrValInfo::from_effective_info(
            diminished_raw,
            CalcModInfo {
                op: Op::from_calc_op(op),
                initial_str: added_raw,
                range_mult: proj_mult,
                resist_mult: res_mult,
                stacking_mult: None,
                applied_str: diminished_raw,
                affectors,
            },
        );
        match aggr_mode {
            AggrMode::Stack => Self::add_raw_stacking_info(&mut self.stack, &mut self.pens, info_raw, pen),
            AggrMode::Min(key) => self
                .aggr_min
                .entry(key)
                .or_default()
                .push(PenEntry { info: info_raw, pen }),
            AggrMode::Max(key) => self
                .aggr_max
                .entry(key)
                .or_default()
                .push(PenEntry { info: info_raw, pen }),
        }
    }
    fn modify_info(&mut self, base_info: AttrValInfo) -> AttrValInfo {
        self.resolve_aggrs();
        self.resolve_pens();
        let comb_info = self.get_comb_info();
        Self::apply_comb_to_base(base_info, comb_info)
    }
    // Functions which are not part of public interface
    fn add_raw_stacking_info(stack: &mut Vec<AttrValInfo>, pens: &mut Pens<M>, mut info: AttrValInfo, pen: bool) {
        match pen {
            // Add with raw value to penalizable container, it will be converted there internally
            true => pens.add_val(info),
            false => {
                // Convert raw to multiplier before adding to stack, since that's what stack stores
                info.value = M::raw_to_mult(info.value);
                stack.push(info)
            }
        }
    }
    fn resolve_aggrs(&mut self) {
        for (_aggr_key, mut grouped_entries) in self.aggr_min.drain() {
            if let Some(mut min_entry) = extract_pen_min(&mut grouped_entries) {
                for other_entry in grouped_entries.into_iter() {
                    min_entry.info.merge_ineffective(other_entry.info)
                }
                Self::add_raw_stacking_info(&mut self.stack, &mut self.pens, min_entry.info, min_entry.pen);
            }
        }
        for (_aggr_key, mut grouped_entries) in self.aggr_max.drain() {
            if let Some(mut max_entry) = extract_pen_max(&mut grouped_entries) {
                for other_entry in grouped_entries.into_iter() {
                    max_entry.info.merge_ineffective(other_entry.info)
                }
                Self::add_raw_stacking_info(&mut self.stack, &mut self.pens, max_entry.info, max_entry.pen);
            }
        }
    }
    fn resolve_pens(&mut self) {
        if let Some(pen_info_mult) = self.pens.get_comb_info() {
            self.stack.push(pen_info_mult);
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

struct PenEntry {
    info: AttrValInfo,
    pen: bool,
}

struct Pens<M>
where
    M: MultMath,
{
    // All the fields store multiplier change in value
    pos: Vec<AttrValInfo>,
    neut: Vec<AttrValInfo>,
    neg: Vec<AttrValInfo>,
    math: std::marker::PhantomData<M>,
}
impl<M> Pens<M>
where
    M: MultMath,
{
    fn new() -> Self {
        Self {
            pos: Vec::new(),
            neut: Vec::new(),
            neg: Vec::new(),
            math: std::marker::PhantomData,
        }
    }
    fn add_val(&mut self, mut added_info: AttrValInfo) {
        added_info.value = M::raw_to_mult_change(added_info.value);
        match added_info.value.cmp(&Value::ZERO) {
            Ordering::Greater => self.pos.push(added_info),
            Ordering::Equal => self.neut.push(added_info),
            Ordering::Less => self.neg.push(added_info),
        }
    }
    fn get_comb_info(&mut self) -> Option<AttrValInfo> {
        if self.neg.is_empty() && self.neut.is_empty() && self.pos.is_empty() {
            return None;
        }
        let mut comb_info_mult = AttrValInfo::new(Value::ONE);
        // Do negative chain first, since it can result in final multiplier of 0
        if !self.neg.is_empty() {
            self.neg.sort_unstable_by_key(|v| v.value);
            let neg_info_mult = Self::get_penalty_chain_info(&mut self.neg);
            comb_info_mult.value *= neg_info_mult.value;
            comb_info_mult.merge(neg_info_mult);
        }
        if !self.pos.is_empty() {
            self.pos.sort_unstable_by_key(|v| -v.value);
            let pos_info_mult = Self::get_penalty_chain_info(&mut self.pos);
            match comb_info_mult.value == Value::ZERO {
                // It doesn't matter what is in positive chain if our multiplier is 0 already
                true => comb_info_mult.merge_ineffective(pos_info_mult),
                false => {
                    comb_info_mult.value *= pos_info_mult.value;
                    comb_info_mult.merge(pos_info_mult);
                }
            }
        }
        // Neutral changes are always ineffective
        if !self.neut.is_empty() {
            for other_info_mult in self.neut.drain(..) {
                comb_info_mult.merge_ineffective(other_info_mult);
            }
        }
        Some(comb_info_mult)
    }
    // Take a slice of mult changes, return mult
    fn get_penalty_chain_info(chain_infos_mult_change: &mut Vec<AttrValInfo>) -> AttrValInfo {
        let mut new_info_mult = AttrValInfo::new(Value::ONE);
        // Special case for when first element of chain is a multiplier by 0, for the same reason as
        // in multiplication combination function. We know final chain multiplier is going to be 0,
        // we know other elements are not going to be multipliers by 0 after penalty is applied, so
        // we just expose multiplier by 0 as the only effective modification, and consider others
        // ineffective
        let first_zeroing = match chain_infos_mult_change.first() {
            Some(chain_info_mult_change) => chain_info_mult_change.value == -Value::ONE,
            None => false,
        };
        for (i, mut other_info_mult_change) in chain_infos_mult_change.drain(..).enumerate() {
            match PENALTY_MULTS.get(i) {
                Some(&pen_mult) => {
                    let val_mult = other_info_mult_change.value.mul_add(pen_mult.into_value(), Value::ONE);
                    for other_info_mod in other_info_mult_change.effective_infos.iter_mut() {
                        other_info_mod.stacking_mult = Some(pen_mult);
                        other_info_mod.applied_str = M::mult_to_raw(val_mult);
                    }
                    match first_zeroing && i > 0 {
                        true => new_info_mult.merge_ineffective(other_info_mult_change),
                        false => {
                            new_info_mult.value *= val_mult;
                            new_info_mult.merge(other_info_mult_change);
                        }
                    }
                }
                // Modifications past those which have penalty multiplier are insignificant
                None => {
                    for other_info_mod in other_info_mult_change.effective_infos.iter_mut() {
                        other_info_mod.stacking_mult = Some(PValue::ZERO);
                        other_info_mod.applied_str = M::mult_to_raw(Value::ONE);
                    }
                    new_info_mult.merge_ineffective(other_info_mult_change);
                }
            }
        }
        new_info_mult
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

fn resolve_aggr_min(stack: &mut Vec<AttrValInfo>, aggr_min: &mut RMap<AggrKey, Vec<AttrValInfo>>) {
    for (_aggr_key, mut grouped_infos) in aggr_min.drain() {
        if let Some(mut min_info) = extract_min(&mut grouped_infos) {
            for other_info in grouped_infos.into_iter() {
                min_info.merge_ineffective(other_info)
            }
            stack.push(min_info);
        }
    }
}
fn resolve_aggr_max(stack: &mut Vec<AttrValInfo>, aggr_max: &mut RMap<AggrKey, Vec<AttrValInfo>>) {
    for (_aggr_key, mut grouped_infos) in aggr_max.drain() {
        if let Some(mut max_info) = extract_max(&mut grouped_infos) {
            for other_info in grouped_infos.into_iter() {
                max_info.merge_ineffective(other_info)
            }
            stack.push(max_info);
        }
    }
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
