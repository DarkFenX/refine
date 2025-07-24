use crate::{ac, ad, def::OF, util::RMap};

pub(super) fn get_light_fighter_flag(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> bool {
    get_fighter_flag(item_attrs, &ac::attrs::FTR_SQ_IS_LIGHT)
}

pub(super) fn get_heavy_fighter_flag(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> bool {
    get_fighter_flag(item_attrs, &ac::attrs::FTR_SQ_IS_HEAVY)
}

pub(super) fn get_support_fighter_flag(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> bool {
    get_fighter_flag(item_attrs, &ac::attrs::FTR_SQ_IS_SUPPORT)
}

pub(super) fn get_st_light_fighter_flag(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> bool {
    get_fighter_flag(item_attrs, &ac::attrs::FTR_SQ_IS_ST_LIGHT)
}

pub(super) fn get_st_heavy_fighter_flag(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> bool {
    get_fighter_flag(item_attrs, &ac::attrs::FTR_SQ_IS_ST_HEAVY)
}

pub(super) fn get_st_support_fighter_flag(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> bool {
    get_fighter_flag(item_attrs, &ac::attrs::FTR_SQ_IS_ST_SUPPORT)
}

fn get_fighter_flag(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>, attr_id: &ad::AAttrId) -> bool {
    match item_attrs.get(attr_id) {
        Some(&value) => value != OF(0.0),
        None => false,
    }
}
