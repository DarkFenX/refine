use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use tokio::sync::RwLock;

use crate::{src::SrcAlias, tpool::ThreadPool};

pub struct Refine {
    pub(crate) tpool: ThreadPool,
    // Source-related fields
    pub(crate) cache_folder: Option<String>,
    pub(super) core_src_map: RwLock<HashMap<SrcAlias, Arc<rc::Src>>>,
    pub(super) default_src_alias: RwLock<Option<SrcAlias>>,
    pub(super) locked_src_aliases: RwLock<HashSet<SrcAlias>>,
}
impl Refine {
    pub fn new(cache_folder: Option<String>, standard_threads: usize, heavy_threads: usize) -> Self {
        Self {
            tpool: ThreadPool::new(standard_threads, heavy_threads),
            cache_folder,
            core_src_map: RwLock::new(HashMap::new()),
            default_src_alias: RwLock::new(None),
            locked_src_aliases: RwLock::new(HashSet::new()),
        }
    }
}
