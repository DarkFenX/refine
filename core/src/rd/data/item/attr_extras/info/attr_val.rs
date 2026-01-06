use crate::{
    ad::{AAttrId, AEveAttrId},
    misc::{Count, FighterCount, PValue, SkillLevel, Value},
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

pub(in crate::rd::data::item::attr_extras) fn get_volume(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> PValue {
    match attr_consts.volume.and_then(|v| item_attrs.get(&v)) {
        Some(&volume) => volume.into(),
        None => Default::default(),
    }
}
pub(in crate::rd::data::item::attr_extras) fn get_capacity(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> PValue {
    match attr_consts.capacity.and_then(|v| item_attrs.get(&v)) {
        Some(&capacity) => capacity.into(),
        None => Default::default(),
    }
}
pub(in crate::rd::data::item::attr_extras) fn get_radius(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> PValue {
    match attr_consts.radius.and_then(|v| item_attrs.get(&v)) {
        Some(&radius) => radius.into(),
        None => Default::default(),
    }
}

pub(in crate::rd::data::item::attr_extras) fn get_bandwidth_use(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> Option<Value> {
    attr_consts
        .drone_bandwidth_used
        .and_then(|v| item_attrs.get(&v).copied())
}

pub(in crate::rd::data::item::attr_extras) fn get_calibration_use(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> Option<Value> {
    attr_consts.upgrade_cost.and_then(|v| item_attrs.get(&v).copied())
}

pub(in crate::rd::data::item::attr_extras) fn get_rig_size(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> Option<Value> {
    attr_consts.rig_size.and_then(|v| item_attrs.get(&v).copied())
}

pub(in crate::rd::data::item::attr_extras) fn get_max_type_fitted_count(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> Option<Count> {
    attr_consts
        .max_type_fitted
        .and_then(|v| item_attrs.get(&v))
        .map(|&v| Count::from_f64_rounded(v.into()))
}

pub(in crate::rd::data::item::attr_extras) fn get_online_max_sec_class(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> Option<Value> {
    attr_consts
        .online_max_security_class
        .and_then(|v| item_attrs.get(&v).copied())
}

pub(in crate::rd::data::item::attr_extras) fn get_remote_resist_attr_id(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
    attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
) -> Option<RAttrId> {
    let attr_rid = attr_consts.remote_resist_id?;
    let attr_value = *item_attrs.get(&attr_rid)?;
    let eve_attr_aid = AEveAttrId::from_f64_rounded(attr_value.into());
    if eve_attr_aid == AEveAttrId::from_i32(0) {
        return None;
    }
    attr_aid_rid_map.get(&AAttrId::Eve(eve_attr_aid)).copied()
}

pub(in crate::rd::data::item::attr_extras) fn get_overload_td_lvl(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> Option<SkillLevel> {
    attr_consts
        .required_thermodynamics_skill
        .and_then(|v| item_attrs.get(&v).map(|&v| SkillLevel::from_f64_rounded(v.into())))
}

pub(in crate::rd::data::item::attr_extras) fn get_charge_size(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> Option<Value> {
    attr_consts.charge_size.and_then(|v| item_attrs.get(&v).copied())
}
pub(in crate::rd::data::item::attr_extras) fn get_charge_rate(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> Count {
    match attr_consts.charge_rate.and_then(|v| item_attrs.get(&v)) {
        Some(&val) => Count::from_f64_rounded(val.into()),
        None => Count::from_u32(1),
    }
}

pub(in crate::rd::data::item::attr_extras) fn get_max_fighter_count(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> FighterCount {
    match attr_consts.ftr_sq_max_size.and_then(|v| item_attrs.get(&v)) {
        // Ensure there can be at least 1 fighter in a squad
        Some(&value) => FighterCount::from_f64_rounded(value.into()),
        None => FighterCount::new_clamped(1),
    }
}

pub(in crate::rd::data::item::attr_extras) fn get_fighter_refuel_time(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> PValue {
    match attr_consts.ftr_refueling_time.and_then(|v| item_attrs.get(&v)) {
        Some(value) => PValue::from(value / 1000.0),
        None => PValue::default(),
    }
}
