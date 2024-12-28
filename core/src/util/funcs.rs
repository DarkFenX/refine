pub(crate) fn vec_push_opt<T>(vec: &mut Vec<T>, opt: Option<T>) {
    if let Some(v) = opt {
        vec.push(v);
    };
}

pub(crate) fn sig_round(val: f64, sig_digits: u32) -> f64 {
    let highest_magnitude = val.abs().log10().floor() as i32;
    let digits = -highest_magnitude - 1 + sig_digits as i32;
    round(val, digits)
}

pub(crate) fn round(val: f64, digits: i32) -> f64 {
    let mul = f64::powi(10.0, digits);
    (val * mul).round() / mul
}
