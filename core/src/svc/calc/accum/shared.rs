use crate::{
    ac,
    ad::AItemCatId,
    def::{AttrVal, OF},
};

const PENALTY_IMMUNE_ITEM_CATS: [AItemCatId; 5] = [
    ac::itemcats::SHIP,
    ac::itemcats::CHARGE,
    ac::itemcats::SKILL,
    ac::itemcats::IMPLANT,
    ac::itemcats::SUBSYSTEM,
];
// Result of calculation of math.exp((i / 2.67) ** 2.0) using 64-bit python 2.7, with i being
// position of penalizable value in chain. In EVE client, it seems to have max of 8 values, after
// which modifications are ignored.
pub(super) const PENALTY_DENOMINATORS: [AttrVal; 8] = [
    OF(f64::from_bits(0x3ff0000000000000)),
    OF(f64::from_bits(0x3ff268d024fc2657)),
    OF(f64::from_bits(0x3ffc0a9eea34dd40)),
    OF(f64::from_bits(0x400c45e565788da0)),
    OF(f64::from_bits(0x4022de860d1e1273)),
    OF(f64::from_bits(0x4040abec60cb53f1)),
    OF(f64::from_bits(0x4063800e9ca1aa8e)),
    OF(f64::from_bits(0x408e320fff24307e)),
];

pub(in crate::svc::calc) fn is_penal(attr_penalizable: bool, affector_a_item_cat_id: &AItemCatId) -> bool {
    attr_penalizable && !PENALTY_IMMUNE_ITEM_CATS.contains(affector_a_item_cat_id)
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

// Multipliers affect assign operations differently: if any of multipliers is 0.0, then modification
// is not applied altogether, otherwise it is applied fully. There are no such modifiers in EVE,
// but the lib makes it to work this way.
pub(super) fn preprocess_assign_diminish_mult(mult: Option<AttrVal>) -> Option<Option<AttrVal>> {
    match mult {
        // None means modification shouldn't be added
        Some(OF(0.0)) => None,
        Some(_) => Some(Some(OF(1.0))),
        None => Some(None),
    }
}
