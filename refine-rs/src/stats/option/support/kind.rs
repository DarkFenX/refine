pub(in crate::stats) use repr::StatOptionRepr;

use super::containers::{StatDefOption, StatDefOptionExt};

// Needed to make containers with per-entity options to be usable for storing them "raw" and
// resolved form. Raw form is the format which is stored in public-facing entities, which have
// "default" field defined elsewhere. Resolved form is default value + stored option combined.
//
// Technically, it is necessary only for the extended option, but since we have it already, it is
// good to use for the regular option as well.
pub(in crate::stats) trait StatOptionKind {
    type Reg: StatOptionRepr;
    type Ext<T>: StatOptionRepr
    where
        T: StatOptionRepr;
}

#[derive(Copy, Clone)]
pub(in crate::stats) struct StatOptionRaw;
impl StatOptionKind for StatOptionRaw {
    type Reg = StatDefOption;
    type Ext<T>
        = StatDefOptionExt<T>
    where
        T: StatOptionRepr;
}

#[derive(Copy, Clone)]
pub(in crate::stats) struct StatOptionResolved;
impl StatOptionKind for StatOptionResolved {
    type Reg = bool;
    type Ext<T>
        = Option<Vec<T>>
    where
        T: StatOptionRepr;
}

// Another trait is needed to remove lots of bounds from specific structs
#[cfg(feature = "serde")]
mod repr {
    pub(in crate::stats) trait StatOptionRepr: Clone + Default + serde::de::DeserializeOwned {}
    impl<T> StatOptionRepr for T where T: Clone + Default + serde::de::DeserializeOwned {}
}

#[cfg(not(feature = "serde"))]
mod repr {
    pub(in crate::stats) trait StatOptionRepr: Clone + Default {}
    impl<T> StatOptionRepr for T where T: Clone + Default {}
}
