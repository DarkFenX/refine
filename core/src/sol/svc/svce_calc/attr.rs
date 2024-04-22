use crate::{
    defs::{AggrKey, AttrVal, EItemCatId},
    ec,
    sol::svc::svce_calc::{SolModAggrMode, SolModOp},
    util::StMap,
};

const PENALTY_IMMUNE_CATS: [EItemCatId; 5] = [
    ec::itemcats::SHIP,
    ec::itemcats::CHARGE,
    ec::itemcats::SKILL,
    ec::itemcats::IMPLANT,
    ec::itemcats::SUBSYSTEM,
];
// Source expression: 1 / e^((1 / 2.67)^2)
const PENALTY_BASE: f64 = 0.86911998080039742919922218788997270166873931884765625;

pub(super) struct SolAttrValues {
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
impl SolAttrValues {
    pub(super) fn new() -> Self {
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
    pub(super) fn add_val(
        &mut self,
        val: AttrVal,
        op: &SolModOp,
        attr_pen: bool,
        item_cat: &EItemCatId,
        aggr_mode: &SolModAggrMode,
    ) {
        match op {
            SolModOp::PreAssign => self.pre_assign.add_val(val, norm_noop, aggr_mode),
            SolModOp::PreMul => self
                .pre_mul
                .add_val(val, norm_noop, is_penal(attr_pen, item_cat), aggr_mode),
            SolModOp::PreDiv => self
                .pre_div
                .add_val(val, norm_div, is_penal(attr_pen, item_cat), aggr_mode),
            SolModOp::Add => self.add.add_val(val, norm_noop, aggr_mode),
            SolModOp::Sub => self.sub.add_val(val, norm_sub, aggr_mode),
            SolModOp::PostMul => self
                .post_mul
                .add_val(val, norm_noop, is_penal(attr_pen, item_cat), aggr_mode),
            SolModOp::PostMulImmune => self.post_mul.add_val(val, norm_noop, false, aggr_mode),
            SolModOp::PostDiv => self
                .post_div
                .add_val(val, norm_div, is_penal(attr_pen, item_cat), aggr_mode),
            SolModOp::PostPerc => self
                .post_perc
                .add_val(val, norm_perc, is_penal(attr_pen, item_cat), aggr_mode),
            SolModOp::PostAssign => self.post_assign.add_val(val, norm_noop, aggr_mode),
            SolModOp::ExtraMul => self.extra_mul.add_val(val, norm_noop, aggr_mode),
        };
    }
    pub(super) fn apply_dogma_mods(&mut self, base_val: AttrVal, hig: bool) -> AttrVal {
        let val = apply_assign(base_val, self.pre_assign.get_comb_val(combine_assigns, hig));
        let val = apply_mul(val, self.pre_mul.get_comb_val(combine_muls, combine_muls_pen, hig));
        let val = apply_mul(val, self.pre_div.get_comb_val(combine_muls, combine_muls_pen, hig));
        let val = apply_add(val, self.add.get_comb_val(combine_adds, hig));
        let val = apply_add(val, self.sub.get_comb_val(combine_adds, hig));
        let val = apply_mul(val, self.post_mul.get_comb_val(combine_muls, combine_muls_pen, hig));
        let val = apply_mul(val, self.post_div.get_comb_val(combine_muls, combine_muls_pen, hig));
        let val = apply_mul(val, self.post_perc.get_comb_val(combine_muls, combine_muls_pen, hig));
        let val = apply_assign(val, self.post_assign.get_comb_val(combine_assigns, hig));
        val
    }
    pub(super) fn apply_extra_mods(&mut self, dogma_val: AttrVal, hig: bool) -> AttrVal {
        let val = apply_mul(dogma_val, self.extra_mul.get_comb_val(combine_muls, hig));
        val
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
    fn add_val<F>(&mut self, val: AttrVal, norm_func: F, penalizable: bool, aggr_mode: &SolModAggrMode)
    where
        F: Fn(AttrVal) -> Option<AttrVal>,
    {
        if penalizable {
            self.penalized.add_val(val, norm_func, aggr_mode)
        } else {
            self.stacked.add_val(val, norm_func, aggr_mode)
        }
    }
    fn get_comb_val<F1, F2>(&mut self, comb_func: F1, pen_func: F2, hig: bool) -> Option<AttrVal>
    where
        F1: Fn(&Vec<AttrVal>, bool) -> Option<AttrVal>,
        F2: Fn(&Vec<AttrVal>, bool) -> Option<AttrVal>,
    {
        if let Some(val) = self.penalized.get_comb_val(pen_func, hig) {
            self.stacked.add_val(val, norm_noop, &SolModAggrMode::Stack);
        }
        self.stacked.get_comb_val(comb_func, hig)
    }
}

struct SolAttrAggr {
    stack: Vec<AttrVal>,
    aggr_min: StMap<AggrKey, Vec<AttrVal>>,
    aggr_max: StMap<AggrKey, Vec<AttrVal>>,
}
impl SolAttrAggr {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            aggr_min: StMap::new(),
            aggr_max: StMap::new(),
        }
    }
    fn add_val<F>(&mut self, val: AttrVal, norm_func: F, aggr_mode: &SolModAggrMode)
    where
        F: Fn(AttrVal) -> Option<AttrVal>,
    {
        let val = match norm_func(val) {
            Some(val) => val,
            None => return,
        };
        match aggr_mode {
            SolModAggrMode::Stack => self.stack.push(val),
            SolModAggrMode::Min(key) => self.aggr_min.entry(*key).or_insert_with(|| Vec::new()).push(val),
            SolModAggrMode::Max(key) => self.aggr_max.entry(*key).or_insert_with(|| Vec::new()).push(val),
        }
    }
    fn get_comb_val<F>(&mut self, comb_func: F, high_is_good: bool) -> Option<AttrVal>
    where
        F: Fn(&Vec<AttrVal>, bool) -> Option<AttrVal>,
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
        comb_func(&self.stack, high_is_good)
    }
}

