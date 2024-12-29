use ordered_float::Float;

use crate::defs::{AttrVal, OF};

pub(crate) fn vec_push_opt<T>(vec: &mut Vec<T>, opt: Option<T>) {
    if let Some(v) = opt {
        vec.push(v);
    };
}

pub(crate) fn sig_round(val: AttrVal, sig_digits: u32) -> AttrVal {
    if val == OF(0.0) {
        return val;
    }
    let highest_magnitude = val.abs().log10().floor().into_inner() as i32;
    let digits = -highest_magnitude - 1 + sig_digits as i32;
    round(val, digits)
}

pub(crate) fn round(val: AttrVal, digits: i32) -> AttrVal {
    let mul = OF(10.0).powi(digits);
    (val * mul).round() / mul
}
