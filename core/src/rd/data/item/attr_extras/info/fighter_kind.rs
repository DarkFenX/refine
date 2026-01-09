use crate::{
    misc::{PValue, Value},
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

pub(in crate::rd::data::item::attr_extras) fn get_light_fighter_flag(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> bool {
    get_fighter_flag(item_attrs, attr_consts.ftr_sq_is_light)
}

pub(in crate::rd::data::item::attr_extras) fn get_heavy_fighter_flag(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> bool {
    get_fighter_flag(item_attrs, attr_consts.ftr_sq_is_heavy)
}

pub(in crate::rd::data::item::attr_extras) fn get_support_fighter_flag(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> bool {
    get_fighter_flag(item_attrs, attr_consts.ftr_sq_is_support)
}

pub(in crate::rd::data::item::attr_extras) fn get_st_light_fighter_flag(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> bool {
    get_fighter_flag(item_attrs, attr_consts.ftr_sq_is_st_light)
}

pub(in crate::rd::data::item::attr_extras) fn get_st_heavy_fighter_flag(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> bool {
    get_fighter_flag(item_attrs, attr_consts.ftr_sq_is_st_heavy)
}

pub(in crate::rd::data::item::attr_extras) fn get_st_support_fighter_flag(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> bool {
    get_fighter_flag(item_attrs, attr_consts.ftr_sq_is_st_support)
}

fn get_fighter_flag(item_attrs: &RMap<RAttrId, Value>, attr_rid: Option<RAttrId>) -> bool {
    match attr_rid.and_then(|attr_rid| item_attrs.get(&attr_rid)) {
        Some(&value) => value.abs() > PValue::FLOAT_TOLERANCE,
        None => false,
    }
}
