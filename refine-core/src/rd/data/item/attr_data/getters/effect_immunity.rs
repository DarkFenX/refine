use crate::{
    Value,
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

pub(in crate::rd::data::item::attr_data) fn get_disallow_vs_ew_immune_tgt(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> bool {
    match attr_consts.disallow_vs_ew_immune_tgt.and_then(|v| item_attrs.get(&v)) {
        Some(&value) => value.is_flag_set(),
        None => false,
    }
}
