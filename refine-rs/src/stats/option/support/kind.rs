use super::containers::{StatDefOption, StatDefOptionExt};

// Needed to make containers with per-entity options to be usable for storing them "raw" (which
// should be usable with conjunction of default value set elsewhere), and for storing their resolved
// version (combined with default value).
//
// Technically, it is necessary only for the extended option, but since we have it already, it is
// good to use for the regular option as well.
pub(in crate::stats) trait StatOptionKind {
    type Reg: Clone;
    type Ext<T>: Clone
    where
        T: Clone;
}

#[derive(Copy, Clone)]
pub(in crate::stats) struct StatOptionRaw;
impl StatOptionKind for StatOptionRaw {
    type Reg = StatDefOption;
    type Ext<T>
        = StatDefOptionExt<T>
    where
        T: Clone;
}

#[derive(Copy, Clone)]
pub(in crate::stats) struct StatOptionResolved;
impl StatOptionKind for StatOptionResolved {
    type Reg = bool;
    type Ext<T>
        = Option<Vec<T>>
    where
        T: Clone;
}
