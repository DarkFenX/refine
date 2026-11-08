use std::{mem::transmute, pat::pattern_type};

use crate::util::round_f64_to_i32;

const MIN_RAW: i32 = -0x7fff_ffff;
const MAX_RAW: i32 = 0x7fff_ffff;
type Range = pattern_type!(i32 is MIN_RAW..=MAX_RAW);

/// Number of the slot an item occupies, as defined by EVE data.
#[derive(Copy, Clone)]
pub struct SlotIndex(Range);
impl SlotIndex {
    // The lowest value is a niche, so it is clamped
    pub const fn from_i32(value: i32) -> Self {
        // SAFETY: clamping leaves the value in range
        Self(unsafe { transmute::<i32, Range>(value.max(MIN_RAW)) })
    }
    pub const fn into_i32(self) -> i32 {
        // SAFETY: any value in range can be represented by base type
        unsafe { transmute::<Range, i32>(self.0) }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SlotIndex {
    pub(crate) fn from_f64_rounded(value: f64) -> Self {
        Self::from_i32(round_f64_to_i32(value))
    }
}
impl From<SlotIndex> for i32 {
    fn from(v: SlotIndex) -> Self {
        v.into_i32()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ordering/equality/hash - pattern types do not provide those
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Eq for SlotIndex {}
impl PartialEq for SlotIndex {
    fn eq(&self, other: &Self) -> bool {
        self.into_i32() == other.into_i32()
    }
}
impl std::marker::StructuralPartialEq for SlotIndex {}

// Pattern types have no Hash to derive
impl std::hash::Hash for SlotIndex {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.into_i32().hash(state);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
impl std::fmt::Display for SlotIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.into_i32(), f)
    }
}

#[cfg(feature = "serde")]
mod custom_serde {
    use serde::ser::{Serialize, Serializer};

    use super::*;

    impl Serialize for SlotIndex {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_i32(self.into_i32())
        }
    }
}
