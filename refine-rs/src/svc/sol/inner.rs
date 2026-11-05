use std::{sync::Arc, time::Instant};

use crate::{SolId, SrcAlias};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inner guarded
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(crate) struct SolInnerGuarded {
    inner: Arc<tokio::sync::Mutex<SolInner>>,
    // Source alias is shared with inner sol, and is available without waiting for requests which
    // hold the inner sol lock
    src_alias: Arc<parking_lot::Mutex<SrcAlias>>,
}
impl SolInnerGuarded {
    pub(crate) fn new(id: SolId, src_alias: SrcAlias, core: rc::Core) -> Self {
        let src_alias = Arc::new(parking_lot::Mutex::new(src_alias));
        Self {
            inner: Arc::new(tokio::sync::Mutex::new(SolInner::new(id, src_alias.clone(), core))),
            src_alias,
        }
    }
    pub(crate) fn get_src_alias(&self) -> SrcAlias {
        *self.src_alias.lock()
    }
    pub(in crate::svc) fn try_lock(&self) -> Result<tokio::sync::MutexGuard<'_, SolInner>, tokio::sync::TryLockError> {
        self.inner.try_lock()
    }
    /// Like regular lock, but updates timestamp on inner sol during drop if requested
    pub(crate) async fn into_lock_owned(self, touch: bool) -> SolOwnedMutexGuard {
        SolOwnedMutexGuard {
            guard: self.inner.lock_owned().await,
            touch,
        }
    }
}

pub(crate) struct SolOwnedMutexGuard {
    guard: tokio::sync::OwnedMutexGuard<SolInner>,
    touch: bool,
}
impl Drop for SolOwnedMutexGuard {
    fn drop(&mut self) {
        if self.touch {
            self.guard.touch();
        }
    }
}
impl std::ops::Deref for SolOwnedMutexGuard {
    type Target = SolInner;
    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}
impl std::ops::DerefMut for SolOwnedMutexGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inner unguarded
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) struct SolInner {
    id: SolId,
    src_alias: Arc<parking_lot::Mutex<SrcAlias>>,
    last_accessed: Instant,
    // Set when the solar system is removed from the map, so that requests which were waiting for
    // its lock do not work on it
    closed: bool,
    pub(super) core: SolCoreGuarded,
}
impl SolInner {
    fn new(id: SolId, src_alias: Arc<parking_lot::Mutex<SrcAlias>>, core: rc::Core) -> Self {
        Self {
            id,
            src_alias,
            last_accessed: Instant::now(),
            closed: false,
            core: SolCoreGuarded::new(core),
        }
    }
    pub(crate) fn get_id(&self) -> SolId {
        self.id
    }
    pub(crate) fn get_src_alias(&self) -> SrcAlias {
        *self.src_alias.lock()
    }
    pub(crate) fn set_src_alias(&mut self, src_alias: SrcAlias) {
        *self.src_alias.lock() = src_alias
    }
    fn touch(&mut self) {
        self.last_accessed = Instant::now();
    }
    pub(in crate::svc) fn get_last_accessed(&self) -> Instant {
        self.last_accessed
    }
    pub(super) fn close(&mut self) {
        self.closed = true;
    }
    pub(crate) fn is_closed(&self) -> bool {
        self.closed
    }
    pub(crate) fn is_poisoned(&self) -> bool {
        self.core.0.is_poisoned()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Core guarded
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(super) struct SolCoreGuarded(Arc<std::sync::Mutex<rc::Core>>);
impl SolCoreGuarded {
    fn new(core: rc::Core) -> Self {
        Self(Arc::new(std::sync::Mutex::new(core)))
    }
    pub(super) fn lock(&self) -> std::sync::MutexGuard<'_, rc::Core> {
        self.0.lock().unwrap()
    }
}
