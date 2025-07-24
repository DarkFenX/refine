use super::{
    attr_val::{
        get_bandwidth_use, get_capacity, get_charge_rate, get_charge_size, get_max_type_fitted_count,
        get_online_max_sec_class, get_radius, get_remote_resist_attr_id, get_volume,
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
    overload_td_lvl::get_overload_td_lvl,
    sec_zone::is_sec_zone_limitable,
    ship_kind::get_item_ship_kind,
    ship_limit::get_item_ship_limit,
    slot_index::{get_booster_slot, get_implant_slot, get_subsystem_slot},
};
use crate::{
    ad,
    rd::{RItem, RItemChargeLimit, RItemContLimit, RItemKind, RItemShipLimit, RShipDroneLimit, RShipKind},
    util::RMap,
};

// On-item container for data derived from item attributes. Has to be stored as a separate entity,
// since it has to be regenerated for mutated items, which get their attributes determined only
// during runtime.
#[derive(Clone)]
pub(crate) struct RItemAXt {
    // Item type
    pub(crate) kind: Option<RItemKind>,
    // Unmutated and unmodified item volume
    pub(crate) volume: ad::AAttrVal,
    // Unmutated and unmodified item capacity
    pub(crate) capacity: ad::AAttrVal,
    // Unmutated and unmodified item radius
    pub(crate) radius: ad::AAttrVal,
    // If set, item can be fit to a ship which fits into the limit
    pub(crate) ship_limit: Option<RItemShipLimit>,
    // If set, item can load only charges which fit into limit
    pub(crate) charge_limit: Option<RItemChargeLimit>,
    // If set, item can be loaded as charge into other items which fits this limit
    pub(crate) cont_limit: Option<RItemContLimit>,
    // Slot index an implant takes
    pub(crate) implant_slot: Option<ad::ASlotIndex>,
    // Slot index a booster takes
    pub(crate) booster_slot: Option<ad::ASlotIndex>,
    // Slot index a subsystem takes
    pub(crate) subsystem_slot: Option<ad::ASlotIndex>,
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
    pub(crate) max_fighter_count: ad::ACount,
    // Drone bandwidth consumption
    pub(crate) bandwidth_use: Option<ad::AAttrVal>,
    // Required thermodynamics skill level
    pub(crate) overload_td_lvl: Option<ad::ASkillLevel>,
    // Max amount of items with this type ID a fit can have
    pub(crate) max_type_fitted: Option<ad::ACount>,
    // Max security class this module can be online in (2 hisec, 1 lowsec, 0 the rest)
    pub(crate) online_max_sec_class: Option<ad::AAttrVal>,
    // Can be limited to specific security zones if some of the limit attributes are defined
    pub(crate) sec_zone_limitable: bool,
    // True if assistive item projected to targets immune to offensive modifiers should break the
    // offense immunity validation
    pub(crate) disallow_vs_ew_immune_tgt: bool,
    // Attribute ID which defines how affectee resists effect
    pub(crate) remote_resist_attr_id: Option<ad::AAttrId>,
    // Unmutated and unmodified charge size
    pub(crate) charge_size: Option<ad::AAttrVal>,
    // Unmutated and unmodified charge rate
    pub(crate) charge_rate: ad::ACount,
}
impl RItemAXt {
    // Build extras out of item with its original attributes
    pub(crate) fn new_initial(a_item: &ad::AItem) -> Self {
        Self {
            kind: get_item_kind(a_item.grp_id, a_item.cat_id, &a_item.attrs, &a_item.effect_datas),
            volume: get_volume(&a_item.attrs),
            capacity: get_capacity(&a_item.attrs),
            radius: get_radius(&a_item.attrs),
            ship_limit: get_item_ship_limit(a_item.id, &a_item.attrs),
            charge_limit: get_item_charge_limit(&a_item.attrs),
            cont_limit: get_item_container_limit(&a_item.attrs),
            implant_slot: get_implant_slot(&a_item.attrs),
            booster_slot: get_booster_slot(&a_item.attrs),
            subsystem_slot: get_subsystem_slot(&a_item.attrs),
            is_light_fighter: get_light_fighter_flag(&a_item.attrs),
            is_heavy_fighter: get_heavy_fighter_flag(&a_item.attrs),
            is_support_fighter: get_support_fighter_flag(&a_item.attrs),
            is_st_light_fighter: get_st_light_fighter_flag(&a_item.attrs),
            is_st_heavy_fighter: get_st_heavy_fighter_flag(&a_item.attrs),
            is_st_support_fighter: get_st_support_fighter_flag(&a_item.attrs),
            item_ship_kind: get_item_ship_kind(a_item.cat_id, &a_item.attrs),
            drone_limit: get_ship_drone_limit(&a_item.attrs),
            max_fighter_count: get_max_fighter_count(&a_item.attrs),
            bandwidth_use: get_bandwidth_use(&a_item.attrs),
            overload_td_lvl: get_overload_td_lvl(&a_item.attrs),
            max_type_fitted: get_max_type_fitted_count(&a_item.attrs),
            online_max_sec_class: get_online_max_sec_class(&a_item.attrs),
            sec_zone_limitable: is_sec_zone_limitable(&a_item.attrs),
            disallow_vs_ew_immune_tgt: get_disallow_vs_ew_immune_tgt(&a_item.attrs),
            remote_resist_attr_id: get_remote_resist_attr_id(&a_item.attrs),
            charge_size: get_charge_size(&a_item.attrs),
            charge_rate: get_charge_rate(&a_item.attrs),
        }
    }
    // Build extras out of item with overridden attributes
    pub(crate) fn new_inherited(r_item: &RItem, attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> Self {
        Self {
            kind: get_item_kind(
                r_item.get_group_id(),
                r_item.get_category_id(),
                attrs,
                r_item.get_effect_datas_ids(),
            ),
            volume: get_volume(attrs),
            capacity: get_capacity(attrs),
            radius: get_radius(attrs),
            ship_limit: get_item_ship_limit(r_item.get_id(), attrs),
            charge_limit: get_item_charge_limit(attrs),
            cont_limit: get_item_container_limit(attrs),
            implant_slot: get_implant_slot(attrs),
            booster_slot: get_booster_slot(attrs),
            subsystem_slot: get_subsystem_slot(attrs),
            is_light_fighter: get_light_fighter_flag(attrs),
            is_heavy_fighter: get_heavy_fighter_flag(attrs),
            is_support_fighter: get_support_fighter_flag(attrs),
            is_st_light_fighter: get_st_light_fighter_flag(attrs),
            is_st_heavy_fighter: get_st_heavy_fighter_flag(attrs),
            is_st_support_fighter: get_st_support_fighter_flag(attrs),
            item_ship_kind: get_item_ship_kind(r_item.get_category_id(), attrs),
            drone_limit: get_ship_drone_limit(attrs),
            max_fighter_count: get_max_fighter_count(attrs),
            bandwidth_use: get_bandwidth_use(attrs),
            overload_td_lvl: get_overload_td_lvl(attrs),
            max_type_fitted: get_max_type_fitted_count(attrs),
            online_max_sec_class: get_online_max_sec_class(attrs),
            sec_zone_limitable: is_sec_zone_limitable(attrs),
            disallow_vs_ew_immune_tgt: get_disallow_vs_ew_immune_tgt(attrs),
            remote_resist_attr_id: get_remote_resist_attr_id(attrs),
            charge_size: get_charge_size(attrs),
            charge_rate: get_charge_rate(attrs),
        }
    }
}
