use std::sync::Arc;

use crate::bridge::{SrcMgr, SsMgr};

pub(crate) struct InnerAppState {
    pub(crate) src_mgr: SrcMgr,
    pub(crate) ss_mgr: SsMgr,
}
impl InnerAppState {
    pub(crate) fn new(cache_folder: Option<String>) -> Self {
        Self {
            src_mgr: SrcMgr::new(cache_folder),
            ss_mgr: SsMgr::new(),
        }
    }
}

pub(crate) type AppState = Arc<InnerAppState>;
