use crate::{PValue, UnitInterval, Value, ad::AItemCatId};

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
pub(super) const PENALTY_MULTS: [UnitInterval; 8] = [
    UnitInterval::from_f64_clamped(1.0 / f64::from_bits(0x3ff0000000000000)),
    UnitInterval::from_f64_clamped(1.0 / f64::from_bits(0x3ff268d024fc2657)),
    UnitInterval::from_f64_clamped(1.0 / f64::from_bits(0x3ffc0a9eea34dd40)),
    UnitInterval::from_f64_clamped(1.0 / f64::from_bits(0x400c45e565788da0)),
    UnitInterval::from_f64_clamped(1.0 / f64::from_bits(0x4022de860d1e1273)),
    UnitInterval::from_f64_clamped(1.0 / f64::from_bits(0x4040abec60cb53f1)),
    UnitInterval::from_f64_clamped(1.0 / f64::from_bits(0x4063800e9ca1aa8e)),
    UnitInterval::from_f64_clamped(1.0 / f64::from_bits(0x408e320fff24307e)),
];

pub(super) fn is_penal(attr_penalizable: bool, affector_item_cat_aid: &AItemCatId) -> bool {
    attr_penalizable && !PENALTY_IMMUNE_ITEM_CATS.contains(affector_item_cat_aid)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Additive math
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) trait AddMath {
    fn diminish_raw(raw: Value, proj_mult: Option<PValue>, res_mult: Option<UnitInterval>) -> Value;
    fn apply_raw(base: Value, raw: Value) -> Value;
}

pub(super) struct AddMathAdd;
impl AddMath for AddMathAdd {
    fn diminish_raw(raw: Value, proj_mult: Option<PValue>, res_mult: Option<UnitInterval>) -> Value {
        diminish_normal(raw, proj_mult, res_mult)
    }
    fn apply_raw(base: Value, raw: Value) -> Value {
        base + raw
    }
}

pub(super) struct AddMathSub;
impl AddMath for AddMathSub {
    fn diminish_raw(raw: Value, proj_mult: Option<PValue>, res_mult: Option<UnitInterval>) -> Value {
        diminish_normal(raw, proj_mult, res_mult)
    }
    fn apply_raw(base: Value, raw: Value) -> Value {
        base - raw
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Multiplicative math
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) trait MultMath {
    fn check_raw(raw: Value) -> bool;
    fn diminish_raw(raw: Value, proj_mult: Option<PValue>, res_mult: Option<UnitInterval>) -> Value;
    fn apply_raw(base: Value, raw: Value) -> Value;
    fn raw_to_mult(raw: Value) -> Value;
    fn raw_to_mult_change(raw: Value) -> Value;
    fn mult_to_raw(mult: Value) -> Value;
}

pub(super) struct MultMathMul;
impl MultMath for MultMathMul {
    fn check_raw(_raw: Value) -> bool {
        true
    }
    fn diminish_raw(raw: Value, proj_mult: Option<PValue>, res_mult: Option<UnitInterval>) -> Value {
        match proj_mult.reduce(res_mult.map(|v| v.into_pvalue()), |x, y| x * y) {
            Some(PValue::ONE) | None => raw,
            Some(mult) => Self::raw_to_mult_change(raw).mul_add(mult.into_value(), Value::ONE),
        }
    }
    fn apply_raw(base: Value, raw: Value) -> Value {
        base * raw
    }
    fn raw_to_mult(raw: Value) -> Value {
        raw
    }
    fn raw_to_mult_change(raw: Value) -> Value {
        raw - Value::ONE
    }
    fn mult_to_raw(mult: Value) -> Value {
        mult
    }
}

pub(super) struct MultMathDiv;
impl MultMath for MultMathDiv {
    fn check_raw(raw: Value) -> bool {
        raw != Value::ZERO
    }
    fn diminish_raw(raw: Value, proj_mult: Option<PValue>, res_mult: Option<UnitInterval>) -> Value {
        match proj_mult.reduce(res_mult.map(|v| v.into_pvalue()), |x, y| x * y) {
            Some(PValue::ONE) | None => raw,
            Some(mult) => Value::ONE / Self::raw_to_mult_change(raw).mul_add(mult.into_value(), Value::ONE),
        }
    }
    fn apply_raw(base: Value, raw: Value) -> Value {
        base / raw
    }
    fn raw_to_mult(raw: Value) -> Value {
        Value::ONE / raw
    }
    fn raw_to_mult_change(raw: Value) -> Value {
        Value::ONE / raw - Value::ONE
    }
    fn mult_to_raw(mult: Value) -> Value {
        Value::ONE / mult
    }
}

pub(super) struct MultMathPerc;
impl MultMath for MultMathPerc {
    fn check_raw(_raw: Value) -> bool {
        true
    }
    fn diminish_raw(raw: Value, proj_mult: Option<PValue>, res_mult: Option<UnitInterval>) -> Value {
        diminish_normal(raw, proj_mult, res_mult)
    }
    fn apply_raw(base: Value, raw: Value) -> Value {
        base * raw.perc_change_to_mult()
    }
    fn raw_to_mult(raw: Value) -> Value {
        raw.perc_change_to_mult()
    }
    fn raw_to_mult_change(raw: Value) -> Value {
        raw / Value::HUNDRED
    }
    fn mult_to_raw(mult: Value) -> Value {
        mult.mul_add(Value::HUNDRED, -Value::HUNDRED)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Misc
////////////////////////////////////////////////////////////////////////////////////////////////////
fn diminish_normal(raw: Value, proj_mult: Option<PValue>, res_mult: Option<UnitInterval>) -> Value {
    match proj_mult.reduce(res_mult.map(|v| v.into_pvalue()), |x, y| x * y) {
        Some(mult) => raw * mult,
        None => raw,
    }
}
