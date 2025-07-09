use super::{
    attr_val::{
        get_bandwidth_use, get_capacity, get_charge_size, get_max_type_fitted_count, get_online_max_sec_class,
        get_radius, get_remote_resist_attr_id, get_volume,
    },
    charge_limit::get_item_charge_limit,
    container_limit::get_item_container_limit,
    drone_limit::get_ship_drone_limit,
    effect_immunity::get_disallow_vs_ew_immune_tgt,
    fighter_count::get_max_fighter_count,
    fighter_kind::{
        get_heavy_fighter_flag, get_light_fighter_flag, get_st_heavy_fighter_flag, get_st_light_fighter_flag,
        get_st_support_fighter_flag, get_support_fighter_flag,
    },
    kind::get_item_kind,
    module_hardpoint::{is_launcher, is_turret},
    overload_td_lvl::get_overload_td_lvl,
    sec_zone::is_sec_zone_limitable,
    ship_kind::{get_item_ship_kind, get_ship_kind},
    ship_limit::get_item_ship_limit,
    slot_index::{get_booster_slot, get_implant_slot, get_subsystem_slot},
};
use crate::{
    ad::{
        AAttrId, AAttrVal, ACount, AItem, AItemChargeLimit, AItemContainerLimit, AItemKind, AItemRt, AItemShipLimit,
        AShipDroneLimit, AShipKind, ASkillLevel, ASlotIndex,
    },
    util::RMap,
};

