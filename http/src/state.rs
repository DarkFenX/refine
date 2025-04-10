use std::sync::Arc;

use crate::bridge::{HSolMgr, HSrcMgr, HThreadPool};

pub(crate) struct HInnerAppState {
    pub(crate) src_mgr: HSrcMgr,
    pub(crate) sol_mgr: HSolMgr,
    pub(crate) tpool: HThreadPool,
}
impl HInnerAppState {
    pub(crate) fn new(cache_folder: Option<String>, std_threads: usize, heavy_threads: usize) -> Self {
        Self {
            src_mgr: HSrcMgr::new(cache_folder),
            sol_mgr: HSolMgr::new(),
            tpool: HThreadPool::new(std_threads, heavy_threads),
        }
    }
}

pub(crate) type HAppState = Arc<HInnerAppState>;
