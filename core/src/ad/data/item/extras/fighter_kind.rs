use crate::{
    defs::{AttrVal, EAttrId, OF},
    ec,
    util::StMap,
};

pub(super) fn get_light_fighter_flag(attrs: &StMap<EAttrId, AttrVal>) -> bool {
    get_fighter_flag(attrs, &ec::attrs::FTR_SQ_IS_LIGHT)
}

pub(super) fn get_heavy_fighter_flag(attrs: &StMap<EAttrId, AttrVal>) -> bool {
    get_fighter_flag(attrs, &ec::attrs::FTR_SQ_IS_HEAVY)
}

pub(super) fn get_support_fighter_flag(attrs: &StMap<EAttrId, AttrVal>) -> bool {
    get_fighter_flag(attrs, &ec::attrs::FTR_SQ_IS_SUPPORT)
}

pub(super) fn get_standup_light_fighter_flag(attrs: &StMap<EAttrId, AttrVal>) -> bool {
    get_fighter_flag(attrs, &ec::attrs::FTR_SQ_IS_STANDUP_LIGHT)
}

pub(super) fn get_standup_heavy_fighter_flag(attrs: &StMap<EAttrId, AttrVal>) -> bool {
    get_fighter_flag(attrs, &ec::attrs::FTR_SQ_IS_STANDUP_HEAVY)
}

pub(super) fn get_standup_support_fighter_flag(attrs: &StMap<EAttrId, AttrVal>) -> bool {
    get_fighter_flag(attrs, &ec::attrs::FTR_SQ_IS_STANDUP_SUPPORT)
}

fn get_fighter_flag(attrs: &StMap<EAttrId, AttrVal>, attr_id: &EAttrId) -> bool {
    match attrs.get(attr_id) {
        Some(&value) => value != OF(0.0),
        None => false,
    }
}
