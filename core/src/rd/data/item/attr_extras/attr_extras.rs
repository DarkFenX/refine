use super::info::{
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
    kind::get_item_kind,
    max_group::{get_max_group_active_limited, get_max_group_fitted_limited, get_max_group_online_limited},
    mobility::{get_enables_conduit, get_entity_has_mwd, get_is_mobile, get_jump_fuel_type_id},
    sec_zone::is_sec_zone_limitable,
    ship_kind::get_item_ship_kind,
    ship_limit::get_item_ship_limit,
    slot_index::{get_booster_slot, get_implant_slot, get_subsystem_slot},
};
use crate::{
    ad::{AAttrId, AItemCatId, AItemGrpId, AItemId},
    dbg::DebugResult,
    misc::ItemKind,
    num::{Count, FighterCount, PValue, SkillLevel, SlotIndex, Value},
    rd::{
        RAttrConsts, RAttrId, REffectConsts, REffectId, RItemChargeLimit, RItemContLimit, RItemEffectData,
        RItemShipLimit, RShipDroneLimit, RShipKind,
    },
    ud::UData,
    util::RMap,
};

// On-item container for data derived from item attributes. Has to be stored as a separate entity,
// since it has to be regenerated for mutated items, which get their attributes determined only
// during runtime.
#[derive(Clone, Default)]
pub(crate) struct RItemAXt {
    // Unmutated and unmodified (by dogma modifiers) attribute values, cast to necessary type
    pub(crate) volume: PValue,
    pub(crate) capacity: PValue,
    pub(crate) radius: PValue,
    pub(crate) calibration_use: Option<Value>,
    pub(crate) bandwidth_use: Option<Value>,
    pub(crate) rig_size: Option<Value>,    // On-rig and on-ship attribute
    pub(crate) charge_size: Option<Value>, // On-module and on-charge attribute
    pub(crate) charge_rate: Count,
    pub(crate) max_fighter_count: FighterCount,
    pub(crate) fighter_refuel_duration: PValue,
    pub(crate) remote_resist_attr_rid: Option<RAttrId>,
    // Mobility
    pub(crate) is_mobile: bool,  // Used to differentiate between mobile and sentry drones
    pub(crate) entity_mwd: bool, // Used to differentiate between single/dual-prop drones
    pub(crate) jump_fuel_type_id: Option<AItemId>,
    pub(crate) enables_conduit: bool,
    // Module cycle flags
    pub(crate) specs_reactivation_delay: bool,
    pub(crate) specs_disallow_repeats: bool,
    // Fighter kind flags
    pub(crate) is_light_fighter: bool,
    pub(crate) is_heavy_fighter: bool,
    pub(crate) is_support_fighter: bool,
    pub(crate) is_st_light_fighter: bool,
    pub(crate) is_st_heavy_fighter: bool,
    pub(crate) is_st_support_fighter: bool,
    // Slot index this item takes
    pub(crate) implant_slot: Option<SlotIndex>,
    pub(crate) booster_slot: Option<SlotIndex>,
    pub(crate) subsystem_slot: Option<SlotIndex>,
    // Various aggregated limits
    pub(crate) ship_limit: Option<RItemShipLimit>, // Items can be fit to those ships
    pub(crate) charge_limit: Option<RItemChargeLimit>, // Items can load those charges
    pub(crate) cont_limit: Option<RItemContLimit>, // Charges can be loaded into those items
    pub(crate) drone_limit: Option<RShipDroneLimit>, // Ship can use those drones
    // Is item limitable by an appropriate "max group" limit, or cannot be affected at all
    pub(crate) max_group_fitted_limited: bool,
    pub(crate) max_group_online_limited: bool,
    pub(crate) max_group_active_limited: bool,
    // Misc
    pub(crate) kind: Option<ItemKind>,
    pub(crate) item_ship_kind: Option<RShipKind>, // Which ship type this item fits to
    pub(crate) max_type_fitted: Option<Count>,    // Max amount of fit items of this type ID
    pub(crate) overload_td_lvl: Option<SkillLevel>, // Required thermodynamics level for overheat
    pub(crate) sec_zone_limitable: bool,          // If item can be sec zone limited altogether
    pub(crate) online_max_sec_class: Option<Value>, // 2 hisec, 1 lowsec, 0 the rest
    pub(crate) disallow_vs_ew_immune_tgt: bool,
}
impl RItemAXt {
    pub(crate) fn fill(
        &mut self,
        item_id: AItemId,
        item_grp_id: AItemGrpId,
        item_cat_id: AItemCatId,
        item_attrs: &RMap<RAttrId, Value>,
        item_effects: &RMap<REffectId, RItemEffectData>,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
        attr_consts: &RAttrConsts,
        effect_consts: &REffectConsts,
    ) {
        // Unmutated and unmodified (by dogma modifiers) attribute values, cast to necessary type
        self.volume = get_volume(item_attrs, attr_consts);
        self.capacity = get_capacity(item_attrs, attr_consts);
        self.radius = get_radius(item_attrs, attr_consts);
        self.calibration_use = get_calibration_use(item_attrs, attr_consts);
        self.bandwidth_use = get_bandwidth_use(item_attrs, attr_consts);
        self.rig_size = get_rig_size(item_attrs, attr_consts);
        self.charge_size = get_charge_size(item_attrs, attr_consts);
        self.charge_rate = get_charge_rate(item_attrs, attr_consts);
        self.max_fighter_count = get_max_fighter_count(item_attrs, attr_consts);
        self.fighter_refuel_duration = get_fighter_refuel_duration(item_attrs, attr_consts);
        self.remote_resist_attr_rid = get_remote_resist_attr_id(item_attrs, attr_consts, attr_aid_rid_map);
        // Mobility
        self.is_mobile = get_is_mobile(item_attrs, attr_consts);
        self.entity_mwd = get_entity_has_mwd(item_attrs, attr_consts);
        self.jump_fuel_type_id = get_jump_fuel_type_id(item_attrs, attr_consts);
        self.enables_conduit = get_enables_conduit(item_attrs, attr_consts);
        // Module cycle flags
        self.specs_reactivation_delay = specifies_reactivation_delay(item_attrs, attr_consts);
        self.specs_disallow_repeats = specifies_disallow_repeats(item_attrs, attr_consts);
        // Fighter kind flags
        self.is_light_fighter = get_light_fighter_flag(item_attrs, attr_consts);
        self.is_heavy_fighter = get_heavy_fighter_flag(item_attrs, attr_consts);
        self.is_support_fighter = get_support_fighter_flag(item_attrs, attr_consts);
        self.is_st_light_fighter = get_st_light_fighter_flag(item_attrs, attr_consts);
        self.is_st_heavy_fighter = get_st_heavy_fighter_flag(item_attrs, attr_consts);
        self.is_st_support_fighter = get_st_support_fighter_flag(item_attrs, attr_consts);
        // Slot index this item takes
        self.implant_slot = get_implant_slot(item_attrs, attr_consts);
        self.booster_slot = get_booster_slot(item_attrs, attr_consts);
        self.subsystem_slot = get_subsystem_slot(item_attrs, attr_consts);
        // Various aggregated limits
        self.ship_limit = get_item_ship_limit(item_id, item_attrs, attr_consts);
        self.charge_limit = get_item_charge_limit(item_attrs, attr_consts);
        self.cont_limit = get_item_container_limit(item_attrs, attr_consts);
        self.drone_limit = get_ship_drone_limit(item_attrs, attr_consts);
        // Is item limitable by an appropriate "max group" limit, or cannot be affected at all
        self.max_group_fitted_limited = get_max_group_fitted_limited(item_attrs, attr_consts);
        self.max_group_online_limited = get_max_group_online_limited(item_attrs, attr_consts);
        self.max_group_active_limited = get_max_group_active_limited(item_attrs, attr_consts);
        // Misc
        self.kind = get_item_kind(
            item_grp_id,
            item_cat_id,
            item_attrs,
            item_effects,
            attr_consts,
            effect_consts,
        );
        self.item_ship_kind = get_item_ship_kind(item_cat_id, item_attrs, attr_consts);
        self.max_type_fitted = get_max_type_fitted_count(item_attrs, attr_consts);
        self.overload_td_lvl = get_overload_td_lvl(item_attrs, attr_consts);
        self.sec_zone_limitable = is_sec_zone_limitable(item_attrs, attr_consts);
        self.online_max_sec_class = get_online_max_sec_class(item_attrs, attr_consts);
        self.disallow_vs_ew_immune_tgt = get_disallow_vs_ew_immune_tgt(item_attrs, attr_consts);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RItemAXt {
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        if let Some(attr_rid) = self.remote_resist_attr_rid {
            attr_rid.consistency_check(u_data)?;
        }
        Ok(())
    }
}
