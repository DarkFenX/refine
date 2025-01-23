use crate::{
    ad::{AItem, AItemEffectData, AItemKind, AItemShipLimit},
    defs::{AttrVal, EAttrId, EEffectId, EItemCatId, EItemGrpId},
    util::StMap,
};

use super::{kind::get_item_kind, ship_limit::get_item_ship_limit, volume::get_item_volume};

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
    /// Item effectively has this group ID for purposes of "max group fitted" validation.
    pub val_fitted_group_id: Option<EItemGrpId>,
    /// Item effectively has this group ID for purposes of "max group online" validation.
    pub val_online_group_id: Option<EItemGrpId>,
    /// Item effectively has this group ID for purposes of "max group active" validation.
    pub val_active_group_id: Option<EItemGrpId>,
}
impl AItemExtras {
    pub(crate) fn new() -> Self {
        Self {
            kind: None,
            volume: None,
            ship_limit: None,
            val_fitted_group_id: None,
            val_online_group_id: None,
            val_active_group_id: None,
        }
    }
    // Build new instance, rebuilding all the data based on new attributes, copying data which does
    // not rely on them
    pub(crate) fn inherit_with_attrs(a_item: &AItem, attrs: &StMap<EAttrId, AttrVal>) -> Self {
        Self {
            kind: get_item_kind(a_item.grp_id, a_item.cat_id, attrs, &a_item.effect_datas),
            volume: get_item_volume(attrs),
            ship_limit: get_item_ship_limit(attrs),
            val_fitted_group_id: a_item.extras.val_fitted_group_id,
            val_online_group_id: a_item.extras.val_online_group_id,
            val_active_group_id: a_item.extras.val_active_group_id,
        }
    }
    pub(crate) fn fill(
        &mut self,
        grp_id: EItemGrpId,
        cat_id: EItemCatId,
        attrs: &StMap<EAttrId, AttrVal>,
        effects: &StMap<EEffectId, AItemEffectData>,
        fitted_limited_groups: &[EItemGrpId],
        online_limited_groups: &[EItemGrpId],
        active_limited_groups: &[EItemGrpId],
    ) {
        self.kind = get_item_kind(grp_id, cat_id, attrs, effects);
        self.volume = get_item_volume(attrs);
        self.ship_limit = get_item_ship_limit(attrs);
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
    }
}
