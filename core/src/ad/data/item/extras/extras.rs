use crate::{
    ad::{
        AEffect, AItem, AItemChargeLimit, AItemEffectData, AItemKind, AItemShipLimit, AShipDroneLimit, AShipKind,
        AState,
    },
    defs::{AttrVal, Count, EAttrId, EEffectId, EItemCatId, EItemGrpId, EItemId, SkillLevel, SlotIndex},
    util::StMap,
};

use super::{
    charge_limit::get_item_charge_limit,
    drone_limit::get_ship_drone_limit,
    fighter_count::get_max_fighter_count,
    fighter_kind::{
        get_heavy_fighter_flag, get_light_fighter_flag, get_standup_heavy_fighter_flag, get_standup_light_fighter_flag,
        get_standup_support_fighter_flag, get_support_fighter_flag,
    },
    kind::get_item_kind,
    max_state::get_max_state,
    ship_kind::{get_item_ship_kind, get_ship_kind},
    ship_limit::get_item_ship_limit,
    slot_index::{get_booster_slot, get_implant_slot, get_subsystem_slot},
    volume::get_item_volume,
};

/// Holds extra item-specific data.
///
/// It is derived from data normally available on item and other entities, but is calculated on
/// cache generation time for optimization purposes.
#[derive(Clone)]
pub struct AItemExtras {
    /// Item type.
    pub kind: Option<AItemKind>,
    /// Unmodified and unmutated item volume.
    pub volume: Option<AttrVal>,
    /// If set, item can be fit to a ship which fits into the limit.
    pub ship_limit: Option<AItemShipLimit>,
    /// If set, item can load only charges which fit into limit.
    pub charge_limit: Option<AItemChargeLimit>,
    /// Item effectively has this group ID for purposes of "max group fitted" validation.
    pub val_fitted_group_id: Option<EItemGrpId>,
    /// Item effectively has this group ID for purposes of "max group online" validation.
    pub val_online_group_id: Option<EItemGrpId>,
    /// Item effectively has this group ID for purposes of "max group active" validation.
    pub val_active_group_id: Option<EItemGrpId>,
    /// Slot index an implant takes.
    pub implant_slot: Option<SlotIndex>,
    /// Slot index a booster takes.
    pub booster_slot: Option<SlotIndex>,
    /// Slot index a subsystem takes.
    pub subsystem_slot: Option<SlotIndex>,
    /// Defines if a fighter take a light fighter slot or not.
    pub is_light_fighter: bool,
    /// Defines if a fighter take a heavy fighter slot or not.
    pub is_heavy_fighter: bool,
    /// Defines if a fighter take a support fighter slot or not.
    pub is_support_fighter: bool,
    /// Defines if a fighter take a standup light fighter slot or not.
    pub is_standup_light_fighter: bool,
    /// Defines if a fighter take a standup heavy fighter slot or not.
    pub is_standup_heavy_fighter: bool,
    /// Defines if a fighter take a standup support fighter slot or not.
    pub is_standup_support_fighter: bool,
    /// Ship type.
    pub ship_kind: Option<AShipKind>,
    /// Which ship type this item fits to.
    pub item_ship_kind: Option<AShipKind>,
    /// Max state item can take.
    pub max_state: AState,
    /// If set, ship can use drones which fit into the limit.
    pub drone_limit: Option<AShipDroneLimit>,
    /// By default, a fighter squad will have this count of fighters.
    pub max_fighter_count: Count,
}
impl AItemExtras {
    pub(crate) fn new() -> Self {
        Self {
            kind: Option::default(),
            volume: Option::default(),
            ship_limit: Option::default(),
            charge_limit: Option::default(),
            val_fitted_group_id: Option::default(),
            val_online_group_id: Option::default(),
            val_active_group_id: Option::default(),
            implant_slot: Option::default(),
            booster_slot: Option::default(),
            subsystem_slot: Option::default(),
            is_light_fighter: bool::default(),
            is_heavy_fighter: bool::default(),
            is_support_fighter: bool::default(),
            is_standup_light_fighter: bool::default(),
            is_standup_heavy_fighter: bool::default(),
            is_standup_support_fighter: bool::default(),
            ship_kind: Option::default(),
            item_ship_kind: Option::default(),
            max_state: AState::Offline,
            drone_limit: Option::default(),
            max_fighter_count: 1,
        }
    }
    // Build new instance, rebuilding all the data based on new attributes, copying data which does
    // not rely on them
    pub(crate) fn inherit_with_attrs(a_item: &AItem, attrs: &StMap<EAttrId, AttrVal>) -> Self {
        Self {
            kind: get_item_kind(a_item.grp_id, a_item.cat_id, attrs, &a_item.effect_datas),
            volume: get_item_volume(attrs),
            ship_limit: get_item_ship_limit(attrs),
            charge_limit: get_item_charge_limit(attrs),
            val_fitted_group_id: a_item.extras.val_fitted_group_id,
            val_online_group_id: a_item.extras.val_online_group_id,
            val_active_group_id: a_item.extras.val_active_group_id,
            implant_slot: get_implant_slot(attrs),
            booster_slot: get_booster_slot(attrs),
            subsystem_slot: get_subsystem_slot(attrs),
            is_light_fighter: get_light_fighter_flag(attrs),
            is_heavy_fighter: get_heavy_fighter_flag(attrs),
            is_support_fighter: get_support_fighter_flag(attrs),
            is_standup_light_fighter: get_standup_light_fighter_flag(attrs),
            is_standup_heavy_fighter: get_standup_heavy_fighter_flag(attrs),
            is_standup_support_fighter: get_standup_support_fighter_flag(attrs),
            ship_kind: a_item.extras.ship_kind,
            item_ship_kind: get_item_ship_kind(a_item.cat_id, attrs),
            max_state: a_item.extras.max_state,
            drone_limit: get_ship_drone_limit(attrs),
            max_fighter_count: get_max_fighter_count(attrs),
        }
    }
    pub(crate) fn fill(
        &mut self,
        item_grp_id: EItemGrpId,
        item_cat_id: EItemCatId,
        item_attrs: &StMap<EAttrId, AttrVal>,
        item_effects: &StMap<EEffectId, AItemEffectData>,
        item_srqs: &StMap<EItemId, SkillLevel>,
        effects: &StMap<EEffectId, &AEffect>,
        fitted_limited_groups: &[EItemGrpId],
        online_limited_groups: &[EItemGrpId],
        active_limited_groups: &[EItemGrpId],
    ) {
        self.kind = get_item_kind(item_grp_id, item_cat_id, item_attrs, item_effects);
        self.volume = get_item_volume(item_attrs);
        self.ship_limit = get_item_ship_limit(item_attrs);
        self.charge_limit = get_item_charge_limit(item_attrs);
        self.val_fitted_group_id = match fitted_limited_groups.contains(&item_grp_id) {
            true => Some(item_grp_id),
            false => None,
        };
        self.val_online_group_id = match online_limited_groups.contains(&item_grp_id) {
            true => Some(item_grp_id),
            false => None,
        };
        self.val_active_group_id = match active_limited_groups.contains(&item_grp_id) {
            true => Some(item_grp_id),
            false => None,
        };
        self.implant_slot = get_implant_slot(item_attrs);
        self.booster_slot = get_booster_slot(item_attrs);
        self.subsystem_slot = get_subsystem_slot(item_attrs);
        self.is_light_fighter = get_light_fighter_flag(item_attrs);
        self.is_heavy_fighter = get_heavy_fighter_flag(item_attrs);
        self.is_support_fighter = get_support_fighter_flag(item_attrs);
        self.is_standup_light_fighter = get_standup_light_fighter_flag(item_attrs);
        self.is_standup_heavy_fighter = get_standup_heavy_fighter_flag(item_attrs);
        self.is_standup_support_fighter = get_standup_support_fighter_flag(item_attrs);
        self.ship_kind = get_ship_kind(item_cat_id, item_srqs);
        self.item_ship_kind = get_item_ship_kind(item_cat_id, item_attrs);
        self.max_state = get_max_state(item_effects.keys(), effects);
        self.drone_limit = get_ship_drone_limit(item_attrs);
        self.max_fighter_count = get_max_fighter_count(item_attrs);
    }
}
