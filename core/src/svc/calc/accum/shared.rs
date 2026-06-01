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

pub(super) fn is_penal(attr_penalizable: bool, affector_item_cat_aid: &AItemCatId) -> bool {
    attr_penalizable && !PENALTY_IMMUNE_ITEM_CATS.contains(affector_item_cat_aid)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Value conversion / manipulation
////////////////////////////////////////////////////////////////////////////////////////////////////
// TODO: check which are actually shared, if not - decide if they need to be moved
pub(super) fn mul_to_mul_change(mul: Value) -> Value {
    mul - Value::ONE
}

pub(super) fn mul_change_to_mul(mul_change: Value) -> Value {
    mul_change + Value::ONE
}

pub(super) fn div_to_mul_change(div: Value) -> Value {
    Value::ONE / div - Value::ONE
}

pub(super) fn perc_change_to_mul_change(perc_change: Value) -> Value {
    perc_change * Value::HUNDREDTH
}

pub(super) fn perc_change_to_mul(perc_change: Value) -> Value {
    perc_change.mul_add(Value::HUNDREDTH, Value::ONE)
}
