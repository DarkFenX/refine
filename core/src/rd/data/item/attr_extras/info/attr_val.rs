use crate::{
    ad::{AAttrId, AAttrVal, ACount, AEveAttrId, ASkillLevel},
    def::OF,
    rd::{RAttrConsts, RAttrKey},
    util::RMap,
};

pub(in crate::rd::data::item::attr_extras) fn get_volume(
    item_attrs: &RMap<RAttrKey, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> AAttrVal {
    match attr_consts.volume.and_then(|v| item_attrs.get(&v)) {
        Some(volume) => *volume,
        None => OF(0.0),
    }
}
pub(in crate::rd::data::item::attr_extras) fn get_capacity(
    item_attrs: &RMap<RAttrKey, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> AAttrVal {
    match attr_consts.capacity.and_then(|v| item_attrs.get(&v)) {
        Some(capacity) => *capacity,
        None => OF(0.0),
    }
}
pub(in crate::rd::data::item::attr_extras) fn get_radius(
    item_attrs: &RMap<RAttrKey, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> AAttrVal {
    match attr_consts.radius.and_then(|v| item_attrs.get(&v)) {
        Some(radius) => *radius,
        None => OF(0.0),
    }
}

pub(in crate::rd::data::item::attr_extras) fn get_bandwidth_use(
    item_attrs: &RMap<RAttrKey, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> Option<AAttrVal> {
    attr_consts
        .drone_bandwidth_used
        .and_then(|v| item_attrs.get(&v).copied())
}

pub(in crate::rd::data::item::attr_extras) fn get_calibration_use(
    item_attrs: &RMap<RAttrKey, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> Option<AAttrVal> {
    attr_consts.upgrade_cost.and_then(|v| item_attrs.get(&v).copied())
}

pub(in crate::rd::data::item::attr_extras) fn get_rig_size(
    item_attrs: &RMap<RAttrKey, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> Option<AAttrVal> {
    attr_consts.rig_size.and_then(|v| item_attrs.get(&v).copied())
}

pub(in crate::rd::data::item::attr_extras) fn get_max_type_fitted_count(
    item_attrs: &RMap<RAttrKey, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> Option<ACount> {
    attr_consts
        .max_type_fitted
        .and_then(|v| item_attrs.get(&v))
        .map(|v| v.round() as ACount)
}

pub(in crate::rd::data::item::attr_extras) fn get_online_max_sec_class(
    item_attrs: &RMap<RAttrKey, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> Option<AAttrVal> {
    attr_consts
        .online_max_security_class
        .and_then(|v| item_attrs.get(&v).copied())
}

pub(in crate::rd::data::item::attr_extras) fn get_remote_resist_attr_id(
    item_attrs: &RMap<RAttrKey, AAttrVal>,
    attr_consts: &RAttrConsts,
    attr_id_key_map: &RMap<AAttrId, RAttrKey>,
) -> Option<RAttrKey> {
    attr_consts
        .remote_resist_id
        .and_then(|v| item_attrs.get(&v))
        .and_then(|v| match v {
            OF(0.0) => None,
            v => {
                let attr_id = AAttrId::Eve(v.into_inner().round() as AEveAttrId);
                attr_id_key_map.get(&attr_id).copied()
            }
        })
}

pub(in crate::rd::data::item::attr_extras) fn get_overload_td_lvl(
    item_attrs: &RMap<RAttrKey, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> Option<ASkillLevel> {
    attr_consts
        .required_thermodynamics_skill
        .and_then(|v| item_attrs.get(&v).map(|v| ASkillLevel::new(v.round() as i32)))
}

pub(in crate::rd::data::item::attr_extras) fn get_charge_size(
    item_attrs: &RMap<RAttrKey, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> Option<AAttrVal> {
    attr_consts.charge_size.and_then(|v| item_attrs.get(&v).copied())
}
pub(in crate::rd::data::item::attr_extras) fn get_charge_rate(
    item_attrs: &RMap<RAttrKey, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> ACount {
    match attr_consts.charge_rate.and_then(|v| item_attrs.get(&v)) {
        Some(val) => val.round() as ACount,
        None => 1,
    }
}

pub(in crate::rd::data::item::attr_extras) fn get_max_fighter_count(
    item_attrs: &RMap<RAttrKey, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> ACount {
    match attr_consts.ftr_sq_max_size.and_then(|v| item_attrs.get(&v)) {
        // Ensure there can be at least 1 fighter in a squad
        Some(value) => ACount::max(value.round() as ACount, 1),
        None => 1,
    }
}

pub(in crate::rd::data::item::attr_extras) fn get_fighter_refuel_time_s(
    item_attrs: &RMap<RAttrKey, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> AAttrVal {
    match attr_consts.ftr_refueling_time.and_then(|v| item_attrs.get(&v)) {
        Some(value) => value.max(&OF(0.0)) / OF(1000.0),
        None => OF(0.0),
    }
}
