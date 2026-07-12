use std::{collections::HashMap, sync::Arc};

use tokio::sync::{Mutex, RwLock};

use super::tpool::ThreadPool;
use crate::{
    sol::{SolarSystemId, SolarSystemInner},
    src::{SrcAliasData, SrcAliasLocks},
};

pub struct Refine {
    pub(crate) tpool: ThreadPool,
    // Source-related fields
    pub(crate) cache_folder: Option<String>,
    pub(crate) src_alias_data: SrcAliasData,
    pub(crate) src_alias_locks: SrcAliasLocks,
    // Sol-related fields
    pub(crate) id_sol_map: RwLock<HashMap<SolarSystemId, Arc<Mutex<SolarSystemInner>>>>,
}
impl Refine {
    pub fn new(cache_folder: Option<String>, standard_threads: usize, heavy_threads: usize) -> Self {
        Self {
            tpool: ThreadPool::new(standard_threads, heavy_threads),
            cache_folder,
            src_alias_data: SrcAliasData::new(),
            src_alias_locks: SrcAliasLocks::new(),
            id_sol_map: RwLock::new(HashMap::new()),
        }
    }
}
