use super::getters::{
    activation_blocks::{get_activation_blocks_cloak, get_activation_blocks_in_assist},
    attr_val::{
        get_calibration_use, get_capacity, get_charge_rate, get_charge_size, get_fighter_refuel_duration,
        get_max_fighter_count, get_max_type_fitted_count, get_online_max_sec_class, get_radius, get_rig_size,
        get_volume,
    },
    charge_limit::get_item_charge_limit,
    container_limit::get_item_container_limit,
    drone_limit::get_ship_drone_limit,
    effect_immunity::get_disallow_vs_ew_immune_tgt,
    fighter_kind::{
        get_heavy_fighter_flag, get_light_fighter_flag, get_st_heavy_fighter_flag, get_st_light_fighter_flag,
        get_st_support_fighter_flag, get_support_fighter_flag,
    },
    has_effect::{has_launcher_effect, has_online_effect, has_turret_effect},
    kind::detect_item_kind,
    mobility::{get_enables_conduit, get_enables_portal_from_attrs, get_jump_fuel_type_id},
    sec_zone::is_sec_zone_limitable,
    ship_kind::{get_item_ship_kind, get_ship_kind},
    ship_limit::get_item_ship_limit,
    slot_index::{get_booster_slot, get_implant_slot, get_subsystem_slot},
};
use crate::{
    Count, CountNz, PValue, SkillLevel, SlotIndex, Value,
    ad::{AAbilId, AAttrId, AEffectId, AItem, AItemCatId, AItemGrpId, AItemId, AItemListId},
    misc::DetectedItemKind,
    rd::{
        RAttrConsts, RAttrId, REffectConsts, REffectId, RItemCapConsumer, RItemChargeLimit, RItemContLimit,
        RItemEffectData, RItemListId, RItemShipLimit, RShipDroneLimit, RShipKind, RState, RcEffect,
    },
    util::{PSlab, RMap},
};

