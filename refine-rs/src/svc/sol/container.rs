use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::inner::SolInnerGuarded;
use crate::{SolarSystemId, util::RMap};

pub(crate) struct SolMapGuarded {
    inner: RwLock<RMap<SolarSystemId, SolInnerGuarded>>,
}
impl SolMapGuarded {
    pub(crate) fn new() -> Self {
        Self {
            inner: RwLock::new(RMap::new()),
        }
    }
    pub(crate) async fn read(&self) -> RwLockReadGuard<'_, RMap<SolarSystemId, SolInnerGuarded>> {
        self.inner.read().await
    }
    pub(crate) async fn write(&self) -> RwLockWriteGuard<'_, RMap<SolarSystemId, SolInnerGuarded>> {
        self.inner.write().await
    }
}
