use std::{mem::transmute, pat::pattern_type};

const MIN_RAW: u32 = 0x0000_0000;
const MAX_RAW: u32 = 0xffff_fffe;
type Range = pattern_type!(u32 is MIN_RAW..=MAX_RAW);

/// Integer which keeps max value as a niche, and stores all the others as they are.
#[derive(Copy, Clone)]
pub(crate) struct U32NonMax(Range);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////
impl U32NonMax {
    // SAFETY: 0 is in range
    pub(crate) const ZERO: Self = Self(unsafe { transmute::<u32, Range>(0) });
    pub(crate) const NICHE: u32 = u32::MAX;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl U32NonMax {
    pub(crate) fn from_usize(value: usize) -> Self {
        let value: u32 = value.try_into().unwrap();
        assert!(value != Self::NICHE);
        // SAFETY: just checked that the value is in range
        Self(unsafe { transmute::<u32, Range>(value) })
    }
    pub(crate) fn into_u32(self) -> u32 {
        // SAFETY: any value in range can be represented by base type
        unsafe { transmute::<Range, u32>(self.0) }
    }
    pub(crate) fn into_usize(self) -> usize {
        self.into_u32() as usize
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ordering/equality/hash - pattern types do not provide those
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Eq for U32NonMax {}
impl PartialEq for U32NonMax {
    fn eq(&self, other: &Self) -> bool {
        self.into_u32() == other.into_u32()
    }
}

impl Ord for U32NonMax {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.into_u32().cmp(&other.into_u32())
    }
}
impl PartialOrd for U32NonMax {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::hash::Hash for U32NonMax {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.into_u32().hash(state);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
impl std::fmt::Debug for U32NonMax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.into_u32(), f)
    }
}
