use super::tpool::ThreadPool;
use crate::{
    sol::GuardedSolMap,
    src::{GuardedSrcAliasData, GuardedSrcAliasLocks},
};

pub struct Refine {
    pub(crate) tpool: ThreadPool,
    // Source-related fields
    pub(crate) cache_folder: Option<String>,
    pub(crate) src_alias_data: GuardedSrcAliasData,
    pub(crate) src_alias_locks: GuardedSrcAliasLocks,
    // Sol-related fields
    pub(crate) id_sol_map: GuardedSolMap,
}
impl Refine {
    pub fn new(cache_folder: Option<String>, standard_threads: usize, heavy_threads: usize) -> Self {
        Self {
            tpool: ThreadPool::new(standard_threads, heavy_threads),
            cache_folder,
            src_alias_data: GuardedSrcAliasData::new(),
            src_alias_locks: GuardedSrcAliasLocks::new(),
            id_sol_map: GuardedSolMap::new(),
        }
    }
}
