use crate::{
    Refine, SolId, SrcAlias,
    svc::{SolInnerGuarded, SolOwnedMutexGuard},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Locked solar system.
///
/// A command whose future is dropped before it completes keeps the solar system locked until its
/// work is done. Neither this handle nor handles taken from it can be used after that, or after a
/// command panicked.
pub struct Sol<'r> {
    pub(crate) refine: &'r Refine,
    // Moves into the threadpool job running a command, and comes back with its result; missing
    // only after a command was canceled or panicked
    inner: Option<SolOwnedMutexGuard>,
}
impl<'r> Sol<'r> {
    pub fn get_id(&self) -> SolId {
        self.inner().get_id()
    }
    pub fn get_src_alias(&self) -> SrcAlias {
        self.inner().get_src_alias()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
const CANCELLED: &str = "solar system handle used after one of its commands was cancelled or panicked";

impl<'r> Sol<'r> {
    pub(super) async fn new(refine: &'r Refine, inner: SolInnerGuarded, touch: bool) -> Self {
        Self {
            refine,
            inner: Some(inner.into_lock_owned(touch).await),
        }
    }
    pub(crate) fn inner(&self) -> &SolOwnedMutexGuard {
        self.inner.as_ref().expect(CANCELLED)
    }
    pub(crate) fn inner_mut(&mut self) -> &mut SolOwnedMutexGuard {
        self.inner.as_mut().expect(CANCELLED)
    }
    pub(crate) fn take_inner(&mut self) -> SolOwnedMutexGuard {
        self.inner.take().expect(CANCELLED)
    }
    pub(crate) fn put_inner(&mut self, inner: SolOwnedMutexGuard) {
        self.inner = Some(inner);
    }
}
