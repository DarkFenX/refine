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
    pub(crate) fn new(id: SolId, src_alias: SrcAlias, core_sol: rc::SolOwned) -> Self {
        let src_alias = Arc::new(parking_lot::Mutex::new(src_alias));
        Self {
            inner: Arc::new(tokio::sync::Mutex::new(SolInner::new(id, src_alias.clone(), core_sol))),
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
    // Set while a command runs on the core solar system, and left set if it panics, since its
    // changes can be partially applied
    poisoned: bool,
    // Taken out when the solar system is removed from the map, so that the remover frees it where
    // it chooses, and requests which were waiting for the lock see a closed solar system
    core_sol: Option<rc::SolOwned>,
}
impl SolInner {
    fn new(id: SolId, src_alias: Arc<parking_lot::Mutex<SrcAlias>>, core_sol: rc::SolOwned) -> Self {
        Self {
            id,
            src_alias,
            last_accessed: Instant::now(),
            poisoned: false,
            core_sol: Some(core_sol),
        }
    }
    pub(crate) fn get_id(&self) -> SolId {
        self.id
    }
    pub(crate) fn get_src_alias(&self) -> SrcAlias {
        *self.src_alias.lock()
    }
    pub(crate) fn get_guarded_src_alias(&self) -> Arc<parking_lot::Mutex<SrcAlias>> {
        self.src_alias.clone()
    }
    fn touch(&mut self) {
        self.last_accessed = Instant::now();
    }
    pub(in crate::svc) fn get_last_accessed(&self) -> Instant {
        self.last_accessed
    }
    pub(super) fn close(&mut self) -> Option<rc::SolOwned> {
        self.core_sol.take()
    }
    pub(crate) fn is_closed(&self) -> bool {
        self.core_sol.is_none()
    }
    pub(crate) fn is_poisoned(&self) -> bool {
        self.poisoned
    }
    pub(super) fn exec<R>(&mut self, func: impl FnOnce(&mut rc::SolOwned) -> R) -> R {
        // Reachable only by a caller which caught the panic and kept its handle
        assert!(!self.poisoned, "solar system poisoned by a panic");
        // Reachable only by a caller which kept its handle after removing the solar system
        let core_sol = self.core_sol.as_mut().expect("solar system used after removal");
        self.poisoned = true;
        let result = func(core_sol);
        self.poisoned = false;
        result
    }
}
