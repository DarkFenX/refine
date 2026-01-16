use crate::num::{PValue, UnitInterval, Value};

pub(in crate::svc::vast::stats) fn calc_regen(
    max: PValue,
    recharge_duration: PValue,
    cap_perc: UnitInterval,
) -> PValue {
    let cap_perc = cap_perc.into_pvalue();
    let result = PValue::TEN * max / recharge_duration * PValue::from_value_unchecked(cap_perc.sqrt() - cap_perc);
    match result.is_finite() {
        true => result,
        false => PValue::ZERO,
    }
}

pub(in crate::svc::vast::stats) fn regenerate(
    c0: PValue,
    c_max: PValue,
    tau: PValue,
    t0: PValue,
    t1: PValue,
) -> PValue {
    (Value::ONE + ((c0 / c_max).sqrt() - PValue::ONE) * ((t0 - t1) / tau).exp()).pow2() * c_max
}
