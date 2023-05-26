use std::sync::Arc;

use crate::bridge::{SolSysMgr, SrcMgr};

pub(crate) struct InnerAppState {
    pub(crate) src_mgr: SrcMgr,
    pub(crate) ss_mgr: SolSysMgr,
}
impl InnerAppState {
    pub(crate) fn new(cache_folder: Option<String>) -> Self {
        Self {
            src_mgr: SrcMgr::new(cache_folder),
            ss_mgr: SolSysMgr::new(),
        }
    }
}

pub(crate) type AppState = Arc<InnerAppState>;
