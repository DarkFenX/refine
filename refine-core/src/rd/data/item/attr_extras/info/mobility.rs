use crate::{
    ad::AItemId,
    num::Value,
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

pub(in crate::rd::data::item::attr_extras) fn get_is_mobile(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> bool {
    match attr_consts.max_velocity.and_then(|attr_rid| item_attrs.get(&attr_rid)) {
        Some(&max_velocity) => max_velocity > Value::from_f64(0.0001),
        None => false,
    }
}

pub(in crate::rd::data::item::attr_extras) fn get_entity_has_mwd(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> bool {
    match attr_consts
        .entity_cruise_speed
        .and_then(|attr_rid| item_attrs.get(&attr_rid))
    {
        Some(&cruise_speed) => cruise_speed > Value::from_f64(0.0001),
        None => false,
    }
}

pub(in crate::rd::data::item::attr_extras) fn get_jump_fuel_type_id(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> Option<AItemId> {
    attr_consts
        .jump_drive_consumption_type
        .and_then(|attr_rid| item_attrs.get(&attr_rid))
        .and_then(|value| AItemId::try_from_f64_rounded(value.into_f64()))
}

pub(in crate::rd::data::item::attr_extras) fn get_enables_conduit(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> bool {
    match attr_consts.enable_perform_conduit_jump.and_then(|v| item_attrs.get(&v)) {
        Some(&value) => value.is_flag_set(),
        None => false,
    }
}

pub(in crate::rd::data::item::attr_extras) fn get_enables_portal(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> bool {
    match attr_consts.enable_open_jump_portal.and_then(|v| item_attrs.get(&v)) {
        Some(&value) => value.is_flag_set(),
        None => false,
    }
}
