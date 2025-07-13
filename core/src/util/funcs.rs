use ordered_float::Float;

use crate::def::OF;

pub(crate) fn vec_push_opt<T>(vec: &mut Vec<T>, opt: Option<T>) {
    if let Some(v) = opt {
        vec.push(v);
    };
}

pub(crate) fn sig_round(val: OF<f64>, sig_digits: u32) -> OF<f64> {
    if val == OF(0.0) {
        return val;
    }
    let highest_magnitude = val.abs().log10().floor().into_inner() as i32;
    let digits = -highest_magnitude - 1 + sig_digits as i32;
    round(val, digits)
}

pub(crate) fn round(val: OF<f64>, digits: i32) -> OF<f64> {
    let mul = OF(10.0).powi(digits);
    (val * mul).round() / mul
}

// Should be used where float value is coerced into integer by rounding down, to avoid cases like
// 2.3 / 0.1 = 22.999999999999996 becoming 22 instead of 23
fn float_unerr(val: OF<f64>) -> OF<f64> {
    round(val, 10)
}

pub(crate) fn floor_unerr(val: OF<f64>) -> f64 {
    float_unerr(val).floor().into_inner()
}

pub(crate) fn ceil_unerr(val: OF<f64>) -> f64 {
    float_unerr(val).ceil().into_inner()
}

pub(crate) fn trunc_unerr(val: OF<f64>) -> f64 {
    float_unerr(val).trunc().into_inner()
}
