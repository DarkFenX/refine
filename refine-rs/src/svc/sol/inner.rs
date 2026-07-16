use std::sync::Arc;

use tokio::sync::{Mutex, MutexGuard, OwnedMutexGuard, TryLockError};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Guarded
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(crate) struct SolarSystemInnerGuarded(Arc<Mutex<SolarSystemInner>>);
impl SolarSystemInnerGuarded {
    pub(crate) fn new(core_sol: rc::SolarSystem) -> Self {
        Self(Arc::new(Mutex::new(SolarSystemInner::new(core_sol))))
    }
    pub(in crate::svc) fn try_lock(&self) -> Result<MutexGuard<'_, SolarSystemInner>, TryLockError> {
        self.0.try_lock()
    }
    // Like regular lock, but updates timestamp on inner sol during drop
    pub(crate) async fn into_lock_touch_owned(self) -> SolOwnedMutexGuard {
        SolOwnedMutexGuard {
            guard: self.0.lock_owned().await,
        }
    }
}

pub(crate) struct SolOwnedMutexGuard {
    guard: OwnedMutexGuard<SolarSystemInner>,
}
impl Drop for SolOwnedMutexGuard {
    fn drop(&mut self) {
        self.guard.touch();
    }
}
impl std::ops::Deref for SolOwnedMutexGuard {
    type Target = SolarSystemInner;
    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}
impl<'m> std::ops::DerefMut for SolOwnedMutexGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Unguarded
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) struct SolarSystemInner {
    last_accessed: chrono::DateTime<chrono::Utc>,
    pub(super) core_sol: Option<Box<rc::SolarSystem>>,
}
impl SolarSystemInner {
    fn new(core_sol: rc::SolarSystem) -> Self {
        Self {
            last_accessed: chrono::Utc::now(),
            core_sol: Some(Box::new(core_sol)),
        }
    }
    pub(in crate::svc) fn get_last_accessed(&self) -> chrono::DateTime<chrono::Utc> {
        self.last_accessed
    }
    fn touch(&mut self) {
        self.last_accessed = chrono::Utc::now();
    }
}
