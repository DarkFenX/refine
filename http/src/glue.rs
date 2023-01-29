use std::sync::Arc;

use tokio::sync::oneshot;

use crate::state::AppState;

#[derive(Debug)]
pub(crate) enum TaskStatus {
    Success,
    Error,
}

pub(crate) fn create_source(state: Arc<AppState>, alias: String, data_version: String, data_base_url: String) {
    let dh = Box::new(reefast::dh_impls::phobos::PhbHttpDHandler::new(data_base_url.as_str(), data_version).unwrap());
    let ch = Box::new(reefast::ch_impls::json_file::JsonFileCHandler::new(
        "/home/dfx/Workspace/eve/reefast/cache",
        alias.as_str(),
    ));
    match state.srcmgr.add(alias.as_str(), dh, ch, true) {
        Result(r) => Result(()),
        Err(e) => Err(Err),
    }
}
