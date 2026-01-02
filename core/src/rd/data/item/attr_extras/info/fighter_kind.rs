use crate::{
    ad::AAttrVal,
    def::OF,
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

pub(in crate::rd::data::item::attr_extras) fn get_light_fighter_flag(
    item_attrs: &RMap<RAttrId, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> bool {
    get_fighter_flag(item_attrs, attr_consts.ftr_sq_is_light)
}

pub(in crate::rd::data::item::attr_extras) fn get_heavy_fighter_flag(
    item_attrs: &RMap<RAttrId, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> bool {
    get_fighter_flag(item_attrs, attr_consts.ftr_sq_is_heavy)
}

pub(in crate::rd::data::item::attr_extras) fn get_support_fighter_flag(
    item_attrs: &RMap<RAttrId, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> bool {
    get_fighter_flag(item_attrs, attr_consts.ftr_sq_is_support)
}

pub(in crate::rd::data::item::attr_extras) fn get_st_light_fighter_flag(
    item_attrs: &RMap<RAttrId, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> bool {
    get_fighter_flag(item_attrs, attr_consts.ftr_sq_is_st_light)
}

pub(in crate::rd::data::item::attr_extras) fn get_st_heavy_fighter_flag(
    item_attrs: &RMap<RAttrId, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> bool {
    get_fighter_flag(item_attrs, attr_consts.ftr_sq_is_st_heavy)
}

pub(in crate::rd::data::item::attr_extras) fn get_st_support_fighter_flag(
    item_attrs: &RMap<RAttrId, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> bool {
    get_fighter_flag(item_attrs, attr_consts.ftr_sq_is_st_support)
}

fn get_fighter_flag(item_attrs: &RMap<RAttrId, AAttrVal>, attr_key: Option<RAttrId>) -> bool {
    match attr_key.and_then(|v| item_attrs.get(&v)) {
        Some(&value) => value != OF(0.0),
        None => false,
    }
}
