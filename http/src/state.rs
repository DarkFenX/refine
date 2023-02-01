use std::sync::Arc;

pub(crate) struct AppState {
    pub(crate) srcmgr: Arc<reefast::SrcMgr>,
}
impl AppState {
    pub(crate) fn new() -> AppState {
        AppState {
            srcmgr: Arc::new(reefast::SrcMgr::new()),
        }
    }
}