/// Item base data - the data which does not depend on anything outside the item itself.
///
/// The distinction with flex data is important when mutated items come into play. Mutated items can
/// combine  attributes from multiple items. For mutated items, base data stores only data derived
/// from mutated item type itself; any changes from base item are not considered.
pub(crate) struct RItemBase {
    // Raw data
    pub(crate) aid: AItemId,
    pub(crate) grp_id: AItemGrpId,
    pub(crate) cat_id: AItemCatId,
    pub(crate) effects: RMap<REffectId, RItemEffectData>,
    pub(crate) defeff_rid: Option<REffectId>,
    pub(crate) abil_ids: Vec<AAbilId>,
    pub(crate) srqs: RMap<AItemId, SkillLevel>,
    // Derived data - item type flags
    pub(crate) is_cloak: bool,
    pub(crate) is_ice_harvester: bool,
    // Derived data - effect flags
    pub(crate) has_online_effect: bool,
    pub(crate) takes_turret_hardpoint: bool,
    pub(crate) takes_launcher_hardpoint: bool,
    // Derived data - max group ID validation data
    pub(crate) val_fitted_group_id: Option<AItemGrpId>,
    pub(crate) val_online_group_id: Option<AItemGrpId>,
    pub(crate) val_active_group_id: Option<AItemGrpId>,
    // Derived data - buff item list IDs. Those are intentionally vectors: they are used for
    // iteration, and for membership checks. For cases where it matters (e.g. non-ship items) those
    // do not have more than a couple of entries, so are faster than sets.
    pub(crate) proj_buff_item_list_rids: Vec<RItemListId>,
    pub(crate) fleet_buff_item_list_rids: Vec<RItemListId>,
    // Derived data - misc
    pub(crate) max_state: RState,
    pub(crate) cap_consumers: Vec<RItemCapConsumer>,
    pub(crate) ship_kind: Option<RShipKind>,
    pub(crate) disallowed_in_wspace: bool,
    // Derived data - base item attribute values, cast to necessary type
    pub(crate) volume: PValue,
    pub(crate) capacity: PValue,
    pub(crate) radius: PValue,
    pub(crate) calibration_use: Option<Value>,
    /// On-rig and on-ship attribute
    pub(crate) rig_size: Option<Value>,
    /// On-module and on-charge attribute
    pub(crate) charge_size: Option<Value>,
    pub(crate) charge_rate: Count,
    pub(crate) max_fighter_count: CountNz,
    pub(crate) fighter_refuel_duration: PValue,
    // Derived data - fighter kind flags
    pub(crate) is_light_fighter: bool,
    pub(crate) is_heavy_fighter: bool,
    pub(crate) is_support_fighter: bool,
    pub(crate) is_st_light_fighter: bool,
    pub(crate) is_st_heavy_fighter: bool,
    pub(crate) is_st_support_fighter: bool,
    // Derived data - mobility
    pub(crate) jump_fuel_item_aid: Option<AItemId>,
    pub(crate) enables_conduit: bool,
    /// Used by ansiblex service (which comes from adapted data) and modules (from attributes)
    pub(crate) enables_portal: bool,
    // Derived data - slot index this item takes
    pub(crate) implant_slot: Option<SlotIndex>,
    pub(crate) booster_slot: Option<SlotIndex>,
    pub(crate) subsystem_slot: Option<SlotIndex>,
    // Derived data - various aggregated limits
    /// Items can be fit to those ships
    pub(crate) ship_limit: Option<RItemShipLimit>,
    /// Items can load those charges
    pub(crate) charge_limit: Option<RItemChargeLimit>,
    /// Charges can be loaded into those items
    pub(crate) cont_limit: Option<RItemContLimit>,
    /// Ship can use those drones
    pub(crate) drone_limit: Option<RShipDroneLimit>,
    // Derived data - self-limits
    /// Max amount of fit items of this type ID
    pub(crate) max_type_fitted: Option<Count>,
    /// If item can be sec zone limited altogether
    pub(crate) sec_zone_limitable: bool,
    /// 2 hisec, 1 lowsec, 0 the rest
    pub(crate) online_max_sec_class: Option<Value>,
    pub(crate) disallow_vs_ew_immune_tgt: bool,
    // Derived data - ship limits
    pub(crate) activation_blocks_cloak: bool,
    pub(crate) activation_blocks_in_assist: bool,
    // Derived data - misc
    pub(crate) detected_kind: Option<DetectedItemKind>,
    /// Which ship type this item fits to
    pub(crate) item_ship_kind: Option<RShipKind>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RItemBase {
    pub(in crate::rd::data::item) fn from_a_item(a_item: &AItem) -> Self {
        Self {
            aid: a_item.id,
            grp_id: a_item.grp_id,
            cat_id: a_item.cat_id,
            abil_ids: a_item.abil_ids.iter().copied().collect(),
            srqs: a_item
                .srqs
                .iter()
                .map(|a_skill_req| (a_skill_req.id, SkillLevel::from_a_skill_level(a_skill_req.level)))
                .collect(),
            is_cloak: a_item.is_cloak,
            is_ice_harvester: a_item.is_ice_harvester,
            enables_portal: a_item.enables_portal,
            val_fitted_group_id: a_item.val_fitted_group_id,
            val_online_group_id: a_item.val_online_group_id,
            val_active_group_id: a_item.val_active_group_id,
            max_state: RState::from_a_state(&a_item.max_state),
            disallowed_in_wspace: a_item.disallowed_in_wspace,
            // Fields which depend on data not available on adapted item
            effects: Default::default(),
            defeff_rid: Default::default(),
            has_online_effect: Default::default(),
            takes_turret_hardpoint: Default::default(),
            takes_launcher_hardpoint: Default::default(),
            proj_buff_item_list_rids: Default::default(),
            fleet_buff_item_list_rids: Default::default(),
            cap_consumers: Default::default(),
            ship_kind: Default::default(),
            // Same, but also rely on attributes
            volume: Default::default(),
            capacity: Default::default(),
            radius: Default::default(),
            calibration_use: Default::default(),
            rig_size: Default::default(),
            charge_size: Default::default(),
            charge_rate: Default::default(),
            max_fighter_count: Default::default(),
            fighter_refuel_duration: Default::default(),
            is_light_fighter: Default::default(),
            is_heavy_fighter: Default::default(),
            is_support_fighter: Default::default(),
            is_st_light_fighter: Default::default(),
            is_st_heavy_fighter: Default::default(),
            is_st_support_fighter: Default::default(),
            jump_fuel_item_aid: Default::default(),
            enables_conduit: Default::default(),
            implant_slot: Default::default(),
            booster_slot: Default::default(),
            subsystem_slot: Default::default(),
            ship_limit: Default::default(),
            charge_limit: Default::default(),
            cont_limit: Default::default(),
            drone_limit: Default::default(),
            max_type_fitted: Default::default(),
            sec_zone_limitable: Default::default(),
            online_max_sec_class: Default::default(),
            disallow_vs_ew_immune_tgt: Default::default(),
            activation_blocks_cloak: Default::default(),
            activation_blocks_in_assist: Default::default(),
            detected_kind: Default::default(),
            item_ship_kind: Default::default(),
        }
    }
    pub(in crate::rd::data::item) fn fill_runtime_basic(
        &mut self,
        a_items: &RMap<AItemId, AItem>,
        item_list_aid_rid_map: &RMap<AItemListId, RItemListId>,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
        effect_aid_rid_map: &RMap<AEffectId, REffectId>,
        r_effects: &PSlab<REffectId, RcEffect>,
    ) {
        let a_item = a_items.get(&self.aid).unwrap();
        for a_item_effect in a_item.effects.iter() {
            let Some(&effect_rid) = effect_aid_rid_map.get(&a_item_effect.id) else {
                continue;
            };
            let r_effect_data = RItemEffectData::from_a_effect_data(&a_item_effect.data, attr_aid_rid_map);
            self.effects.insert(effect_rid, r_effect_data);
        }
        self.defeff_rid = a_item
            .defeff_id
            .and_then(|defeff_aid| effect_aid_rid_map.get(&defeff_aid).copied());
        self.has_online_effect = has_online_effect(&self.effects, effect_aid_rid_map);
        self.takes_turret_hardpoint = has_turret_effect(&self.effects, effect_aid_rid_map);
        self.takes_launcher_hardpoint = has_launcher_effect(&self.effects, effect_aid_rid_map);
        self.proj_buff_item_list_rids.extend(
            a_item
                .proj_buff_item_list_ids
                .iter()
                .filter_map(|item_list_aid| item_list_aid_rid_map.get(item_list_aid).copied()),
        );
        self.fleet_buff_item_list_rids.extend(
            a_item
                .fleet_buff_item_list_ids
                .iter()
                .filter_map(|item_list_aid| item_list_aid_rid_map.get(item_list_aid).copied()),
        );

        for &effect_rid in self.effects.keys() {
            let r_effect = r_effects.get(effect_rid).unwrap();
            if let Some(opc_spec) = r_effect.cap_consume {
                self.cap_consumers.push(RItemCapConsumer { effect_rid, opc_spec })
            }
        }
        self.ship_kind = get_ship_kind(self.cat_id, &self.srqs);
    }
    pub(in crate::rd::data::item) fn fill_runtime_extended(
        &mut self,
        attrs: &RMap<RAttrId, Value>,
        attr_consts: &RAttrConsts,
        effect_consts: &REffectConsts,
    ) {
        // Base item attribute values
        self.volume = get_volume(attrs, attr_consts);
        self.capacity = get_capacity(attrs, attr_consts);
        self.radius = get_radius(attrs, attr_consts);
        self.calibration_use = get_calibration_use(attrs, attr_consts);
        self.rig_size = get_rig_size(attrs, attr_consts);
        self.charge_size = get_charge_size(attrs, attr_consts);
        self.charge_rate = get_charge_rate(attrs, attr_consts);
        self.max_fighter_count = get_max_fighter_count(attrs, attr_consts);
        self.fighter_refuel_duration = get_fighter_refuel_duration(attrs, attr_consts);
        // Mobility
        self.jump_fuel_item_aid = get_jump_fuel_type_id(attrs, attr_consts);
        self.enables_conduit = get_enables_conduit(attrs, attr_consts);
        self.enables_portal = self.enables_portal || get_enables_portal_from_attrs(attrs, attr_consts);
        // Fighter kind flags
        self.is_light_fighter = get_light_fighter_flag(attrs, attr_consts);
        self.is_heavy_fighter = get_heavy_fighter_flag(attrs, attr_consts);
        self.is_support_fighter = get_support_fighter_flag(attrs, attr_consts);
        self.is_st_light_fighter = get_st_light_fighter_flag(attrs, attr_consts);
        self.is_st_heavy_fighter = get_st_heavy_fighter_flag(attrs, attr_consts);
        self.is_st_support_fighter = get_st_support_fighter_flag(attrs, attr_consts);
        // Slot index this item takes
        self.implant_slot = get_implant_slot(attrs, attr_consts);
        self.booster_slot = get_booster_slot(attrs, attr_consts);
        self.subsystem_slot = get_subsystem_slot(attrs, attr_consts);
        // Various aggregated limits
        self.ship_limit = get_item_ship_limit(self.aid, attrs, attr_consts);
        self.charge_limit = get_item_charge_limit(attrs, attr_consts);
        self.cont_limit = get_item_container_limit(attrs, attr_consts);
        self.drone_limit = get_ship_drone_limit(attrs, attr_consts);
        // Self-limits
        self.max_type_fitted = get_max_type_fitted_count(attrs, attr_consts);
        self.sec_zone_limitable = is_sec_zone_limitable(attrs, attr_consts);
        self.online_max_sec_class = get_online_max_sec_class(attrs, attr_consts);
        self.disallow_vs_ew_immune_tgt = get_disallow_vs_ew_immune_tgt(attrs, attr_consts);
        // Ship limits
        self.activation_blocks_cloak = get_activation_blocks_cloak(attrs, attr_consts);
        self.activation_blocks_in_assist = get_activation_blocks_in_assist(attrs, attr_consts);
        // Misc
        self.detected_kind = detect_item_kind(
            self.grp_id,
            self.cat_id,
            attrs,
            &self.effects,
            attr_consts,
            effect_consts,
        );
        self.item_ship_kind = get_item_ship_kind(self.cat_id, attrs, attr_consts);
    }
}
