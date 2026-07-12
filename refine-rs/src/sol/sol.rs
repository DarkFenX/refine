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
    async fn lock(&self) -> MutexGuard<'_, SolarSystemInner> {
        self.0.lock().await
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
