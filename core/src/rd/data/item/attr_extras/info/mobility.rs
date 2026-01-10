use crate::{
    num::Value,
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

pub(in crate::rd::data::item::attr_extras) fn is_mobile(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> bool {
    match attr_consts.max_velocity.and_then(|attr_rid| item_attrs.get(&attr_rid)) {
        Some(&max_velocity) => max_velocity > Value::from_f64(0.0001),
        None => false,
    }
}
