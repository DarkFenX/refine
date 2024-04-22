use std::sync::Arc;

use tokio::sync::{Mutex, MutexGuard, TryLockError};

use super::HSolarSystem;

#[derive(Clone)]
pub(crate) struct HGuardedSol {
    h_sol: Arc<Mutex<HSolarSystem>>,
}
impl HGuardedSol {
    pub(crate) fn new(id: String, core_sol: rc::SolarSystem) -> Self {
        Self {
            h_sol: Arc::new(Mutex::new(HSolarSystem::new(id, core_sol))),
        }
    }
    pub(crate) fn try_lock(&self) -> Result<MutexGuard<HSolarSystem>, TryLockError> {
        self.h_sol.try_lock()
    }
    pub(crate) async fn lock(&self) -> MutexGuard<HSolarSystem> {
        self.h_sol.lock().await
    }
}
