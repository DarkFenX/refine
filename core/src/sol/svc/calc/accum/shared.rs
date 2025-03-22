use crate::{
    consts,
    defs::{AttrVal, EItemCatId, OF},
};

const PENALTY_IMMUNE_CATS: [EItemCatId; 5] = [
    consts::itemcats::SHIP,
    consts::itemcats::CHARGE,
    consts::itemcats::SKILL,
    consts::itemcats::IMPLANT,
    consts::itemcats::SUBSYSTEM,
];
// Source expression: 1 / e^((1 / 2.67)^2)
pub(super) const PENALTY_BASE: AttrVal = OF(0.86911998080039742919922218788997270166873931884765625);

pub(in crate::sol::svc::calc) fn is_penal(attr_penalizable: bool, affector_item_cat_id: &EItemCatId) -> bool {
    attr_penalizable && !PENALTY_IMMUNE_CATS.contains(affector_item_cat_id)
}

// Normalization functions
pub(super) fn normalize_noop(val: AttrVal) -> Option<AttrVal> {
    Some(val)
}
pub(super) fn normalize_sub(val: AttrVal) -> Option<AttrVal> {
    Some(-val)
}
pub(super) fn normalize_div(val: AttrVal) -> Option<AttrVal> {
    if val == OF(0.0) {
        return None;
    }
    Some(OF(1.0) / val)
}
pub(super) fn normalize_perc(val: AttrVal) -> Option<AttrVal> {
    Some(OF(1.0) + val / OF(100.0))
}

// Apply diminishing factors (resistance- and projection-related reductions)
pub(super) fn diminish_noop(val: AttrVal, _proj_mult: Option<AttrVal>, _res_mult: Option<AttrVal>) -> AttrVal {
    val
}
pub(super) fn diminish_basic(mut val: AttrVal, proj_mult: Option<AttrVal>, res_mult: Option<AttrVal>) -> AttrVal {
    if let Some(proj_mult) = proj_mult {
        val *= proj_mult;
    }
    if let Some(res_mult) = res_mult {
        val *= res_mult;
    }
    val
}
pub(super) fn diminish_mul(val: AttrVal, proj_mult: Option<AttrVal>, res_mult: Option<AttrVal>) -> AttrVal {
    if res_mult.is_none() && proj_mult.is_none() {
        return val;
    }
    diminish_basic(val - OF(1.0), res_mult, proj_mult) + OF(1.0)
}