// Normalization functions
fn norm_noop(val: AttrVal) -> Option<AttrVal> {
    Some(val)
}
fn norm_sub(val: AttrVal) -> Option<AttrVal> {
    Some(-val)
}
fn norm_div(val: AttrVal) -> Option<AttrVal> {
    if val == 0.0 {
        return None;
    }
    Some(1.0 / val)
}
fn norm_perc(val: AttrVal) -> Option<AttrVal> {
    Some(1.0 + val / 100.0)
}

// Application functions
fn apply_assign(base_val: AttrVal, other_val: Option<AttrVal>) -> AttrVal {
    other_val.unwrap_or_else(|| base_val)
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
fn combine_assigns(vals: &Vec<AttrVal>, high_is_good: bool) -> Option<AttrVal> {
    match high_is_good {
        true => get_max(vals),
        false => get_min(vals),
    }
}
fn combine_adds(vals: &Vec<AttrVal>, _: bool) -> Option<AttrVal> {
    if vals.is_empty() {
        return None;
    }
    Some(vals.iter().sum())
}
fn combine_muls(vals: &Vec<AttrVal>, _: bool) -> Option<AttrVal> {
    if vals.is_empty() {
        return None;
    }
    Some(vals.iter().product())
}

// Penalized combination functions
fn combine_muls_pen(vals: &Vec<AttrVal>, _: bool) -> Option<AttrVal> {
    penalize_vals(vals.iter().map(|v| *v))
}

// Misc functions
pub(super) fn is_penal(attr_penalizable: bool, src_item_cat_id: &EItemCatId) -> bool {
    attr_penalizable && !PENALTY_IMMUNE_CATS.contains(src_item_cat_id)
}
fn get_min(vals: &Vec<AttrVal>) -> Option<AttrVal> {
    vals.iter().min_by(|a, b| a.total_cmp(b)).copied()
}
fn get_max(vals: &Vec<AttrVal>) -> Option<AttrVal> {
    vals.iter().max_by(|a, b| a.total_cmp(b)).copied()
}
fn penalize_vals(vals: impl Iterator<Item = AttrVal>) -> Option<AttrVal> {
    // Gather positive multipliers into one chain, negative into another, with stronger
    // modifications being first
    let mut positive = Vec::new();
    let mut negative = Vec::new();
    for val in vals {
        if val >= 1.0 {
            positive.push(val);
        } else {
            negative.push(val);
        }
    }
    if positive.is_empty() && negative.is_empty() {
        return None;
    }
    positive.sort_by(|a, b| b.partial_cmp(a).unwrap());
    negative.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Some(get_chain_val(positive) * get_chain_val(negative))
}
fn get_chain_val(vals: Vec<AttrVal>) -> AttrVal {
    let mut val = 1.0;
    for (i, mod_val) in vals.iter().enumerate() {
        // Ignore 12th modification and further as non-significant
        if i > 10 {
            break;
        }
        val *= 1.0 + (mod_val - 1.0) * PENALTY_BASE.powi((i as i32).pow(2));
    }
    val
}
