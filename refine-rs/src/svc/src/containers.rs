use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::inner::SrcInnerGuarded;
use crate::{
    src::SrcAlias,
    util::{RMap, RSet},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Alias data
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) struct SrcAliasDataGuarded {
    inner: RwLock<SrcAliasData>,
}
impl SrcAliasDataGuarded {
    pub(crate) fn new() -> Self {
        Self {
            inner: RwLock::new(SrcAliasData::new()),
        }
    }
    pub(crate) async fn read(&self) -> RwLockReadGuard<'_, SrcAliasData> {
        self.inner.read().await
    }
    pub(crate) async fn write(&self) -> RwLockWriteGuard<'_, SrcAliasData> {
        self.inner.write().await
    }
}

pub(crate) struct SrcAliasData {
    pub(crate) map: RMap<SrcAlias, SrcInnerGuarded>,
    pub(crate) default: Option<SrcInnerGuarded>,
}
impl SrcAliasData {
    fn new() -> Self {
        Self {
            map: RMap::new(),
            default: None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Locked aliases
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) struct SrcAliasLocksGuarded {
    inner: RwLock<RSet<SrcAlias>>,
}
impl SrcAliasLocksGuarded {
    pub(crate) fn new() -> Self {
        Self {
            inner: RwLock::new(RSet::new()),
        }
    }
    pub(crate) async fn read(&self) -> RwLockReadGuard<'_, RSet<SrcAlias>> {
        self.inner.read().await
    }
    pub(crate) async fn write(&self) -> RwLockWriteGuard<'_, RSet<SrcAlias>> {
        self.inner.write().await
    }
}
