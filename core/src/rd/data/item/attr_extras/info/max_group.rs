use crate::{
    ad::AAttrVal,
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

pub(in crate::rd::data::item::attr_extras) fn get_max_group_fitted_limited(
    item_attrs: &RMap<RAttrId, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> bool {
    get_attr_presence(item_attrs, attr_consts.max_group_fitted)
}
pub(in crate::rd::data::item::attr_extras) fn get_max_group_online_limited(
    item_attrs: &RMap<RAttrId, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> bool {
    get_attr_presence(item_attrs, attr_consts.max_group_online)
}
pub(in crate::rd::data::item::attr_extras) fn get_max_group_active_limited(
    item_attrs: &RMap<RAttrId, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> bool {
    get_attr_presence(item_attrs, attr_consts.max_group_active)
}

fn get_attr_presence(item_attrs: &RMap<RAttrId, AAttrVal>, attr_key: Option<RAttrId>) -> bool {
    match attr_key {
        Some(attr_key) => item_attrs.contains_key(&attr_key),
        None => false,
    }
}
