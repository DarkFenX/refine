use std::sync::Arc;

use crate::sol_sys_mgr::SolSysManager;

pub(crate) struct AppState {
    pub(crate) src_mgr: Arc<reefast::SrcMgr>,
    pub(crate) sol_sys_mgr: SolSysManager,
}
impl AppState {
    pub(crate) fn new() -> AppState {
        AppState {
            src_mgr: Arc::new(reefast::SrcMgr::new()),
            sol_sys_mgr: SolSysManager::new(),
        }
    }
}
