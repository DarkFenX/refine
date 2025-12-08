use crate::{
    ad::AAttrVal,
    rd::{RAttrConsts, RAttrKey},
    util::RMap,
};

pub(super) fn get_max_group_fitted_limited(item_attrs: &RMap<RAttrKey, AAttrVal>, attr_consts: &RAttrConsts) -> bool {
    get_attr_presence(item_attrs, attr_consts.max_group_fitted)
}
pub(super) fn get_max_group_online_limited(item_attrs: &RMap<RAttrKey, AAttrVal>, attr_consts: &RAttrConsts) -> bool {
    get_attr_presence(item_attrs, attr_consts.max_group_online)
}
pub(super) fn get_max_group_active_limited(item_attrs: &RMap<RAttrKey, AAttrVal>, attr_consts: &RAttrConsts) -> bool {
    get_attr_presence(item_attrs, attr_consts.max_group_active)
}

fn get_attr_presence(item_attrs: &RMap<RAttrKey, AAttrVal>, attr_key: Option<RAttrKey>) -> bool {
    match attr_key {
        Some(attr_key) => item_attrs.contains_key(&attr_key),
        None => false,
    }
}
