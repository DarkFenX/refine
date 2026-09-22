use crate::{
    Count, SkillLevel, Value,
    ad::AAttrId,
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

pub(in crate::rd::data::item::flex) fn get_bandwidth_use(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> Option<Value> {
    attr_consts
        .drone_bandwidth_used
        .and_then(|v| item_attrs.get(&v).copied())
}

pub(in crate::rd::data::item::flex) fn get_max_type_fitted_count(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> Option<Count> {
    attr_consts
        .max_type_fitted
        .and_then(|v| item_attrs.get(&v))
        .map(|&v| Count::from_value_rounded(v))
}

pub(in crate::rd::data::item::flex) fn get_online_max_sec_class(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> Option<Value> {
    attr_consts
        .online_max_security_class
        .and_then(|v| item_attrs.get(&v).copied())
}

pub(in crate::rd::data::item::flex) fn get_remote_resist_attr_id(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
    attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
) -> Option<RAttrId> {
    let attr_rid = attr_consts.remote_resist_id?;
    let attr_value = *item_attrs.get(&attr_rid)?;
    let attr_aid = AAttrId::try_eve_from_f64_rounded(attr_value.into_f64())?;
    attr_aid_rid_map.get(&attr_aid).copied()
}

pub(in crate::rd::data::item::flex) fn get_overload_td_lvl(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> Option<SkillLevel> {
    attr_consts
        .required_thermodynamics_skill
        .and_then(|v| item_attrs.get(&v).map(|&v| SkillLevel::from_f64_rounded(v.into_f64())))
}
