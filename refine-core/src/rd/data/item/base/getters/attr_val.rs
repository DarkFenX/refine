use crate::{
    PValue, Value,
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

pub(in crate::rd::data::item::base) fn get_volume(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> PValue {
    match attr_consts.volume.and_then(|v| item_attrs.get(&v)) {
        Some(&volume) => PValue::from_value_clamped(volume),
        None => Default::default(),
    }
}
pub(in crate::rd::data::item::base) fn get_capacity(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> PValue {
    match attr_consts.capacity.and_then(|v| item_attrs.get(&v)) {
        Some(&capacity) => PValue::from_value_clamped(capacity),
        None => Default::default(),
    }
}
pub(in crate::rd::data::item::base) fn get_radius(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> PValue {
    match attr_consts.radius.and_then(|v| item_attrs.get(&v)) {
        Some(&radius) => PValue::from_value_clamped(radius),
        None => Default::default(),
    }
}
