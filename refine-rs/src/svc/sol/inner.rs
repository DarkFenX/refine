use std::{sync::Arc, time::Instant};

use crate::{SolarSystemId, src::SrcAlias};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inner guarded
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(crate) struct SolInnerGuarded(Arc<tokio::sync::Mutex<SolInner>>);
impl SolInnerGuarded {
    pub(crate) fn new(id: SolarSystemId, src_alias: SrcAlias, core_sol: rc::SolarSystem) -> Self {
        Self(Arc::new(tokio::sync::Mutex::new(SolInner::new(
            id, src_alias, core_sol,
        ))))
    }
    pub(in crate::svc) fn try_lock(&self) -> Result<tokio::sync::MutexGuard<'_, SolInner>, tokio::sync::TryLockError> {
        self.0.try_lock()
    }
    /// Like regular lock, but updates timestamp on inner sol during drop
    pub(crate) async fn into_lock_touch_owned(self) -> SolOwnedMutexGuard {
        SolOwnedMutexGuard {
            guard: self.0.lock_owned().await,
        }
    }
}

pub(crate) struct SolOwnedMutexGuard {
    guard: tokio::sync::OwnedMutexGuard<SolInner>,
}
impl Drop for SolOwnedMutexGuard {
    fn drop(&mut self) {
        self.guard.touch();
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
    id: SolarSystemId,
    src_alias: SrcAlias,
    last_accessed: Instant,
    pub(super) core_sol: SolCoreGuarded,
}
impl SolInner {
    fn new(id: SolarSystemId, src_alias: SrcAlias, core_sol: rc::SolarSystem) -> Self {
        Self {
            id,
            src_alias,
            last_accessed: Instant::now(),
            core_sol: SolCoreGuarded::new(core_sol),
        }
    }
    pub(crate) fn get_id(&self) -> SolarSystemId {
        self.id
    }
    pub(crate) fn get_src_alias(&self) -> SrcAlias {
        self.src_alias
    }
    pub(crate) fn set_src_alias(&mut self, src_alias: SrcAlias) {
        self.src_alias = src_alias
    }
    pub(in crate::svc) fn get_last_accessed(&self) -> Instant {
        self.last_accessed
    }
    fn touch(&mut self) {
        self.last_accessed = Instant::now();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Core guarded
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(super) struct SolCoreGuarded(Arc<parking_lot::Mutex<rc::SolarSystem>>);
impl SolCoreGuarded {
    fn new(core_sol: rc::SolarSystem) -> Self {
        Self(Arc::new(parking_lot::Mutex::new(core_sol)))
    }
    pub(super) fn lock(&self) -> parking_lot::MutexGuard<'_, rc::SolarSystem> {
        self.0.lock()
    }
}
