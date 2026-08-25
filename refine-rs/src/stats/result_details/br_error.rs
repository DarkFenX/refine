use crate::err::BrResolveError;

/// Wraps per-option error for stats which can fail during backref resolution.
#[derive(Clone, Debug, thiserror::Error)]
pub enum StatBrFallibleError<E> {
    #[error(transparent)]
    Stat(E),
    #[error(transparent)]
    BrResolve(BrResolveError),
}