// Extra data attached to item during runtime
#[derive(Clone)]
pub(crate) struct AItemXt {
    // Item type
    pub(crate) kind: Option<AItemKind>,
    // Unmutated and unmodified item volume
    pub(crate) volume: AAttrVal,
    // Unmutated and unmodified item capacity
    pub(crate) capacity: AAttrVal,
    // Unmutated and unmodified item radius
    pub(crate) radius: AAttrVal,
    // If set, item can be fit to a ship which fits into the limit
    pub(crate) ship_limit: Option<AItemShipLimit>,
    // If set, item can load only charges which fit into limit
    pub(crate) charge_limit: Option<AItemChargeLimit>,
    // If set, item can be loaded as charge into other items which fits this limit
    pub(crate) container_limit: Option<AItemContainerLimit>,
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
    // Ship type
    pub(crate) ship_kind: Option<AShipKind>,
    // Which ship type this item fits to
    pub(crate) item_ship_kind: Option<AShipKind>,
    // If set, ship can use drones which fit into the limit
    pub(crate) drone_limit: Option<AShipDroneLimit>,
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
    // True if item has turretFitted effect
    pub(crate) takes_turret_hardpoint: bool,
    // True if item has launcherFitted effect
    pub(crate) takes_launcher_hardpoint: bool,
    // True if assistive item projected to targets immune to offensive modifiers should break the
    // offense immunity validation
    pub(crate) disallow_vs_ew_immune_tgt: bool,
    // Attribute ID which defines how affectee resists effect
    pub(crate) remote_resist_attr_id: Option<AAttrId>,
    // Unmutated and unmodified item volume
    pub(crate) charge_size: Option<AAttrVal>,
}
impl AItemXt {
    // Build extras out of item with its original attributes
    pub(crate) fn new_initial(a_item: &AItem) -> Self {
        Self {
            kind: get_item_kind(a_item.grp_id, a_item.cat_id, &a_item.attrs, &a_item.effect_datas),
            volume: get_volume(&a_item.attrs),
            capacity: get_capacity(&a_item.attrs),
            radius: get_radius(&a_item.attrs),
            ship_limit: get_item_ship_limit(a_item.id, &a_item.attrs),
            charge_limit: get_item_charge_limit(&a_item.attrs),
            container_limit: get_item_container_limit(&a_item.attrs),
            implant_slot: get_implant_slot(&a_item.attrs),
            booster_slot: get_booster_slot(&a_item.attrs),
            subsystem_slot: get_subsystem_slot(&a_item.attrs),
            is_light_fighter: get_light_fighter_flag(&a_item.attrs),
            is_heavy_fighter: get_heavy_fighter_flag(&a_item.attrs),
            is_support_fighter: get_support_fighter_flag(&a_item.attrs),
            is_st_light_fighter: get_st_light_fighter_flag(&a_item.attrs),
            is_st_heavy_fighter: get_st_heavy_fighter_flag(&a_item.attrs),
            is_st_support_fighter: get_st_support_fighter_flag(&a_item.attrs),
            ship_kind: get_ship_kind(a_item.cat_id, &a_item.srqs),
            item_ship_kind: get_item_ship_kind(a_item.cat_id, &a_item.attrs),
            drone_limit: get_ship_drone_limit(&a_item.attrs),
            max_fighter_count: get_max_fighter_count(&a_item.attrs),
            bandwidth_use: get_bandwidth_use(&a_item.attrs),
            overload_td_lvl: get_overload_td_lvl(&a_item.attrs),
            max_type_fitted: get_max_type_fitted_count(&a_item.attrs),
            online_max_sec_class: get_online_max_sec_class(&a_item.attrs),
            sec_zone_limitable: is_sec_zone_limitable(&a_item.attrs),
            takes_turret_hardpoint: is_turret(&a_item.effect_datas),
            takes_launcher_hardpoint: is_launcher(&a_item.effect_datas),
            disallow_vs_ew_immune_tgt: get_disallow_vs_ew_immune_tgt(&a_item.attrs),
            remote_resist_attr_id: get_remote_resist_attr_id(&a_item.attrs),
            charge_size: get_charge_size(&a_item.attrs),
        }
    }
    // Build extras out of item with overridden attributes
    pub(crate) fn new_inherited(a_item: &AItemRt, attrs: &RMap<AAttrId, AAttrVal>) -> Self {
        Self {
            kind: get_item_kind(a_item.ai.grp_id, a_item.ai.cat_id, attrs, &a_item.ai.effect_datas),
            volume: get_volume(attrs),
            capacity: get_capacity(attrs),
            radius: get_radius(attrs),
            ship_limit: get_item_ship_limit(a_item.ai.id, attrs),
            charge_limit: get_item_charge_limit(attrs),
            container_limit: get_item_container_limit(attrs),
            implant_slot: get_implant_slot(attrs),
            booster_slot: get_booster_slot(attrs),
            subsystem_slot: get_subsystem_slot(attrs),
            is_light_fighter: get_light_fighter_flag(attrs),
            is_heavy_fighter: get_heavy_fighter_flag(attrs),
            is_support_fighter: get_support_fighter_flag(attrs),
            is_st_light_fighter: get_st_light_fighter_flag(attrs),
            is_st_heavy_fighter: get_st_heavy_fighter_flag(attrs),
            is_st_support_fighter: get_st_support_fighter_flag(attrs),
            ship_kind: a_item.xt.ship_kind,
            item_ship_kind: get_item_ship_kind(a_item.ai.cat_id, attrs),
            drone_limit: get_ship_drone_limit(attrs),
            max_fighter_count: get_max_fighter_count(attrs),
            bandwidth_use: get_bandwidth_use(attrs),
            overload_td_lvl: get_overload_td_lvl(attrs),
            max_type_fitted: get_max_type_fitted_count(attrs),
            online_max_sec_class: get_online_max_sec_class(attrs),
            sec_zone_limitable: is_sec_zone_limitable(attrs),
            takes_turret_hardpoint: is_turret(&a_item.ai.effect_datas),
            takes_launcher_hardpoint: is_launcher(&a_item.ai.effect_datas),
            disallow_vs_ew_immune_tgt: get_disallow_vs_ew_immune_tgt(attrs),
            remote_resist_attr_id: get_remote_resist_attr_id(attrs),
            charge_size: get_charge_size(attrs),
        }
    }
}
