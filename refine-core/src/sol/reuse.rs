use crate::{svc::cycle::CseqCache, ud::UEffectUpdates};

/// Containers reused across operations on a solar system, to avoid reallocating them.
pub(crate) struct SolReuse {
    /// Used when modifying an item change can change its running effects.
    ///
    /// Since it's filled by one call and read by another, have to be very careful to trace how it
    /// is used. There should be no writes between writing call and final reader call. This could be
    /// enforced by the compiler, but any way I came up with is not performance-free; so, the way to
    /// manage is just discipline.
    pub(crate) eupdates: UEffectUpdates,
    /// Secondary effect updates container, for updates which have to be processed while the primary
    /// one still holds data which is going to be read.
    pub(crate) eupdates_secondary: UEffectUpdates,
    /// Keeps calculated cycle sequences, since they are reusable for some getters (mostly stats)
    pub(crate) cseq_cache: CseqCache,
    /// Cycle sequences of stats which switch item states, valid only while they are switched.
    pub(crate) cseq_cache_switched: CseqCache,
}
impl SolReuse {
    pub(crate) fn new() -> Self {
        Self {
            eupdates: UEffectUpdates::new(),
            eupdates_secondary: UEffectUpdates::new(),
            cseq_cache: CseqCache::new(),
            cseq_cache_switched: CseqCache::new(),
        }
    }
}
// There is no need to actually clone contents of the cache
impl Clone for SolReuse {
    fn clone(&self) -> Self {
        Self::new()
    }
}
