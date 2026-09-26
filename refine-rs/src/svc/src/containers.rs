use super::inner::SrcInnerGuarded;
use crate::{
    src::SrcAlias,
    util::{RMap, RSet},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Alias data
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) struct SrcAliasDataGuarded {
    inner: tokio::sync::RwLock<SrcAliasData>,
}
impl SrcAliasDataGuarded {
    pub(crate) fn new() -> Self {
        Self {
            inner: tokio::sync::RwLock::new(SrcAliasData::new()),
        }
    }
    pub(crate) async fn read(&self) -> tokio::sync::RwLockReadGuard<'_, SrcAliasData> {
        self.inner.read().await
    }
    pub(crate) async fn write(&self) -> tokio::sync::RwLockWriteGuard<'_, SrcAliasData> {
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
    inner: parking_lot::RwLock<RSet<SrcAlias>>,
}
impl SrcAliasLocksGuarded {
    pub(crate) fn new() -> Self {
        Self {
            inner: parking_lot::RwLock::new(RSet::new()),
        }
    }
    pub(crate) fn reserve(&self, alias: SrcAlias) -> Option<SrcAliasLocksReservation<'_>> {
        match self.inner.write().insert(alias) {
            true => {
                tracing::trace!("locking alias \"{alias}\"");
                Some(SrcAliasLocksReservation { locks: self, alias })
            }
            false => None,
        }
    }
}

pub(crate) struct SrcAliasLocksReservation<'a> {
    locks: &'a SrcAliasLocksGuarded,
    alias: SrcAlias,
}
impl Drop for SrcAliasLocksReservation<'_> {
    fn drop(&mut self) {
        tracing::trace!("unlocking alias \"{}\"", self.alias);
        self.locks.inner.write().remove(&self.alias);
    }
}
