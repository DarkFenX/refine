use std::sync::Arc;

use crate::bridge::{HSrcMgr, HSsMgr};

pub(crate) struct HInnerAppState {
    pub(crate) src_mgr: HSrcMgr,
    pub(crate) ss_mgr: HSsMgr,
}
impl HInnerAppState {
    pub(crate) fn new(cache_folder: Option<String>) -> Self {
        Self {
            src_mgr: HSrcMgr::new(cache_folder),
            ss_mgr: HSsMgr::new(),
        }
    }
}

pub(crate) type HAppState = Arc<HInnerAppState>;
