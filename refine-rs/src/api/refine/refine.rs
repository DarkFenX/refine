use crate::{
    AdCaching,
    svc::{SolMapGuarded, SrcAliasDataGuarded, SrcAliasLocksGuarded, ThreadPool},
};

pub struct Refine {
    pub(crate) tpool: ThreadPool,
    // Source-related fields
    pub(crate) ad_caching: AdCaching,
    pub(crate) src_alias_data: SrcAliasDataGuarded,
    pub(crate) src_alias_locks: SrcAliasLocksGuarded,
    // Sol-related fields
    pub(crate) id_sol_map: SolMapGuarded,
}
impl Refine {
    pub fn new(ad_caching: AdCaching, standard_threads: usize, heavy_threads: usize) -> Self {
        Self {
            tpool: ThreadPool::new(standard_threads, heavy_threads),
            ad_caching,
            src_alias_data: SrcAliasDataGuarded::new(),
            src_alias_locks: SrcAliasLocksGuarded::new(),
            id_sol_map: SolMapGuarded::new(),
        }
    }
}
