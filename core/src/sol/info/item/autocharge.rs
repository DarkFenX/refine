use crate::sol::{
    EffectId, FitId, ItemId, ItemTypeId,
    uad::{Uad, item::UadAutocharge},
};

pub struct AutochargeInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub cont_item_id: ItemId,
    pub cont_effect_id: EffectId,
    pub enabled: bool,
    // No projections because they fully match to projections of parent item
}
impl AutochargeInfo {
    pub(in crate::sol) fn from_autocharge(uad: &Uad, autocharge: &UadAutocharge) -> Self {
        Self {
            id: autocharge.get_item_id(),
            type_id: autocharge.get_a_item_id(),
            fit_id: uad.fits.id_by_key(autocharge.get_fit_key()),
            cont_item_id: uad.items.id_by_key(autocharge.get_cont_item_key()),
            cont_effect_id: autocharge.get_cont_effect_id().into(),
            enabled: !autocharge.get_force_disable(),
        }
    }
}
