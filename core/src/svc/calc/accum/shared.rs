use crate::{
    ad::AItemCatId,
    num::{PValue, Value},
};

const PENALTY_IMMUNE_ITEM_CATS: [AItemCatId; 5] = [
    AItemCatId::SHIP,
    AItemCatId::CHARGE,
    AItemCatId::SKILL,
    AItemCatId::IMPLANT,
    AItemCatId::SUBSYSTEM,
];
// Result of calculation of math.exp((i / 2.67) ** 2.0) using 64-bit python 2.7, with i being
// position of penalizable value in chain. In EVE client, it seems to have max of 8 values, after
// which modifications are ignored.
pub(super) const PENALTY_MULTS: [PValue; 8] = [
    PValue::ONE / PValue::from_f64_clamped(f64::from_bits(0x3ff0000000000000)),
    PValue::ONE / PValue::from_f64_clamped(f64::from_bits(0x3ff268d024fc2657)),
    PValue::ONE / PValue::from_f64_clamped(f64::from_bits(0x3ffc0a9eea34dd40)),
    PValue::ONE / PValue::from_f64_clamped(f64::from_bits(0x400c45e565788da0)),
    PValue::ONE / PValue::from_f64_clamped(f64::from_bits(0x4022de860d1e1273)),
    PValue::ONE / PValue::from_f64_clamped(f64::from_bits(0x4040abec60cb53f1)),
    PValue::ONE / PValue::from_f64_clamped(f64::from_bits(0x4063800e9ca1aa8e)),
    PValue::ONE / PValue::from_f64_clamped(f64::from_bits(0x408e320fff24307e)),
];

pub(in crate::svc::calc) fn is_penal(attr_penalizable: bool, affector_item_cat_aid: &AItemCatId) -> bool {
    attr_penalizable && !PENALTY_IMMUNE_ITEM_CATS.contains(affector_item_cat_aid)
}

// Normalization functions
pub(super) fn normalize_noop(val: Value) -> Option<Value> {
    Some(val)
}
pub(super) fn normalize_sub(val: Value) -> Option<Value> {
    Some(-val)
}
pub(super) fn normalize_div(val: Value) -> Option<Value> {
    if val == Value::ZERO {
        return None;
    }
    Some(Value::ONE / val)
}
pub(super) fn normalize_perc(val: Value) -> Option<Value> {
    Some(Value::ONE + val / Value::HUNDRED)
}

// Apply diminishing factors (resistance- and projection-related reductions)
pub(super) fn diminish_basic(mut val: Value, proj_mult: Option<PValue>, res_mult: Option<PValue>) -> Value {
    if let Some(proj_mult) = proj_mult {
        val *= proj_mult;
    }
    if let Some(res_mult) = res_mult {
        val *= res_mult;
    }
    val
}
pub(super) fn diminish_mul(val: Value, proj_mult: Option<PValue>, res_mult: Option<PValue>) -> Value {
    if res_mult.is_none() && proj_mult.is_none() {
        return val;
    }
    diminish_basic(val - Value::ONE, res_mult, proj_mult) + Value::ONE
}

// Multipliers affect assign operations differently: if any of multipliers is 0.0, then modification
// is not applied altogether, otherwise it is applied fully. There are no such modifiers in EVE,
// but the lib makes it to work this way.
pub(super) fn preprocess_assign_diminish_mult(mult: Option<PValue>) -> Option<Option<PValue>> {
    match mult {
        // None means modification shouldn't be added
        Some(PValue::ZERO) => None,
        Some(_) => Some(Some(PValue::ONE)),
        None => Some(None),
    }
}
