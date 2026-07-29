use super::getters::{
    activation_blocks::{get_activation_blocks_cloak, get_activation_blocks_in_assist},
    attr_val::{
        get_bandwidth_use, get_calibration_use, get_capacity, get_charge_rate, get_charge_size,
        get_fighter_refuel_duration, get_max_fighter_count, get_max_type_fitted_count, get_online_max_sec_class,
        get_overload_td_lvl, get_radius, get_remote_resist_attr_id, get_rig_size, get_volume,
    },
    charge_limit::get_item_charge_limit,
    container_limit::get_item_container_limit,
    cycle::{specifies_disallow_repeats, specifies_reactivation_delay},
    drone_limit::get_ship_drone_limit,
    effect_immunity::get_disallow_vs_ew_immune_tgt,
    fighter_kind::{
        get_heavy_fighter_flag, get_light_fighter_flag, get_st_heavy_fighter_flag, get_st_light_fighter_flag,
        get_st_support_fighter_flag, get_support_fighter_flag,
    },
    kind::detect_item_kind,
    max_group::{get_max_group_active_limited, get_max_group_fitted_limited, get_max_group_online_limited},
    mobility::{get_enables_conduit, get_enables_portal, get_entity_has_mwd, get_is_mobile, get_jump_fuel_type_id},
    sec_zone::is_sec_zone_limitable,
    ship_kind::get_item_ship_kind,
    ship_limit::get_item_ship_limit,
    slot_index::{get_booster_slot, get_implant_slot, get_subsystem_slot},
};
use crate::{
    Count, CountNz, PValue, SkillLevel, SlotIndex, Value,
    ad::{AAttrId, AItem, AItemId, AItemListId},
    dbg::DebugResult,
    misc::DetectedItemKind,
    rd::{
        RAttrConsts, RAttrId, RData, REffectConsts, REffectId, RItemAttrEffectData, RItemBase, RItemChargeLimit,
        RItemContLimit, RItemListId, RItemShipLimit, RShipDroneLimit, RShipKind, RcEffect,
    },
    ud::UData,
    util::{PSlab, RMap},
};

