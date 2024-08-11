use crate::{
    defs::SolItemId,
    sol::{SolDebugError, SolDebugResult, SolView},
};

pub(in crate::sol::item) fn check_item(sol_view: &SolView, item_id: &SolItemId) -> SolDebugResult {
    match sol_view.items.get_item(item_id) {
        Ok(_) => Ok(()),
        _ => return Err(SolDebugError::new()),
    }
}
