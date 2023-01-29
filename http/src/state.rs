pub(crate) struct AppState {
    pub(crate) srcmgr: reefast::SrcMgr,
}
impl AppState {
    pub(crate) fn new() -> AppState {
        AppState {
            srcmgr: reefast::SrcMgr::new(),
        }
    }
}
