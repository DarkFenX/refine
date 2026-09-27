use crate::ud::UEffectUpdates;

/// Containers reused across operations on a solar system, to avoid reallocating them.
pub(crate) struct SolCache {
    /// Used when modifying an item change can change its running effects.
    ///
    /// Since it's filled by one call and read by another, have to be very careful to trace how it
    /// is used. There should be no writes between writing call and final reader call. This could be
    /// enforced by the compiler, but any way I came up with is not performance-free; so, the way to
    /// manage is just discipline.
    pub(crate) eupdates: UEffectUpdates,
}
impl SolCache {
    pub(crate) fn new() -> Self {
        Self {
            eupdates: UEffectUpdates::new(),
        }
    }
}
impl Clone for SolCache {
    fn clone(&self) -> Self {
        Self::new()
    }
}
