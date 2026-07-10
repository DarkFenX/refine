use crate::num::{Count, PValue};

pub(super) fn duration_to_ticks_floor(duration: PValue) -> Count {
    Count::from_pvalue_trunced(duration * PValue::SERVER_TICK_HZ)
}

pub(super) fn duration_to_ticks_ceil(duration: PValue) -> Count {
    Count::from_pvalue_ceiled(duration * PValue::SERVER_TICK_HZ)
}

pub(super) fn ticks_to_duration(ticks: Count) -> PValue {
    ticks.into_pvalue() * PValue::SERVER_TICK_S
}
