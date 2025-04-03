use crate::{
    sol::{EffectId, FitId, ItemId, ItemTypeId, SlotIndex, info::SideEffectInfo, uad::item::Booster},
    util::StMap,
};

pub struct BoosterInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub slot: Option<SlotIndex>,
    pub enabled: bool,
    pub side_effects: StMap<EffectId, SideEffectInfo>,
}
impl BoosterInfo {
    pub(in crate::sol) fn from_booster_and_side_effects(
        sol_booster: &Booster,
        side_effects: StMap<EffectId, SideEffectInfo>,
    ) -> Self {
        Self {
            id: sol_booster.get_item_id(),
            type_id: sol_booster.get_a_item_id(),
            fit_id: sol_booster.get_fit_id(),
            slot: sol_booster.get_a_slot(),
            enabled: sol_booster.get_booster_state(),
            side_effects,
        }
    }
}
