use std::sync::Arc;

use tokio::sync::{Mutex, MutexGuard, TryLockError};

use crate::{refine::Refine, sol::SolarSystemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct SolarSystem<'a> {
    pub(super) refine: &'a mut Refine,
    pub(super) id: SolarSystemId,
    inner: SolarSystemInnerGuarded,
}
impl<'a> SolarSystem<'a> {
    pub fn get_id(&self) -> SolarSystemId {
        self.id
    }
}
// Private part
impl<'a> SolarSystem<'a> {
    pub(super) fn new(refine: &'a mut Refine, id: SolarSystemId, inner: SolarSystemInnerGuarded) -> Self {
        Self { refine, id, inner }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inner guarded
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(crate) struct SolarSystemInnerGuarded(Arc<Mutex<SolarSystemInner>>);
impl SolarSystemInnerGuarded {
    pub(super) fn new(core_sol: rc::SolarSystem) -> Self {
        Self(Arc::new(Mutex::new(SolarSystemInner::new(core_sol))))
    }
    fn try_lock(&self) -> Result<MutexGuard<'_, SolarSystemInner>, TryLockError> {
        self.0.try_lock()
    }
    // Like regular lock, but updates timestamp on inner sol during drop
    async fn lock_touch(&self) -> TouchingMutexGuard<'_> {
        TouchingMutexGuard {
            guard: self.0.lock().await,
        }
    }
}

struct TouchingMutexGuard<'a> {
    guard: MutexGuard<'a, SolarSystemInner>,
}
impl<'a> Drop for TouchingMutexGuard<'a> {
    fn drop(&mut self) {
        self.guard.accessed = chrono::Utc::now();
    }
}
impl<'a> std::ops::Deref for TouchingMutexGuard<'a> {
    type Target = SolarSystemInner;
    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}
impl<'a> std::ops::DerefMut for TouchingMutexGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inner unguarded
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) struct SolarSystemInner {
    accessed: chrono::DateTime<chrono::Utc>,
    core_sol: Option<Box<rc::SolarSystem>>,
}
impl SolarSystemInner {
    fn new(core_sol: rc::SolarSystem) -> Self {
        Self {
            accessed: chrono::Utc::now(),
            core_sol: Some(Box::new(core_sol)),
        }
    }
    pub(super) fn touch(&mut self) {
        self.accessed = chrono::Utc::now();
    }
}
