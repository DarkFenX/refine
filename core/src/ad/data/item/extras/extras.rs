use crate::{
    ad::{AItem, AItemChargeLimit, AItemEffectData, AItemKind, AItemShipLimit},
    defs::{AttrVal, EAttrId, EEffectId, EItemCatId, EItemGrpId, EItemId, SkillLevel, SlotIndex},
    util::StMap,
};

use super::{
    charge_limit::get_item_charge_limit,
    fighter_kind::{
        get_heavy_fighter_flag, get_light_fighter_flag, get_standup_heavy_fighter_flag, get_standup_light_fighter_flag,
        get_standup_support_fighter_flag, get_support_fighter_flag,
    },
    kind::get_item_kind,
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
    /// If set, item can be fit to a ship which has a type or group match with the limit.
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
        }
    }
    // Build new instance, rebuilding all the data based on new attributes, copying data which does
    // not rely on them
    pub(crate) fn inherit_with_attrs(a_item: &AItem, attrs: &StMap<EAttrId, AttrVal>) -> Self {
        Self {
            kind: get_item_kind(a_item.grp_id, a_item.cat_id, attrs, &a_item.effect_datas, &a_item.srqs),
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
        }
    }
    pub(crate) fn fill(
        &mut self,
        grp_id: EItemGrpId,
        cat_id: EItemCatId,
        attrs: &StMap<EAttrId, AttrVal>,
        effects: &StMap<EEffectId, AItemEffectData>,
        srqs: &StMap<EItemId, SkillLevel>,
        fitted_limited_groups: &[EItemGrpId],
        online_limited_groups: &[EItemGrpId],
        active_limited_groups: &[EItemGrpId],
    ) {
        self.kind = get_item_kind(grp_id, cat_id, attrs, effects, srqs);
        self.volume = get_item_volume(attrs);
        self.ship_limit = get_item_ship_limit(attrs);
        self.charge_limit = get_item_charge_limit(attrs);
        self.val_fitted_group_id = match fitted_limited_groups.contains(&grp_id) {
            true => Some(grp_id),
            false => None,
        };
        self.val_online_group_id = match online_limited_groups.contains(&grp_id) {
            true => Some(grp_id),
            false => None,
        };
        self.val_active_group_id = match active_limited_groups.contains(&grp_id) {
            true => Some(grp_id),
            false => None,
        };
        self.implant_slot = get_implant_slot(attrs);
        self.booster_slot = get_booster_slot(attrs);
        self.subsystem_slot = get_subsystem_slot(attrs);
        self.is_light_fighter = get_light_fighter_flag(attrs);
        self.is_heavy_fighter = get_heavy_fighter_flag(attrs);
        self.is_support_fighter = get_support_fighter_flag(attrs);
        self.is_standup_light_fighter = get_standup_light_fighter_flag(attrs);
        self.is_standup_heavy_fighter = get_standup_heavy_fighter_flag(attrs);
        self.is_standup_support_fighter = get_standup_support_fighter_flag(attrs);
    }
}
