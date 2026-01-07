use crate::misc::{PValue, UnitInterval, Value};

pub(in crate::svc::vast) fn calc_regen(c_max: PValue, c_rech: PValue, cap_perc: UnitInterval) -> PValue {
    let cap_perc = cap_perc.into_pvalue();
    let result = PValue::TEN * c_max / c_rech * PValue::from_value_unchecked(cap_perc.sqrt() - cap_perc);
    match result.is_finite() {
        true => result,
        false => PValue::ZERO,
    }
}

pub(in crate::svc::vast) fn regenerate(c0: PValue, c_max: PValue, tau: PValue, t0: PValue, t1: PValue) -> PValue {
    (Value::ONE + ((c0 / c_max).sqrt() - PValue::ONE) * ((t0 - t1) / tau).exp()).pow2() * c_max
}
