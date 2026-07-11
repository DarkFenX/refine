use std::sync::Arc;

use tokio::sync::{Mutex, MutexGuard, TryLockError};

use super::sol_inner::SolarSystemInner;

#[derive(Clone)]
pub(crate) struct SolarSystem {
    inner: Arc<Mutex<SolarSystemInner>>,
}
impl SolarSystem {
    pub(crate) fn new(core_sol: Box<rc::SolarSystem>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(SolarSystemInner::new(core_sol))),
        }
    }
    pub(crate) fn try_lock(&self) -> Result<MutexGuard<'_, SolarSystemInner>, TryLockError> {
        self.inner.try_lock()
    }
    pub(crate) async fn lock(&self) -> MutexGuard<'_, SolarSystemInner> {
        self.inner.lock().await
    }
}
