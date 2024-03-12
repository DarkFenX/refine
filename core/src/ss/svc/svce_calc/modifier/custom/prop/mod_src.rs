use crate::{
    defs::{EAttrId, SsItemId},
    ss::SsView,
};

use super::{
    attr::{PROP_BOOST, PROP_THRUST, SHIP_MASS},
    misc::get_ship_id,
};

pub(in crate::ss::svc::svce_calc::modifier) fn get_srcs(
    ss_view: &SsView,
    item_id: &SsItemId,
) -> Vec<(SsItemId, EAttrId)> {
    let mut srcs = Vec::new();
    if let Ok(Some(ship_id)) = get_ship_id(ss_view, item_id) {
        srcs.push((*item_id, PROP_BOOST));
        srcs.push((*item_id, PROP_THRUST));
        srcs.push((ship_id, SHIP_MASS));
    }
    srcs
}
