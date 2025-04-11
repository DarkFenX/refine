use std::sync::Arc;

use tokio::sync::{Mutex, MutexGuard, TryLockError};

use super::HSolarSystemInner;

#[derive(Clone)]
pub(crate) struct HSolarSystem {
    inner: Arc<Mutex<HSolarSystemInner>>,
}
impl HSolarSystem {
    pub(crate) fn new(id: String, core_sol: rc::SolarSystem) -> Self {
        Self {
            inner: Arc::new(Mutex::new(HSolarSystemInner::new(id, core_sol))),
        }
    }
    pub(crate) fn try_lock(&self) -> Result<MutexGuard<HSolarSystemInner>, TryLockError> {
        self.inner.try_lock()
    }
    pub(crate) async fn lock(&self) -> MutexGuard<HSolarSystemInner> {
        self.inner.lock().await
    }
}
