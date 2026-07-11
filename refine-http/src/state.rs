use std::sync::Arc;

use rs::Refine;

use crate::bridge::HSolMgr;

pub(crate) struct HInnerAppState {
    pub(crate) refine: Refine,
    pub(crate) sol_mgr: HSolMgr,
}
impl HInnerAppState {
    pub(crate) fn new(cache_folder: Option<String>, standard_threads: usize, heavy_threads: usize) -> Self {
        Self {
            refine: Refine::new(cache_folder, standard_threads, heavy_threads),
            sol_mgr: HSolMgr::new(),
        }
    }
}

pub(crate) type HAppState = Arc<HInnerAppState>;
