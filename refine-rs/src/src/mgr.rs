use std::collections::{HashMap, HashSet};

use tokio::sync::RwLock;

use super::src::Src;

pub struct SrcMgr {
    pub(super) cache_folder: Option<String>,
    pub(super) alias_src_map: RwLock<HashMap<String, Src>>,
    pub(super) default_alias: RwLock<Option<String>>,
    pub(super) locked_aliases: RwLock<HashSet<String>>,
}
impl SrcMgr {
    pub(crate) fn new(cache_folder: Option<String>) -> Self {
        Self {
            cache_folder,
            alias_src_map: RwLock::new(HashMap::new()),
            default_alias: RwLock::new(None),
            locked_aliases: RwLock::new(HashSet::new()),
        }
    }
}
