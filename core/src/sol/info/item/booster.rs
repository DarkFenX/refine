use crate::{
    defs::{EEffectId, EItemId, SlotIndex, SolFitId, SolItemId},
    sol::{info::SolSideEffectInfo, uad::item::SolBooster},
    util::StMap,
};

pub struct SolBoosterInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub fit_id: SolFitId,
    pub slot: Option<SlotIndex>,
    pub enabled: bool,
    pub side_effects: StMap<EEffectId, SolSideEffectInfo>,
}
impl SolBoosterInfo {
    fn new(
        id: SolItemId,
        type_id: EItemId,
        fit_id: SolFitId,
        slot: Option<SlotIndex>,
        enabled: bool,
        side_effects: StMap<EEffectId, SolSideEffectInfo>,
    ) -> Self {
        Self {
            id,
            type_id,
            fit_id,
            slot,
            enabled,
            side_effects,
        }
    }
    pub(in crate::sol) fn from_booster_and_side_effects(
        sol_booster: &SolBooster,
        side_effects: StMap<EEffectId, SolSideEffectInfo>,
    ) -> Self {
        SolBoosterInfo::new(
            sol_booster.get_id(),
            sol_booster.get_type_id(),
            sol_booster.get_fit_id(),
            sol_booster.get_slot(),
            sol_booster.get_bool_state(),
            side_effects,
        )
    }
}
