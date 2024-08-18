use crate::{
    defs::{EEffectId, EItemId, SolFitId, SolItemId},
    sol::{item::SolBooster, item_info::SolSideEffectInfo},
    util::StMap,
};

pub struct SolBoosterInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub fit_id: SolFitId,
    pub enabled: bool,
    pub side_effects: StMap<EEffectId, SolSideEffectInfo>,
}
impl SolBoosterInfo {
    fn new(
        id: SolItemId,
        type_id: EItemId,
        fit_id: SolFitId,
        enabled: bool,
        side_effects: StMap<EEffectId, SolSideEffectInfo>,
    ) -> Self {
        Self {
            id,
            fit_id,
            type_id,
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
            sol_booster.get_bool_state(),
            side_effects,
        )
    }
}
