use crate::{
    defs::{EAttrId, EEffectId, SsItemId},
    ss::SsView,
    util::{DebugError, DebugResult},
};

pub(in crate::ss::svc) fn check_item(ss_view: &SsView, item_id: &SsItemId) -> DebugResult {
    let item = match ss_view.items.get_item(item_id) {
        Ok(item) => item,
        _ => return Err(DebugError::new()),
    };
    if item.get_a_item().is_err() {
        return Err(DebugError::new());
    }
    Ok(())
}

pub(in crate::ss::svc) fn check_effect(ss_view: &SsView, effect_id: &EEffectId) -> DebugResult {
    if ss_view.src.get_a_effect(effect_id).is_none() {
        return Err(DebugError::new());
    }
    Ok(())
}

pub(in crate::ss::svc) fn check_attr(ss_view: &SsView, attr_id: &EAttrId) -> DebugResult {
    if ss_view.src.get_a_attr(attr_id).is_none() {
        return Err(DebugError::new());
    }
    Ok(())
}
