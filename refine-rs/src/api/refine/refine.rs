use crate::svc::{AdCaching, SolMapGuarded, SrcAliasDataGuarded, SrcAliasLocksGuarded, ThreadPool};

/// Main object of the library.
///
/// Holds state required for all the operations. Every other entity is created via it, either
/// directly or indirectly.
///
/// Has a set of constructors which depends on feature flags `adc-*`. When core library receives EVE
/// data, it processes it into format which is more convenient to work with. This processing can
/// take some time (usually a few seconds); if it is undesirable, result can be cached somewhere -
/// the cache is much faster to load. No-cache constructor is always available, the rest are enabled
/// by the feature flags.
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
    /// Constructs main object without adapted data caching.
    pub fn new(standard_threads: usize, heavy_threads: usize) -> Self {
        Self {
            tpool: ThreadPool::new(standard_threads, heavy_threads),
            ad_caching: AdCaching::Disabled,
            src_alias_data: SrcAliasDataGuarded::new(),
            src_alias_locks: SrcAliasLocksGuarded::new(),
            id_sol_map: SolMapGuarded::new(),
        }
    }
    /// Constructs main object with adapted data cache stored on local filesystem.
    #[cfg(feature = "adc-fs")]
    pub fn with_fs_adc(standard_threads: usize, heavy_threads: usize, adc_dir: std::path::PathBuf) -> Self {
        Self {
            tpool: ThreadPool::new(standard_threads, heavy_threads),
            ad_caching: AdCaching::Filesystem { dir: adc_dir },
            src_alias_data: SrcAliasDataGuarded::new(),
            src_alias_locks: SrcAliasLocksGuarded::new(),
            id_sol_map: SolMapGuarded::new(),
        }
    }
}