// Item attributes and any data which relies on item attributes.
#[derive(Clone, Default)]
pub(crate) struct RItemAttrData {
    // Raw data
    pub(crate) attrs: RMap<RAttrId, Value>,
    // Derived data - per-effect attribute-dependent data
    pub(crate) effect_adds: RMap<REffectId, RItemAttrEffectData>,
    // Derived data - unmutated and unmodified (by dogma modifiers) attribute values, cast to
    // necessary type
    pub(crate) volume: PValue,
    pub(crate) capacity: PValue,
    pub(crate) radius: PValue,
    pub(crate) calibration_use: Option<Value>,
    pub(crate) bandwidth_use: Option<Value>,
    pub(crate) rig_size: Option<Value>,    // On-rig and on-ship attribute
    pub(crate) charge_size: Option<Value>, // On-module and on-charge attribute
    pub(crate) charge_rate: Count,
    pub(crate) max_fighter_count: CountNz,
    pub(crate) fighter_refuel_duration: PValue,
    pub(crate) remote_resist_attr_rid: Option<RAttrId>,
    // Derived data - mobility
    pub(crate) is_mobile: bool,  // Used to differentiate between mobile and sentry drones
    pub(crate) entity_mwd: bool, // Used to differentiate between single/dual-prop drones
    pub(crate) jump_fuel_item_aid: Option<AItemId>,
    pub(crate) enables_conduit: bool,
    pub(crate) enables_portal: bool, // Used by bridge modules
    // Derived data - module cycle flags
    pub(crate) specs_reactivation_delay: bool,
    pub(crate) specs_disallow_repeats: bool,
    // Derived data - fighter kind flags
    pub(crate) is_light_fighter: bool,
    pub(crate) is_heavy_fighter: bool,
    pub(crate) is_support_fighter: bool,
    pub(crate) is_st_light_fighter: bool,
    pub(crate) is_st_heavy_fighter: bool,
    pub(crate) is_st_support_fighter: bool,
    // Derived data - slot index this item takes
    pub(crate) implant_slot: Option<SlotIndex>,
    pub(crate) booster_slot: Option<SlotIndex>,
    pub(crate) subsystem_slot: Option<SlotIndex>,
    // Derived data - various aggregated limits
    pub(crate) ship_limit: Option<RItemShipLimit>, // Items can be fit to those ships
    pub(crate) charge_limit: Option<RItemChargeLimit>, // Items can load those charges
    pub(crate) cont_limit: Option<RItemContLimit>, // Charges can be loaded into those items
    pub(crate) drone_limit: Option<RShipDroneLimit>, // Ship can use those drones
    // Derived data - is item limitable by an appropriate "max group" limit, or cannot be affected
    // at all
    pub(crate) max_group_fitted_limited: bool,
    pub(crate) max_group_online_limited: bool,
    pub(crate) max_group_active_limited: bool,
    // Derived data - self-limits
    pub(crate) max_type_fitted: Option<Count>, // Max amount of fit items of this type ID
    pub(crate) sec_zone_limitable: bool,       // If item can be sec zone limited altogether
    pub(crate) online_max_sec_class: Option<Value>, // 2 hisec, 1 lowsec, 0 the rest
    pub(crate) disallow_vs_ew_immune_tgt: bool,
    // Derived data - ship limits
    pub(crate) activation_blocks_cloak: bool,
    pub(crate) activation_blocks_in_assist: bool,
    // Derived data - misc
    pub(crate) kind: Option<DetectedItemKind>,
    pub(crate) item_ship_kind: Option<RShipKind>, // Which ship type this item fits to
    pub(crate) overload_td_lvl: Option<SkillLevel>, // Required thermodynamics level for overheat
}
impl RItemAttrData {
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
impl RItemAttrData {
    pub(crate) fn from_attrs(attrs: RMap<RAttrId, Value>, r_base: &RItemBase, r_data: &RData) -> Self {
        let mut data = Self::default();
        data.attrs = attrs;
        data.fill_derived(
            r_base,
            &r_data.item_list_aid_rid_map,
            &r_data.attr_aid_rid_map,
            &r_data.attr_consts,
            &r_data.effect_consts,
            &r_data.effects,
        );
        data
    }
    pub(in crate::rd::data::item) fn fill_runtime(
        &mut self,
        r_base: &RItemBase,
        a_items: &RMap<AItemId, AItem>,
        item_list_aid_rid_map: &RMap<AItemListId, RItemListId>,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
        attr_consts: &RAttrConsts,
        effect_consts: &REffectConsts,
        r_effects: &PSlab<REffectId, RcEffect>,
    ) {
        let a_item = a_items.get(&r_base.aid).unwrap();
        // Raw data
        for a_item_attr in a_item.attrs.iter() {
            if let Some(&attr_rid) = attr_aid_rid_map.get(&a_item_attr.id) {
                self.attrs.insert(attr_rid, Value::from_a_value(a_item_attr.value));
            }
        }
        self.fill_derived(
            r_base,
            item_list_aid_rid_map,
            attr_aid_rid_map,
            attr_consts,
            effect_consts,
            r_effects,
        );
    }
    fn fill_derived(
        &mut self,
        r_base: &RItemBase,
        item_list_aid_rid_map: &RMap<AItemListId, RItemListId>,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
        attr_consts: &RAttrConsts,
        effect_consts: &REffectConsts,
        r_effects: &PSlab<REffectId, RcEffect>,
    ) {
        // Per-effect data
        for (&effect_rid, r_effect_data) in r_base.effects.iter() {
            let Some(r_item_attr_effect) = RItemAttrEffectData::try_from_r_effect_data(
                r_effect_data,
                &self.attrs,
                effect_rid,
                item_list_aid_rid_map,
                r_effects,
            ) else {
                continue;
            };
            self.effect_adds.insert(effect_rid, r_item_attr_effect);
        }
        // Unmutated and unmodified attribute values
        self.volume = get_volume(&self.attrs, attr_consts);
        self.capacity = get_capacity(&self.attrs, attr_consts);
        self.radius = get_radius(&self.attrs, attr_consts);
        self.calibration_use = get_calibration_use(&self.attrs, attr_consts);
        self.bandwidth_use = get_bandwidth_use(&self.attrs, attr_consts);
        self.rig_size = get_rig_size(&self.attrs, attr_consts);
        self.charge_size = get_charge_size(&self.attrs, attr_consts);
        self.charge_rate = get_charge_rate(&self.attrs, attr_consts);
        self.max_fighter_count = get_max_fighter_count(&self.attrs, attr_consts);
        self.fighter_refuel_duration = get_fighter_refuel_duration(&self.attrs, attr_consts);
        self.remote_resist_attr_rid = get_remote_resist_attr_id(&self.attrs, attr_consts, attr_aid_rid_map);
        // Mobility
        self.is_mobile = get_is_mobile(&self.attrs, attr_consts);
        self.entity_mwd = get_entity_has_mwd(&self.attrs, attr_consts);
        self.jump_fuel_item_aid = get_jump_fuel_type_id(&self.attrs, attr_consts);
        self.enables_conduit = get_enables_conduit(&self.attrs, attr_consts);
        self.enables_portal = get_enables_portal(&self.attrs, attr_consts);
        // Module cycle flags
        self.specs_reactivation_delay = specifies_reactivation_delay(&self.attrs, attr_consts);
        self.specs_disallow_repeats = specifies_disallow_repeats(&self.attrs, attr_consts);
        // Fighter kind flags
        self.is_light_fighter = get_light_fighter_flag(&self.attrs, attr_consts);
        self.is_heavy_fighter = get_heavy_fighter_flag(&self.attrs, attr_consts);
        self.is_support_fighter = get_support_fighter_flag(&self.attrs, attr_consts);
        self.is_st_light_fighter = get_st_light_fighter_flag(&self.attrs, attr_consts);
        self.is_st_heavy_fighter = get_st_heavy_fighter_flag(&self.attrs, attr_consts);
        self.is_st_support_fighter = get_st_support_fighter_flag(&self.attrs, attr_consts);
        // Slot index this item takes
        self.implant_slot = get_implant_slot(&self.attrs, attr_consts);
        self.booster_slot = get_booster_slot(&self.attrs, attr_consts);
        self.subsystem_slot = get_subsystem_slot(&self.attrs, attr_consts);
        // Various aggregated limits
        self.ship_limit = get_item_ship_limit(r_base.aid, &self.attrs, attr_consts);
        self.charge_limit = get_item_charge_limit(&self.attrs, attr_consts);
        self.cont_limit = get_item_container_limit(&self.attrs, attr_consts);
        self.drone_limit = get_ship_drone_limit(&self.attrs, attr_consts);
        // Is item limitable by an appropriate "max group" limit
        self.max_group_fitted_limited = get_max_group_fitted_limited(&self.attrs, attr_consts);
        self.max_group_online_limited = get_max_group_online_limited(&self.attrs, attr_consts);
        self.max_group_active_limited = get_max_group_active_limited(&self.attrs, attr_consts);
        // Self-limits
        self.max_type_fitted = get_max_type_fitted_count(&self.attrs, attr_consts);
        self.sec_zone_limitable = is_sec_zone_limitable(&self.attrs, attr_consts);
        self.online_max_sec_class = get_online_max_sec_class(&self.attrs, attr_consts);
        self.disallow_vs_ew_immune_tgt = get_disallow_vs_ew_immune_tgt(&self.attrs, attr_consts);
        // Ship limits
        self.activation_blocks_cloak = get_activation_blocks_cloak(&self.attrs, attr_consts);
        self.activation_blocks_in_assist = get_activation_blocks_in_assist(&self.attrs, attr_consts);
        // Misc
        self.kind = detect_item_kind(
            r_base.grp_id,
            r_base.cat_id,
            &self.attrs,
            &r_base.effects,
            attr_consts,
            effect_consts,
        );
        self.item_ship_kind = get_item_ship_kind(r_base.cat_id, &self.attrs, attr_consts);
        self.overload_td_lvl = get_overload_td_lvl(&self.attrs, attr_consts);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RItemAttrData {
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for attr_rid in self.attrs.keys() {
            attr_rid.consistency_check(u_data)?;
        }
        for effect_rid in self.effect_adds.keys() {
            effect_rid.consistency_check(u_data)?;
        }
        if let Some(attr_rid) = self.remote_resist_attr_rid {
            attr_rid.consistency_check(u_data)?;
        }
        Ok(())
    }
}
