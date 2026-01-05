use crate::def::SERVER_TICK_HZ;

pub(crate) const FLOAT_TOLERANCE: f64 = 0.0000000001;

// Round number to a specified significant digit.
pub(crate) fn sig_round(val: f64, sig_digits: u32) -> f64 {
    if val == 0.0 {
        return val;
    }
    let highest_magnitude = val.abs().log10().floor() as i32;
    let digits = -highest_magnitude - 1 + sig_digits as i32;
    round(val, digits)
}

// Round number to a specific digit after decimal dot.
pub(crate) fn round(val: f64, digits: i32) -> f64 {
    let mul = 10.0_f64.powi(digits);
    (val * mul).round() / mul
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rounding with error "cancel" - should be used where float value is coerced into integer by
// rounding down, to avoid cases like 2.3 / 0.1 = 22.999999999999996 becoming 22 instead of 23
////////////////////////////////////////////////////////////////////////////////////////////////////
fn float_unerr(val: f64) -> f64 {
    round(val, 10)
}

pub(crate) fn round_unerr(val: f64) -> f64 {
    float_unerr(val).round()
}

pub(crate) fn floor_unerr(val: f64) -> f64 {
    float_unerr(val).floor()
}

pub(crate) fn ceil_unerr(val: f64) -> f64 {
    float_unerr(val).ceil()
}

pub(crate) fn trunc_unerr(val: f64) -> f64 {
    float_unerr(val).trunc()
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ticks
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) fn floor_tick(val: f64) -> f64 {
    (val * SERVER_TICK_HZ as f64).floor() / SERVER_TICK_HZ as f64
}

pub(crate) fn ceil_tick(val: f64) -> f64 {
    (val * SERVER_TICK_HZ as f64).ceil() / SERVER_TICK_HZ as f64
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Casts
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) fn trunc_f64_to_u32(value: f64) -> u32 {
    trunc_unerr(value.clamp(u32::MIN as f64, u32::MAX as f64)) as u32
}
pub(crate) fn round_f64_to_u32(value: f64) -> u32 {
    value.clamp(u32::MIN as f64, u32::MAX as f64).round() as u32
}
pub(crate) fn round_f64_to_i32(value: f64) -> i32 {
    value.clamp(i32::MIN as f64, i32::MAX as f64).round() as i32
}
