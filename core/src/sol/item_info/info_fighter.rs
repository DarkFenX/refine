use crate::{
    defs::{Amount, EItemId, SolFitId, SolItemId},
    sol::item::{SolFighter, SolItemState},
};

pub struct SolFighterInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub a_item_id: EItemId,
    pub state: SolItemState,
    pub amt_override: Option<Amount>,
}
impl SolFighterInfo {
    fn new(
        id: SolItemId,
        fit_id: SolFitId,
        a_item_id: EItemId,
        state: SolItemState,
        amt_override: Option<Amount>,
    ) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            state,
            amt_override,
        }
    }
}
impl From<&SolFighter> for SolFighterInfo {
    fn from(sol_fighter: &SolFighter) -> Self {
        SolFighterInfo::new(
            sol_fighter.base.id,
            sol_fighter.fit_id,
            sol_fighter.base.a_item_id,
            sol_fighter.state,
            sol_fighter.amt_override,
        )
    }
}
