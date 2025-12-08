use crate::{
    ad::AAttrVal,
    def::OF,
    rd::{RAttrConsts, RAttrKey},
    util::RMap,
};

pub(super) fn get_disallow_vs_ew_immune_tgt(item_attrs: &RMap<RAttrKey, AAttrVal>, attr_consts: &RAttrConsts) -> bool {
    match attr_consts.disallow_vs_ew_immune_tgt.and_then(|v| item_attrs.get(&v)) {
        Some(&val) => val != OF(0.0),
        None => false,
    }
}
