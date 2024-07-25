use crate::{
    defs::{Amount, EEffectId, EItemId, SolFitId, SolItemId},
    sol::{
        item::{SolFighter, SolItemState},
        item_info::SolAutoChargeInfo,
    },
    util::StMap,
};

pub struct SolFighterInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub a_item_id: EItemId,
    pub state: SolItemState,
    pub amt_override: Option<Amount>,
    pub autocharges: StMap<EEffectId, SolAutoChargeInfo>,
}
impl SolFighterInfo {
    fn new(
        id: SolItemId,
        fit_id: SolFitId,
        a_item_id: EItemId,
        state: SolItemState,
        amt_override: Option<Amount>,
        autocharges: StMap<EEffectId, SolAutoChargeInfo>,
    ) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            state,
            amt_override,
            autocharges,
        }
    }
    pub(in crate::sol) fn from_fighter_and_autocharges(
        sol_fighter: &SolFighter,
        autocharges: StMap<EEffectId, SolAutoChargeInfo>,
    ) -> Self {
        SolFighterInfo::new(
            sol_fighter.base.id,
            sol_fighter.fit_id,
            sol_fighter.base.a_item_id,
            sol_fighter.state,
            sol_fighter.amt_override,
            autocharges,
        )
    }
}
