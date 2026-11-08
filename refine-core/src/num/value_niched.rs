use std::{mem::transmute, pat::pattern_type};

use crate::Value;

// Only one kind of NaN is excluded
const MIN_BITS: u64 = 0x0000_0000_0000_0000;
const MAX_BITS: u64 = 0xffff_ffff_ffff_fffe;
type Range = pattern_type!(u64 is MIN_BITS..=MAX_BITS);

/// Value with a niche.
///
/// Useful when a type containing it needs extra space for a niche, like value wrapped into an
/// option. It does not come for free, but sometimes can be worth (e.g. when this value is rarely
/// used, but often moved around).
#[derive(Copy, Clone)]
pub(crate) struct ValueNiched(Range);
impl ValueNiched {
    pub(crate) const fn from_value(value: Value) -> Self {
        // All bits set is a NaN which arithmetic does not produce
        let bits = match value.into_f64().to_bits() {
            bits @ MIN_BITS..=MAX_BITS => bits,
            _ => MIN_BITS,
        };
        // SAFETY: just excluded the only value out of range
        Self(unsafe { transmute::<u64, Range>(bits) })
    }
    pub(crate) const fn into_value(self) -> Value {
        // SAFETY: any value in range can be represented by base type
        let bits = unsafe { transmute::<Range, u64>(self.0) };
        Value::from_f64_unchecked(f64::from_bits(bits))
    }
}
