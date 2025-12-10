use crate::{
    ad::{AAttrId, AAttrVal, ACount, AItemCatId, AItemGrpId, AItemId, ASkillLevel, ASlotIndex},
    misc::ItemKind,
    rd::{
        RAttrConsts, RAttrKey, REffectConsts, REffectKey, RItemChargeLimit, RItemContLimit, RItemEffectData,
        RItemShipLimit, RShipDroneLimit, RShipKind,
        data::item::attr_extras::info::{
            attr_val::{
                get_bandwidth_use, get_calibration_use, get_capacity, get_charge_rate, get_charge_size,
                get_max_fighter_count, get_max_type_fitted_count, get_online_max_sec_class, get_overload_td_lvl,
                get_radius, get_remote_resist_attr_id, get_rig_size, get_volume,
            },
            charge_limit::get_item_charge_limit,
            container_limit::get_item_container_limit,
            drone_limit::get_ship_drone_limit,
            effect_immunity::get_disallow_vs_ew_immune_tgt,
            fighter_kind::{
                get_heavy_fighter_flag, get_light_fighter_flag, get_st_heavy_fighter_flag, get_st_light_fighter_flag,
                get_st_support_fighter_flag, get_support_fighter_flag,
            },
            kind::get_item_kind,
            max_group::{get_max_group_active_limited, get_max_group_fitted_limited, get_max_group_online_limited},
            mobility::is_mobile,
            sec_zone::is_sec_zone_limitable,
            ship_kind::get_item_ship_kind,
            ship_limit::get_item_ship_limit,
            slot_index::{get_booster_slot, get_implant_slot, get_subsystem_slot},
        },
    },
    util::RMap,
};

