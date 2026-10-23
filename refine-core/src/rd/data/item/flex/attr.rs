use super::getters::{
    attr_val::{get_bandwidth_use, get_overload_td_lvl, get_remote_resist_attr_id},
    cycle::{specifies_disallow_repeats, specifies_reactivation_delay},
    max_group::{get_max_group_active_limited, get_max_group_fitted_limited, get_max_group_online_limited},
    mobility::{get_entity_has_mwd, get_is_mobile},
};
use crate::{
    SkillLevel, Value,
    ad::{AAttrId, AItem, AItemId},
    dbg::DebugResult,
    rd::{RAttrConsts, RAttrId, RData, RItemBase},
    ud::UData,
    util::RMap,
};

/// Flexible item data. It stores attributes and data derived from them for cases when those
/// attributes might need to be supplied outside from item itself.
///
/// For unmutated items, this container just stores item attributes, as well as data derived from
/// them. The distinction with item base becomes important for mutated items: this data container
/// stores merged attributes (base item + mutated item), as well as all the derived data which needs
/// merged attributes specifically. If it does not need merged attributes - it better be put onto
/// base item.
#[derive(Clone, Default)]
pub(crate) struct RItemFlexData {
    // Raw data
    pub(crate) attrs: RMap<RAttrId, Value>,
    // Derived data - unmutated and unmodified (by dogma modifiers) attribute values, cast to
    // necessary type
    /// Mutated drones do not specify bandwidth, it has to be taken from base item
    pub(crate) bandwidth_use: Option<Value>,
    /// Mutated webs and nosfs do not specify remoteResistanceID, it has to be taken from base item.
    /// Nosfs likely do not use it (it is defined on the nosf effect), but webs certainly do.
    pub(crate) remote_resist_attr_rid: Option<RAttrId>,
    // Derived data - mobility
    /// Used to differentiate between mobile and sentry drones. Relies on maxVelocity attribute,
    /// which is not specified on mutated drones, so has to be taken from base item.
    pub(crate) is_mobile: bool,
    /// Used to differentiate between single/dual-prop drones. Relies on entityFlySpeed attribute,
    /// which is not specified on mutated drones, so has to be taken from base item.
    pub(crate) entity_mwd: bool,
    // Derived data - module cycle flags
    /// Mutated ADCs do not specify it, it is taken from base item; have to use merged attrs.
    pub(crate) specs_reactivation_delay: bool,
    /// Mutated ADCs do not specify it, it is taken from base item; have to use merged attrs.
    pub(crate) specs_disallow_repeats: bool,
    // Derived data - is item limitable by an appropriate "max group" limit, or cannot be affected
    // at all. Those fields are put here because mutated item sometimes does not have limit defined,
    // while base item does (siege modules, ancillary reps).
    pub(crate) max_group_fitted_limited: bool,
    pub(crate) max_group_online_limited: bool,
    pub(crate) max_group_active_limited: bool,
    // Derived data - misc
    /// Required thermodynamics level for overheat. Mutated items borrow it from base item, so have
    /// to keep it here.
    pub(crate) overload_td_lvl: Option<SkillLevel>,
}
impl RItemFlexData {
    pub(crate) fn get_oattr_ffb(&self, attr_rid: Option<RAttrId>, fallback: Value) -> Value {
        let Some(attr_rid) = attr_rid else {
            return fallback;
        };
        match self.attrs.get(&attr_rid) {
            Some(attr_value) => *attr_value,
            None => fallback,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RItemFlexData {
    pub(crate) fn from_attrs(attrs: RMap<RAttrId, Value>, r_data: &RData) -> Self {
        let mut data = Self {
            attrs,
            ..Self::default()
        };
        data.fill_derived(&r_data.attr_aid_rid_map, &r_data.attr_consts);
        data
    }
    pub(in crate::rd::data::item) fn fill_runtime(
        &mut self,
        r_base: &RItemBase,
        a_items: &RMap<AItemId, AItem>,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
        attr_consts: &RAttrConsts,
    ) {
        let a_item = a_items.get(&r_base.aid).unwrap();
        // Raw data
        for a_item_attr in a_item.attrs.iter() {
            if let Some(&attr_rid) = attr_aid_rid_map.get(&a_item_attr.id) {
                self.attrs.insert(attr_rid, Value::from_a_value(a_item_attr.value));
            }
        }
        self.fill_derived(attr_aid_rid_map, attr_consts);
    }
    fn fill_derived(&mut self, attr_aid_rid_map: &RMap<AAttrId, RAttrId>, attr_consts: &RAttrConsts) {
        let Self {
            attrs,
            bandwidth_use,
            remote_resist_attr_rid,
            is_mobile,
            entity_mwd,
            specs_reactivation_delay,
            specs_disallow_repeats,
            max_group_fitted_limited,
            max_group_online_limited,
            max_group_active_limited,
            overload_td_lvl,
        } = self;
        // Unmutated and unmodified attribute values
        *bandwidth_use = get_bandwidth_use(attrs, attr_consts);
        *remote_resist_attr_rid = get_remote_resist_attr_id(attrs, attr_consts, attr_aid_rid_map);
        // Mobility
        *is_mobile = get_is_mobile(attrs, attr_consts);
        *entity_mwd = get_entity_has_mwd(attrs, attr_consts);
        // Module cycle flags
        *specs_reactivation_delay = specifies_reactivation_delay(attrs, attr_consts);
        *specs_disallow_repeats = specifies_disallow_repeats(attrs, attr_consts);
        // Is item limitable by an appropriate "max group" limit
        *max_group_fitted_limited = get_max_group_fitted_limited(attrs, attr_consts);
        *max_group_online_limited = get_max_group_online_limited(attrs, attr_consts);
        *max_group_active_limited = get_max_group_active_limited(attrs, attr_consts);
        // Misc
        *overload_td_lvl = get_overload_td_lvl(attrs, attr_consts);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RItemFlexData {
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        let Self {
            attrs,
            remote_resist_attr_rid,
            // Not checked
            bandwidth_use: _,
            is_mobile: _,
            entity_mwd: _,
            specs_reactivation_delay: _,
            specs_disallow_repeats: _,
            max_group_fitted_limited: _,
            max_group_online_limited: _,
            max_group_active_limited: _,
            overload_td_lvl: _,
        } = self;
        for attr_rid in attrs.keys() {
            attr_rid.consistency_check(u_data)?;
        }
        if let Some(attr_rid) = *remote_resist_attr_rid {
            attr_rid.consistency_check(u_data)?;
        }
        Ok(())
    }
}
