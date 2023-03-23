use crate::{src_mgr::SrcMgr, ss::SolSysManager};

pub(crate) struct AppState {
    pub(crate) src_mgr: SrcMgr,
    pub(crate) ss_mgr: SolSysManager,
}
impl AppState {
    pub(crate) fn new(cache_folder: Option<String>) -> Self {
        Self {
            src_mgr: SrcMgr::new(cache_folder),
            ss_mgr: SolSysManager::new(),
        }
    }
}
