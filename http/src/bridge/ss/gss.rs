use std::sync::Arc;

use tokio::sync::{Mutex, MutexGuard, TryLockError};

use super::HSolarSystem;

#[derive(Clone)]
pub(crate) struct HGuardedSs {
    h_ss: Arc<Mutex<HSolarSystem>>,
}
impl HGuardedSs {
    pub(crate) fn new(id: String, core_ss: rc::SolarSystem) -> Self {
        Self {
            h_ss: Arc::new(Mutex::new(HSolarSystem::new(id, core_ss))),
        }
    }
    pub(crate) fn try_lock(&self) -> Result<MutexGuard<HSolarSystem>, TryLockError> {
        self.h_ss.try_lock()
    }
    pub(crate) async fn lock(&self) -> MutexGuard<HSolarSystem> {
        self.h_ss.lock().await
    }
}
