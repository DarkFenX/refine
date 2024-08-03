use crate::{
    defs::{EEffectId, EItemId, SolFitId, SolItemId},
    sol::{item::SolBooster, item_info::SolSideEffectInfo},
    util::StMap,
};

pub struct SolBoosterInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub a_item_id: EItemId,
    pub enabled: bool,
    pub side_effects: StMap<EEffectId, SolSideEffectInfo>,
}
impl SolBoosterInfo {
    fn new(
        id: SolItemId,
        fit_id: SolFitId,
        a_item_id: EItemId,
        enabled: bool,
        side_effects: StMap<EEffectId, SolSideEffectInfo>,
    ) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            enabled,
            side_effects,
        }
    }
    pub(in crate::sol) fn from_booster_and_side_effects(
        sol_booster: &SolBooster,
        side_effects: StMap<EEffectId, SolSideEffectInfo>,
    ) -> Self {
        SolBoosterInfo::new(
            sol_booster.base.id,
            sol_booster.fit_id,
            sol_booster.base.a_item_id,
            sol_booster.get_bool_state(),
            side_effects,
        )
    }
}
