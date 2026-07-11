use std::sync::Arc;

use tokio::sync::{Mutex, MutexGuard, TryLockError};

use crate::{refine::Refine, sol::SolarSystemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct SolarSystem<'a> {
    pub(super) refine: &'a mut Refine,
    id: SolarSystemId,
    inner: Arc<Mutex<SolarSystemInner>>,
}
impl<'a> SolarSystem<'a> {
    pub fn get_id(&self) -> SolarSystemId {
        self.id
    }
}
// Private part
impl<'a> SolarSystem<'a> {
    pub(super) fn new(refine: &'a mut Refine, id: SolarSystemId, inner: Arc<Mutex<SolarSystemInner>>) -> Self {
        Self { refine, id, inner }
    }
    fn try_lock(&self) -> Result<MutexGuard<'_, SolarSystemInner>, TryLockError> {
        self.inner.try_lock()
    }
    async fn lock(&self) -> MutexGuard<'_, SolarSystemInner> {
        self.inner.lock().await
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inner
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) struct SolarSystemInner {
    accessed: chrono::DateTime<chrono::Utc>,
    core_sol: Option<Box<rc::SolarSystem>>,
}
impl SolarSystemInner {
    pub(super) fn new(core_sol: rc::SolarSystem) -> Self {
        Self {
            accessed: chrono::Utc::now(),
            core_sol: Some(Box::new(core_sol)),
        }
    }
}
