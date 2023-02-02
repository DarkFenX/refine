use std::sync::Arc;

use crate::session::SessionManager;

pub(crate) struct AppState {
    pub(crate) source: Arc<reefast::SrcMgr>,
    pub(crate) session: SessionManager,
}
impl AppState {
    pub(crate) fn new() -> AppState {
        AppState {
            source: Arc::new(reefast::SrcMgr::new()),
            session: SessionManager::new(),
        }
    }
}
