use crate::{
    ac,
    ad::{AAttrId, AAttrVal},
    def::OF,
    util::RMap,
};

pub(super) fn is_mobile(item_attrs: &RMap<AAttrId, AAttrVal>) -> bool {
    match item_attrs.get(&ac::attrs::MAX_VELOCITY) {
        Some(&max_velocity) => max_velocity > OF(0.0001),
        None => false,
    }
}
