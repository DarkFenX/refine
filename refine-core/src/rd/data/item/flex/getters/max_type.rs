use crate::{
    Count, Value,
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

pub(in crate::rd::data::item::flex) fn get_max_type_fitted_count(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> Option<Count> {
    attr_consts
        .max_type_fitted
        .and_then(|v| item_attrs.get(&v))
        .map(|&v| Count::from_value_rounded(v))
}
