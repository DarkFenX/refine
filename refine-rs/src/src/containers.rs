use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use tokio::sync::RwLock;

use crate::src::SrcAlias;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Alias data
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) struct SrcAliasData {
    pub(super) inner: RwLock<SrcAliasDataInner>,
}
impl SrcAliasData {
    pub(crate) fn new() -> Self {
        Self {
            inner: RwLock::new(SrcAliasDataInner::new()),
        }
    }
}

pub(super) struct SrcAliasDataInner {
    pub(super) map: HashMap<SrcAlias, Arc<rc::Src>>,
    pub(super) default: Option<SrcAlias>,
}
impl SrcAliasDataInner {
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
pub(crate) struct SrcAliasLocks {
    pub(super) inner: RwLock<HashSet<SrcAlias>>,
}
impl SrcAliasLocks {
    pub(crate) fn new() -> Self {
        Self {
            inner: RwLock::new(HashSet::new()),
        }
    }
}
