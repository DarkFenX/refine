use crate::svc::{SolMapGuarded, SrcAliasDataGuarded, SrcAliasLocksGuarded, ThreadPool};

pub struct Refine {
    pub(crate) tpool: ThreadPool,
    // Source-related fields
    pub(crate) cache_folder: Option<String>,
    pub(crate) src_alias_data: SrcAliasDataGuarded,
    pub(crate) src_alias_locks: SrcAliasLocksGuarded,
    // Sol-related fields
    pub(crate) id_sol_map: SolMapGuarded,
}
impl Refine {
    pub fn new(cache_folder: Option<String>, standard_threads: usize, heavy_threads: usize) -> Self {
        Self {
            tpool: ThreadPool::new(standard_threads, heavy_threads),
            cache_folder,
            src_alias_data: SrcAliasDataGuarded::new(),
            src_alias_locks: SrcAliasLocksGuarded::new(),
            id_sol_map: SolMapGuarded::new(),
        }
    }
}
