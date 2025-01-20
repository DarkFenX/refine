use crate::{
    ad::{AItemEffectData, AItemKind, AItemShipLimit},
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
    /// Defines which ships this item can be fit to. If set, item is limited either via type or
    /// group.
    pub ship_limit: Option<AItemShipLimit>,
}
impl AItemExtras {
    pub(crate) fn new_with_data(
        grp_id: EItemGrpId,
        cat_id: EItemCatId,
        attrs: &StMap<EAttrId, AttrVal>,
        effects: &StMap<EEffectId, AItemEffectData>,
    ) -> Self {
        let mut extras = Self {
            kind: None,
            volume: None,
            ship_limit: None,
        };
        extras.update(grp_id, cat_id, attrs, effects);
        extras
    }
    pub(crate) fn update(
        &mut self,
        grp_id: EItemGrpId,
        cat_id: EItemCatId,
        attrs: &StMap<EAttrId, AttrVal>,
        effects: &StMap<EEffectId, AItemEffectData>,
    ) {
        self.kind = get_item_kind(grp_id, cat_id, attrs, effects);
        self.volume = get_item_volume(attrs);
        self.ship_limit = get_item_ship_limit(attrs);
    }
}
