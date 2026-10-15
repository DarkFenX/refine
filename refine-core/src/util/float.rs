use crate::def::SERVER_TICK_HZ;

pub(crate) const FLOAT_TOLERANCE: f64 = 1e-10;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Math
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Round number to a specified significant digit.
pub(crate) fn sig_round(val: f64, sig_digits: u32) -> f64 {
    if val == 0.0 {
        return val;
    }
    let highest_magnitude = get_magnitude(val.abs());
    let digits = -highest_magnitude - 1 + sig_digits as i32;
    round(val, digits)
}

/// Round number to a specific digit after decimal dot.
pub(crate) fn round(val: f64, digits: i32) -> f64 {
    let mul = pow10(digits);
    (val * mul).round() / mul
}

// Precalculated powers of 10
const POW10_MIN: i32 = -22;
const POW10_MAX: i32 = 22;
const POW10: [f64; (POW10_MAX - POW10_MIN + 1) as usize] = {
    let mut table = [0.0; (POW10_MAX - POW10_MIN + 1) as usize];
    let mut pos_val = 1.0;
    let mut exp = 0;
    while exp <= POW10_MAX {
        table[(exp - POW10_MIN) as usize] = pos_val;
        table[(-exp - POW10_MIN) as usize] = 1.0 / pos_val;
        pos_val *= 10.0;
        exp += 1;
    }
    table
};

fn pow10(exp: i32) -> f64 {
    match (POW10_MIN..=POW10_MAX).contains(&exp) {
        true => POW10[(exp - POW10_MIN) as usize],
        false => 10.0_f64.powi(exp),
    }
}

fn get_magnitude(abs_val: f64) -> i32 {
    // Estimate via binary exponent, which can be lower than actual value by up to 2, or higher by 1
    let bin_exp = ((abs_val.to_bits() >> 52) & 0x7ff) as i32 - 1023;
    let mut magnitude = (bin_exp * 1233) >> 12;
    if magnitude <= POW10_MIN || magnitude >= POW10_MAX - 2 {
        return abs_val.log10().floor() as i32;
    }
    if abs_val < POW10[(magnitude - POW10_MIN) as usize] {
        magnitude -= 1;
    }
    while abs_val >= POW10[(magnitude + 1 - POW10_MIN) as usize] {
        magnitude += 1;
    }
    magnitude
}

/// Sum with positive zero as additive identity, to avoid having -0.0 exposed in e.g. stats
pub(crate) fn sum_pai_owned<I>(iter: I) -> f64
where
    I: Iterator<Item = f64>,
{
    iter.fold(0.0, |a, b| a + b)
}
pub(crate) fn sum_pai_ref<'a, I>(iter: I) -> f64
where
    I: Iterator<Item = &'a f64>,
{
    iter.fold(0.0, |a, b| a + b)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rounding with error "cancel" - should be used where float value is coerced into integer by
// rounding down, to avoid cases like 2.3 / 0.1 = 22.999999999999996 becoming 22 instead of 23
////////////////////////////////////////////////////////////////////////////////////////////////////
fn float_unerr(val: f64) -> f64 {
    round(val, 10)
}

pub(crate) fn floor_unerr(val: f64) -> f64 {
    float_unerr(val).floor()
}

pub(crate) fn ceil_unerr(val: f64) -> f64 {
    float_unerr(val).ceil()
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
// Explicit truncation (when it's the desired behavior) and clamping are not needed, float-to-int
// "as" is a saturating cast, and performs rounding towards 0, see link below for more info:
// https://doc.rust-lang.org/reference/expressions/operator-expr.html#r-expr.as.numeric.float-as-int
pub(crate) fn round_f64_to_u32(value: f64) -> u32 {
    value.round() as u32
}
pub(crate) fn trunc_f64_to_u32(value: f64) -> u32 {
    float_unerr(value) as u32
}
pub(crate) fn ceil_f64_to_u32(value: f64) -> u32 {
    ceil_unerr(value) as u32
}
pub(crate) fn round_f64_to_i32(value: f64) -> i32 {
    value.round() as i32
}
