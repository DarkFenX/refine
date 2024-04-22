use std::sync::Arc;

use crate::bridge::{HSolMgr, HSrcMgr};

pub(crate) struct HInnerAppState {
    pub(crate) src_mgr: HSrcMgr,
    pub(crate) sol_mgr: HSolMgr,
}
impl HInnerAppState {
    pub(crate) fn new(cache_folder: Option<String>) -> Self {
        Self {
            src_mgr: HSrcMgr::new(cache_folder),
            sol_mgr: HSolMgr::new(),
        }
    }
}

pub(crate) type HAppState = Arc<HInnerAppState>;