// On-item container for data derived from item attributes. Has to be stored as a separate entity,
// since it has to be regenerated for mutated items, which get their attributes determined only
// during runtime.
#[derive(Clone, Default)]
pub(crate) struct RItemAXt {
    // Item type
    pub(crate) kind: Option<ItemKind>,
    // Unmutated and unmodified item volume
    pub(crate) volume: AAttrVal,
    // Unmutated and unmodified item capacity
    pub(crate) capacity: AAttrVal,
    // Unmutated and unmodified item radius
    pub(crate) radius: AAttrVal,
    // If set, item can be fit to a ship which fits into the limit
    pub(crate) ship_limit: Option<RItemShipLimit>,
    // If set, item can load only charges which fit into limit
    pub(crate) charge_limit: Option<RItemChargeLimit>,
    // If set, item can be loaded as charge into other items which fits this limit
    pub(crate) cont_limit: Option<RItemContLimit>,
    // Slot index an implant takes
    pub(crate) implant_slot: Option<ASlotIndex>,
    // Slot index a booster takes
    pub(crate) booster_slot: Option<ASlotIndex>,
    // Slot index a subsystem takes
    pub(crate) subsystem_slot: Option<ASlotIndex>,
    // Defines if a fighter takes a light fighter slot or not
    pub(crate) is_light_fighter: bool,
    // Defines if a fighter takes a heavy fighter slot or not
    pub(crate) is_heavy_fighter: bool,
    // Defines if a fighter takes a support fighter slot or not
    pub(crate) is_support_fighter: bool,
    // Defines if a fighter takes a standup light fighter slot or not
    pub(crate) is_st_light_fighter: bool,
    // Defines if a fighter takes a standup heavy fighter slot or not
    pub(crate) is_st_heavy_fighter: bool,
    // Defines if a fighter takes a standup support fighter slot or not
    pub(crate) is_st_support_fighter: bool,
    // Which ship type this item fits to
    pub(crate) item_ship_kind: Option<RShipKind>,
    // If set, ship can use drones which fit into the limit
    pub(crate) drone_limit: Option<RShipDroneLimit>,
    // By default, a fighter squad will have this count of fighters
    pub(crate) max_fighter_count: ACount,
    // Drone bandwidth consumption
    pub(crate) bandwidth_use: Option<AAttrVal>,
    // Required thermodynamics skill level
    pub(crate) overload_td_lvl: Option<ASkillLevel>,
    // Max amount of items with this type ID a fit can have
    pub(crate) max_type_fitted: Option<ACount>,
    // Max security class this module can be online in (2 hisec, 1 lowsec, 0 the rest)
    pub(crate) online_max_sec_class: Option<AAttrVal>,
    // Can be limited to specific security zones if some of the limit attributes are defined
    pub(crate) sec_zone_limitable: bool,
    // True if assistive item projected to targets immune to offensive modifiers should break the
    // offense immunity validation
    pub(crate) disallow_vs_ew_immune_tgt: bool,
    // Attribute key which defines how affectee resists effect
    pub(crate) remote_resist_attr_key: Option<RAttrKey>,
    // Unmutated and unmodified charge size
    pub(crate) charge_size: Option<AAttrVal>,
    // Unmutated and unmodified charge rate
    pub(crate) charge_rate: ACount,
    // True if item has some speed
    pub(crate) is_mobile: bool,
    // Rig calibration cost
    pub(crate) calibration_use: Option<AAttrVal>,
    // Can item be limited by "max group fitted" limit
    pub(crate) max_group_fitted_limited: bool,
    // Can item be limited by "max group online" limit
    pub(crate) max_group_online_limited: bool,
    // Can item be limited by "max group active" limit
    pub(crate) max_group_active_limited: bool,
    // Size of a rig, or rig size used by a ship
    pub(crate) rig_size: Option<AAttrVal>,
}
impl RItemAXt {
    pub(crate) fn fill(
        &mut self,
        item_id: AItemId,
        item_grp_id: AItemGrpId,
        item_cat_id: AItemCatId,
        item_attrs: &RMap<RAttrKey, AAttrVal>,
        item_effects: &RMap<REffectKey, RItemEffectData>,
        attr_id_key_map: &RMap<AAttrId, RAttrKey>,
        attr_consts: &RAttrConsts,
        effect_consts: &REffectConsts,
    ) {
        self.kind = get_item_kind(
            item_grp_id,
            item_cat_id,
            item_attrs,
            item_effects,
            attr_consts,
            effect_consts,
        );
        self.volume = get_volume(item_attrs, attr_consts);
        self.capacity = get_capacity(item_attrs, attr_consts);
        self.radius = get_radius(item_attrs, attr_consts);
        self.ship_limit = get_item_ship_limit(item_id, item_attrs, attr_consts);
        self.charge_limit = get_item_charge_limit(item_attrs, attr_consts);
        self.cont_limit = get_item_container_limit(item_attrs, attr_consts);
        self.implant_slot = get_implant_slot(item_attrs, attr_consts);
        self.booster_slot = get_booster_slot(item_attrs, attr_consts);
        self.subsystem_slot = get_subsystem_slot(item_attrs, attr_consts);
        self.is_light_fighter = get_light_fighter_flag(item_attrs, attr_consts);
        self.is_heavy_fighter = get_heavy_fighter_flag(item_attrs, attr_consts);
        self.is_support_fighter = get_support_fighter_flag(item_attrs, attr_consts);
        self.is_st_light_fighter = get_st_light_fighter_flag(item_attrs, attr_consts);
        self.is_st_heavy_fighter = get_st_heavy_fighter_flag(item_attrs, attr_consts);
        self.is_st_support_fighter = get_st_support_fighter_flag(item_attrs, attr_consts);
        self.item_ship_kind = get_item_ship_kind(item_cat_id, item_attrs, attr_consts);
        self.drone_limit = get_ship_drone_limit(item_attrs, attr_consts);
        self.max_fighter_count = get_max_fighter_count(item_attrs, attr_consts);
        self.bandwidth_use = get_bandwidth_use(item_attrs, attr_consts);
        self.overload_td_lvl = get_overload_td_lvl(item_attrs, attr_consts);
        self.max_type_fitted = get_max_type_fitted_count(item_attrs, attr_consts);
        self.online_max_sec_class = get_online_max_sec_class(item_attrs, attr_consts);
        self.sec_zone_limitable = is_sec_zone_limitable(item_attrs, attr_consts);
        self.disallow_vs_ew_immune_tgt = get_disallow_vs_ew_immune_tgt(item_attrs, attr_consts);
        self.remote_resist_attr_key = get_remote_resist_attr_id(item_attrs, attr_consts, attr_id_key_map);
        self.charge_size = get_charge_size(item_attrs, attr_consts);
        self.charge_rate = get_charge_rate(item_attrs, attr_consts);
        self.is_mobile = is_mobile(item_attrs, attr_consts);
        self.calibration_use = get_calibration_use(item_attrs, attr_consts);
        self.max_group_fitted_limited = get_max_group_fitted_limited(item_attrs, attr_consts);
        self.max_group_online_limited = get_max_group_online_limited(item_attrs, attr_consts);
        self.max_group_active_limited = get_max_group_active_limited(item_attrs, attr_consts);
        self.rig_size = get_rig_size(item_attrs, attr_consts);
    }
}
