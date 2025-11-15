use ordered_float::Float;

use crate::def::{AttrVal, OF};

pub(in crate::svc::vast) fn calc_regen(c_max: AttrVal, c_rech: AttrVal, cap_perc: AttrVal) -> AttrVal {
    let result = OF(10.0) * c_max / c_rech * (cap_perc.sqrt() - cap_perc);
    match result.is_finite() {
        true => result,
        false => OF(0.0),
    }
}

pub(in crate::svc::vast) fn regenerate(c0: AttrVal, c_max: AttrVal, tau: AttrVal, t0: AttrVal, t1: AttrVal) -> AttrVal {
    (OF(1.0) + ((c0 / c_max).sqrt() - OF(1.0)) * ((t0 - t1) / tau).exp()).powi(2) * c_max
}
