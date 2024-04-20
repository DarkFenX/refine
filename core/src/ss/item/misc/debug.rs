use crate::{
    defs::SsItemId,
    ss::SsView,
    util::{DebugError, DebugResult},
};

pub(in crate::ss::item) fn check_item(ss_view: &SsView, item_id: &SsItemId) -> DebugResult {
    match ss_view.items.get_item(item_id) {
        Ok(_) => Ok(()),
        _ => return Err(DebugError::new()),
    }
}
