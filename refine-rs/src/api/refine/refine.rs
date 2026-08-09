use crate::svc::{SolMapGuarded, SrcAliasDataGuarded, SrcAliasLocksGuarded, ThreadPool};

/// Main object of the library.
///
/// Holds state required for all the operations. Every other entity is created via it, either
/// directly or indirectly.
pub struct Refine {
    pub(crate) tpool: ThreadPool,
    // Source-related fields
    pub(crate) src_alias_data: SrcAliasDataGuarded,
    pub(crate) src_alias_locks: SrcAliasLocksGuarded,
    // Sol-related fields
    pub(crate) id_sol_map: SolMapGuarded,
}
impl Refine {
    pub fn new(standard_threads: usize, heavy_threads: usize) -> Self {
        Self {
            tpool: ThreadPool::new(standard_threads, heavy_threads),
            src_alias_data: SrcAliasDataGuarded::new(),
            src_alias_locks: SrcAliasLocksGuarded::new(),
            id_sol_map: SolMapGuarded::new(),
        }
    }
}
