use crate::{
    misc::Value,
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

pub(in crate::rd::data::item::attr_extras) fn get_max_group_fitted_limited(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> bool {
    get_attr_presence(item_attrs, attr_consts.max_group_fitted)
}
pub(in crate::rd::data::item::attr_extras) fn get_max_group_online_limited(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> bool {
    get_attr_presence(item_attrs, attr_consts.max_group_online)
}
pub(in crate::rd::data::item::attr_extras) fn get_max_group_active_limited(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> bool {
    get_attr_presence(item_attrs, attr_consts.max_group_active)
}

fn get_attr_presence(item_attrs: &RMap<RAttrId, Value>, attr_rid: Option<RAttrId>) -> bool {
    match attr_rid {
        Some(attr_rid) => item_attrs.contains_key(&attr_rid),
        None => false,
    }
}
