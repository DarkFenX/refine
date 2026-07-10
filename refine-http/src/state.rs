use std::sync::Arc;

use rs::Refine;

use crate::bridge::{HSolMgr, HSrcMgr};

pub(crate) struct HInnerAppState {
    pub(crate) refine: Refine,
    pub(crate) src_mgr: HSrcMgr,
    pub(crate) sol_mgr: HSolMgr,
}
impl HInnerAppState {
    pub(crate) fn new(cache_folder: Option<String>, standard_threads: usize, heavy_threads: usize) -> Self {
        Self {
            refine: Refine::new(standard_threads, heavy_threads),
            src_mgr: HSrcMgr::new(cache_folder),
            sol_mgr: HSolMgr::new(),
        }
    }
}

pub(crate) type HAppState = Arc<HInnerAppState>;
