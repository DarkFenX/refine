use std::sync::Arc;

use tokio::sync::{Mutex, MutexGuard, TryLockError};

use crate::{refine::Refine, sol::SolarSystemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct SolarSystem<'r> {
    pub(super) refine: &'r Refine,
    pub(super) id: SolarSystemId,
    inner: SolarSystemInnerGuarded,
}
impl<'r> SolarSystem<'r> {
    pub fn get_id(&self) -> SolarSystemId {
        self.id
    }
}
// Private part
impl<'r> SolarSystem<'r> {
    pub(super) fn new(refine: &'r Refine, id: SolarSystemId, inner: SolarSystemInnerGuarded) -> Self {
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
    pub(super) async fn lock_touch(&self) -> SolInnerTouchingMutexGuard<'_> {
        SolInnerTouchingMutexGuard {
            guard: self.0.lock().await,
        }
    }
}

struct SolInnerTouchingMutexGuard<'m> {
    guard: MutexGuard<'m, SolarSystemInner>,
}
impl<'m> Drop for SolInnerTouchingMutexGuard<'m> {
    fn drop(&mut self) {
        self.guard.touch();
    }
}
impl<'m> std::ops::Deref for SolInnerTouchingMutexGuard<'m> {
    type Target = SolarSystemInner;
    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}
impl<'m> std::ops::DerefMut for SolInnerTouchingMutexGuard<'m> {
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
    fn touch(&mut self) {
        self.accessed = chrono::Utc::now();
    }
}
