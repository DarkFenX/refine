use std::{collections::HashMap, sync::Arc};

use tokio::sync::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::sol::{SolarSystemId, SolarSystemInner};

pub(crate) struct SolMap {
    inner: RwLock<HashMap<SolarSystemId, Arc<Mutex<SolarSystemInner>>>>,
}
impl SolMap {
    pub(crate) fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
        }
    }
    pub(super) async fn read(&self) -> RwLockReadGuard<'_, HashMap<SolarSystemId, Arc<Mutex<SolarSystemInner>>>> {
        self.inner.read().await
    }
    pub(super) async fn write(&self) -> RwLockWriteGuard<'_, HashMap<SolarSystemId, Arc<Mutex<SolarSystemInner>>>> {
        self.inner.write().await
    }
}
