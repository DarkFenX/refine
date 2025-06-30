use crate::{
    ac,
    ad::{AAttrId, AAttrVal},
    def::OF,
    util::RMap,
};

pub(super) fn get_disallow_vs_ew_immune_tgt(item_attrs: &RMap<AAttrId, AAttrVal>) -> bool {
    match item_attrs.get(&ac::attrs::DISALLOW_VS_EW_IMMUNE_TGT) {
        Some(&val) => val != OF(0.0),
        None => false,
    }
}
