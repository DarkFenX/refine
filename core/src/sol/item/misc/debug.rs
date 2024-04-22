use crate::{
    defs::SolItemId,
    sol::SolView,
    util::{DebugError, DebugResult},
};

pub(in crate::sol::item) fn check_item(sol_view: &SolView, item_id: &SolItemId) -> DebugResult {
    match sol_view.items.get_item(item_id) {
        Ok(_) => Ok(()),
        _ => return Err(DebugError::new()),
    }
}
