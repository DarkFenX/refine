use super::containers::StatDefOptionExt;

// Needed to make containers with per-entity options to be usable for storing them "raw" (which
// should be usable with conjunction of default value set elsewhere), and for storing their resolved
// version (combined with default value)
pub(in crate::stats) trait StatOptionExtKind {
    type Repr<T>: Clone
    where
        T: Clone;
}

pub(in crate::stats) struct StatOptionExtRaw;
impl StatOptionExtKind for StatOptionExtRaw {
    type Repr<T>
        = StatDefOptionExt<T>
    where
        T: Clone;
}

#[derive(Copy, Clone)]
pub(in crate::stats) struct StatOptionExtResolved;
impl StatOptionExtKind for StatOptionExtResolved {
    type Repr<T>
        = Option<Vec<T>>
    where
        T: Clone;
}
