use std::collections::HashMap;

use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::inner::SolarSystemInnerGuarded;
use crate::SolarSystemId;

pub(crate) struct SolMapGuarded {
    inner: RwLock<HashMap<SolarSystemId, SolarSystemInnerGuarded>>,
}
impl SolMapGuarded {
    pub(crate) fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
        }
    }
    pub(crate) async fn read(&self) -> RwLockReadGuard<'_, HashMap<SolarSystemId, SolarSystemInnerGuarded>> {
        self.inner.read().await
    }
    pub(crate) async fn write(&self) -> RwLockWriteGuard<'_, HashMap<SolarSystemId, SolarSystemInnerGuarded>> {
        self.inner.write().await
    }
}
