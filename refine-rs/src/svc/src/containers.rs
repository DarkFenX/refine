use std::collections::{HashMap, HashSet};

use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::inner::SrcInnerGuarded;
use crate::src::SrcAlias;

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
    pub(crate) map: HashMap<SrcAlias, SrcInnerGuarded>,
    pub(crate) default: Option<SrcInnerGuarded>,
}
impl SrcAliasData {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            default: None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Locked aliases
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) struct SrcAliasLocksGuarded {
    inner: RwLock<HashSet<SrcAlias>>,
}
impl SrcAliasLocksGuarded {
    pub(crate) fn new() -> Self {
        Self {
            inner: RwLock::new(HashSet::new()),
        }
    }
    pub(crate) async fn read(&self) -> RwLockReadGuard<'_, HashSet<SrcAlias>> {
        self.inner.read().await
    }
    pub(crate) async fn write(&self) -> RwLockWriteGuard<'_, HashSet<SrcAlias>> {
        self.inner.write().await
    }
}
