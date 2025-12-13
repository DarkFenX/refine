use ordered_float::Float;

use crate::def::{OF, SERVER_TICK_HZ};

pub(crate) const FLOAT_TOLERANCE: OF<f64> = OF(0.0000000001);

/// Round number to a specified significant digit.
pub(crate) fn sig_round(val: OF<f64>, sig_digits: u32) -> OF<f64> {
    if val == OF(0.0) {
        return val;
    }
    let highest_magnitude = val.abs().log10().floor().into_inner() as i32;
    let digits = -highest_magnitude - 1 + sig_digits as i32;
    round(val, digits)
}

/// Round number to a specific digit after decimal dot.
pub(crate) fn round(val: OF<f64>, digits: i32) -> OF<f64> {
    let mul = OF(10.0).powi(digits);
    (val * mul).round() / mul
}

// Should be used where float value is coerced into integer by rounding down, to avoid cases like
// 2.3 / 0.1 = 22.999999999999996 becoming 22 instead of 23
fn float_unerr(val: OF<f64>) -> OF<f64> {
    round(val, 10)
}

pub(crate) fn round_unerr(val: OF<f64>) -> OF<f64> {
    float_unerr(val).round()
}

pub(crate) fn floor_unerr(val: OF<f64>) -> OF<f64> {
    float_unerr(val).floor()
}

pub(crate) fn ceil_unerr(val: OF<f64>) -> OF<f64> {
    float_unerr(val).ceil()
}

pub(crate) fn trunc_unerr(val: OF<f64>) -> OF<f64> {
    float_unerr(val).trunc()
}

pub(crate) fn floor_tick(val: OF<f64>) -> OF<f64> {
    (val * SERVER_TICK_HZ as f64).floor() / SERVER_TICK_HZ as f64
}

pub(crate) fn ceil_tick(val: OF<f64>) -> OF<f64> {
    (val * SERVER_TICK_HZ as f64).ceil() / SERVER_TICK_HZ as f64
}
