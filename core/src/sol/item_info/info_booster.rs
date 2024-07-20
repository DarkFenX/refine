use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolBooster,
};

pub struct SolBoosterInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub a_item_id: EItemId,
    pub enabled: bool,
}
impl SolBoosterInfo {
    fn new(id: SolItemId, fit_id: SolFitId, a_item_id: EItemId, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            enabled,
        }
    }
}
impl From<&SolBooster> for SolBoosterInfo {
    fn from(sol_booster: &SolBooster) -> Self {
        SolBoosterInfo::new(
            sol_booster.base.id,
            sol_booster.fit_id,
            sol_booster.base.a_item_id,
            sol_booster.get_bool_state(),
        )
    }
}
